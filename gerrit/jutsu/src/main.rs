mod config;

use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

use crate::config::Ctx;
use gerrit_events::{Change, GerritEvent};
use threadpool::ThreadPool;

mod util;

fn main() {
    let ctx: &'static Ctx = Box::leak(Box::new(Ctx::load()));

    let mut cmd = Command::new("ssh");
    cmd.args([&ctx.gerrit_server, "gerrit", "stream-events"]);

    for event in &ctx.events {
        cmd.args(["-s", event]);
    }

    let mut child = cmd.stdout(Stdio::piped()).spawn().unwrap();

    // Subscribe first so events arriving during startup remain in the stream.
    full_sync(ctx);

    let pool = ThreadPool::new(4);

    let stdout = child.stdout.take().expect("Failed to capture stdout");

    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        let line = line.unwrap();
        pool.execute({
            move || match serde_json::from_str::<GerritEvent>(&line) {
                Ok(event) => handle_event(event, ctx),
                Err(_) => {
                    println!("warning: failed to deserialize event");
                    println!("{line}");
                    println!(" ");
                }
            }
        });
    }
    child.wait().unwrap();

    println!("gerrit stream-events channel closed.");
}

fn full_sync(ctx: &'static Ctx) {
    let out = Command::new("ssh")
        .args([
            &ctx.gerrit_server,
            "gerrit",
            "query",
            "--format=JSON",
            "--current-patch-set",
            "--no-limit",
            "status:open",
            "branch:main",
            format!("project:{}", ctx.repo).as_str(),
        ])
        .output()
        .unwrap();

    assert!(
        out.status.success(),
        "failed to query open changes: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let out = String::from_utf8(out.stdout).unwrap();
    let mut missing = Vec::new();
    for line in out.lines().filter(|line| !line.trim().is_empty()) {
        let change: Change = match serde_json::from_str(line) {
            Ok(change) => change,
            _ => continue,
        };
        let Some(patch_set) = change.current_patch_set else {
            eprintln!("no visible current patch set for change {}", change.number);
            continue;
        };
        let patch_ref = patch_set.patch_set_ref;

        let suffix: String = serde_plain::to_string(&patch_ref).unwrap();
        missing.push((suffix, patch_set.revision, patch_ref));
    }

    for (_, revision, patch_ref) in &missing {
        util::pre_fetch(ctx, revision, *patch_ref);
    }
    util::fetch_all(ctx);
    util::jj_sync(ctx)
}

fn handle_event<'a>(event: GerritEvent<'a>, ctx: &'static Ctx) {
    if let Some(project) = event.project()
        && project != ctx.repo.as_str()
    {
        return;
    }
    match event {
        GerritEvent::ChangeAbandoned { change, .. } => {
            if change.branch == "main" {
                util::jj_abandon(ctx, format!("gerrit/{}", change.number).as_str());
            }
        }
        GerritEvent::PatchsetCreated {
            patch_set, change, ..
        }
        | GerritEvent::ChangeRestored {
            patch_set, change, ..
        } => {
            if change.branch == "main" {
                util::pre_fetch(ctx, patch_set.revision, patch_set.patch_set_ref);
                util::git_fetch(
                    ctx,
                    &[&serde_plain::to_string(&patch_set.patch_set_ref).unwrap()],
                );
                util::jj_sync(ctx);
            }
        }
        GerritEvent::DroppedOutput => full_sync(ctx),
        GerritEvent::ChangeMerged { change, .. } if change.branch == "main" => {
            util::git_fetch(ctx, &["main"]);
            util::jj_sync(ctx);
            util::jj_abandon(ctx, format!("gerrit/{}", change.number).as_str());
            util::jj_rebase_onto_main(ctx);
        }
        _ => (),
    }
}
