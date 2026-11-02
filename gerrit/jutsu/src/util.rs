use std::process::Command;

use crate::config::Ctx;
use gerrit_events::PatchSetRef;

pub fn git_fetch<'a>(ctx: &'static Ctx, refs: &'a [&'a str]) {
    if refs.is_empty() {
        return;
    }
    Command::new("git")
        .arg(format!("--git-dir={}", ctx.git_dir.display()))
        .args(["fetch", "origin"])
        .args(refs)
        .status()
        .unwrap();
}

pub fn fetch_all(ctx: &'static Ctx) {
    Command::new("git")
        .arg(format!("--git-dir={}", ctx.git_dir.display()))
        .args(["fetch", "origin"])
        .status()
        .unwrap();
}

pub fn jj_rebase_onto_main(ctx: &'static Ctx) {
    for jj_dir in ctx.jj_locks.iter() {
        let guard = jj_dir.lock().unwrap();
        Command::new("jj")
            .args([
                "rebase",
                "-r",
                "mutable() & ~(::@ | @::)",
                "-d",
                "main@origin",
            ])
            .current_dir(&*guard)
            .status()
            .ok();
    }
}

pub fn jj_sync(ctx: &'static Ctx) {
    for jj_dir in ctx.jj_locks.iter() {
        let guard = jj_dir.lock().unwrap();
        Command::new("jj")
            .args(["git", "import"])
            .current_dir(&*guard)
            .status()
            .unwrap();
        Command::new("jj")
            .args(["config", "set", "--repo", "templates.commit_trailers"])
            .arg(
                r#"'''
     if(
       !trailers.contains_key("Change-Id"),
       format_gerrit_change_id_trailer(self)
     )
     '''"#,
            )
            .current_dir(&*guard)
            .output()
            .unwrap();
        Command::new("jj")
            .args(["config", "set", "--repo", "gerrit.default-remote", "origin"])
            .current_dir(&*guard)
            .output()
            .unwrap();
        Command::new("jj")
            .args([
                "config",
                "set",
                "--repo",
                "gerrit.default-remote-branch",
                "main",
            ])
            .current_dir(&*guard)
            .output()
            .unwrap();
    }
}

pub fn jj_abandon(ctx: &'static Ctx, rf: &str) {
    for jj_dir in ctx.jj_locks.iter() {
        let guard = jj_dir.lock().unwrap();
        let _ = Command::new("jj")
            .args(["abandon", rf])
            .current_dir(&*guard)
            .output();
    }
}

pub fn pre_fetch(ctx: &'static Ctx, commit_id: &str, patchset_ref: PatchSetRef) {
    for jj_dir in ctx.jj_locks.iter() {
        let guard = jj_dir.lock().unwrap();
        let x = Command::new("jj")
            .args([
                "log",
                "-r",
                commit_id,
                "--no-graph",
                "-T",
                "change_id.short()",
            ])
            .current_dir(&*guard)
            .output()
            .unwrap();

        let change_id = String::from_utf8(x.stdout).unwrap();

        if x.status.success() {
            Command::new("jj")
                .args([
                    "metaedit",
                    "--update-author-timestamp",
                    "-r",
                    format!("{}/0", change_id).as_str(),
                ])
                .current_dir(&*guard)
                .output()
                .unwrap();
            Command::new("jj")
                .args([
                    "bookmark",
                    "set",
                    format!("gerrit/{}", patchset_ref.change_num).as_str(),
                    "-r",
                    &change_id,
                    "--allow-backwards",
                ])
                .current_dir(&*guard)
                .output()
                .unwrap();
        }
    }
}
