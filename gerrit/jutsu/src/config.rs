use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use std::sync::Mutex;

use git_url_parse::GitUrl;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BaseConfig {
    #[serde(default = "project_root")]
    pub project_root: PathBuf,

    #[serde(default = "git_dir")]
    pub git_dir: PathBuf,

    #[serde(default)]
    pub jj_dirs: Vec<PathBuf>,

    #[serde(default = "default_events")]
    pub events: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Ctx {
    #[serde(default = "git_dir")]
    pub git_dir: PathBuf,

    #[serde(default)]
    pub jj_locks: Vec<std::sync::Mutex<PathBuf>>,

    #[serde(default = "parse_server")]
    pub gerrit_server: String,

    #[serde(default = "parse_repo")]
    pub repo: String,

    #[serde(default = "default_events")]
    pub events: Vec<String>,
}

fn parse_server() -> String {
    let out = Command::new("git")
        .args(["remote", "get-url", "origin"])
        .output()
        .unwrap();
    let url = GitUrl::parse(String::from_utf8(out.stdout).unwrap().as_str()).unwrap();
    url.host().unwrap().to_string()
}

fn parse_repo() -> String {
    let out = Command::new("git")
        .args(["remote", "get-url", "origin"])
        .output()
        .unwrap();
    let url = GitUrl::parse(String::from_utf8(out.stdout).unwrap().as_str()).unwrap();
    url.path().to_string()
}

fn project_root() -> PathBuf {
    std::env::current_dir().expect("failed to get current working directory")
}

fn git_dir() -> PathBuf {
    PathBuf::from_str(".git").unwrap()
}

fn default_events() -> Vec<String> {
    vec![
        "change-abandoned".to_string(),
        "change-deleted".to_string(),
        "change-merged".to_string(),
        "change-restored".to_string(),
        "dropped-output".to_string(),
        "comment-added".to_string(),
        "patchset-created".to_string(),
        "ref-updated".to_string(),
        "batch-ref-updated".to_string(),
        "reviewer-added".to_string(),
        "reviewer-deleted".to_string(),
        "topic-changed".to_string(),
        "wip-state-changed".to_string(),
        "private-state-changed".to_string(),
        "vote-deleted".to_string(),
    ]
}

impl Ctx {
    pub fn load() -> Self {
        let config_path = std::env::args()
            .nth(1)
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("gerrit-jutsu.toml"));

        let config_text = std::fs::read_to_string(&config_path).unwrap_or_default();

        let base_cfg: BaseConfig = toml::from_str(config_text.as_str()).unwrap();

        let git_dir = base_cfg.project_root.join(base_cfg.git_dir);

        let (gerrit_server, repo) = {
            let out = Command::new("git")
                .args([
                    "-C",
                    git_dir.to_str().unwrap(),
                    "remote",
                    "get-url",
                    "origin",
                ])
                .output()
                .unwrap();
            let url = GitUrl::parse(String::from_utf8(out.stdout).unwrap().as_str()).unwrap();
            (url.host().unwrap().to_string(), url.path().to_string())
        };
        let jj_locks = base_cfg.jj_dirs.into_iter().map(Mutex::new).collect();

        let cfg: Ctx = Ctx {
            git_dir,
            gerrit_server,
            repo,
            events: base_cfg.events,
            jj_locks,
        };
        println!("{:?}", cfg);

        cfg
    }
}
