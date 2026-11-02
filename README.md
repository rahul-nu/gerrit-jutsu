# gerrit-jutsu

Synchronize Gerrit changes with local Git and Jujutsu repositories.

## Intended use and alpha status

As changes are uploaded, revised, merged, or abandoned in Gerrit, local jj
repositories need to catch up. Working across multiple independent jj
repositories means repeating fetches, imports, and local updates in each one.
gerrit-jutsu listens to Gerrit events and automates those updates by subscribing
to the gerrit event-stream, and updating local refs/tags/changes in response to
gerrit events.

gerrit-jutsu works well with **jjui** running permanently in a neighboring window.
On the happy path, local jj bookmarks should synchronize with gerrit changes
uploaded by us, and earlier patchsets should be visible as tags in the evolog.

1. This is hacky, alpha code. We are still figuring out the ideal workflow, what
   actions to take in response to Gerrit events, and what level of fine-tuning via
   to expose via config.

2. Whenever a change is merged into `main`, gerrit-jutsu automatically
   rebases mutable changes onto `main@origin` in every configured jj repository.
   The current rebase selection excludes the working-copy commit (`@`), its
   ancestors, and its descendants: `mutable() & ~(::@ | @::)`.

3. When a patch set appears on Gerrit, we tag the precise commit. This causes the
   jj change to be conflicted. This is expected, and there are currently no plans
   to change this behavior: multiple patch sets represent multiple versions of a
   change, so describing them as a conflict is semantically valid.

4. Earlier patch sets are visible as tags in a change's evolog, letting you
   inspect the versions previously uploaded to Gerrit alongside the change's
   local evolution.

## Workspace layout

The Cargo workspace contains:

- `gerrit/events`: the `gerrit_events` library, with Serde types for Gerrit events.
- `gerrit/jutsu`: the `gerrit_jutsu` binary, including its internal `config` module.

## Build

Install a current stable Rust toolchain, then run from the repository root:

```sh
cargo build --workspace --locked
cargo test --workspace --locked
```

## Run

The binary requires `git`, `ssh`, and `jj` on your PATH, plus SSH access to
Gerrit's `stream-events` and `query` commands. The configured Git repository's
`origin` remote supplies the Gerrit hostname and project path. Configure SSH
connection details (such as username and port) in your SSH config.

The backing Git repository must fetch Gerrit change refs as local tags. Add
this line to the existing `[remote "origin"]` section in `.git/config`, keeping
the existing URL and branch fetch entries:

```gitconfig
[remote "origin"]
    fetch = +refs/changes/*:refs/tags/changes/*
```

This maps Gerrit patch-set refs to `changes/*` tags that gerrit-jutsu relies on.
If you use a shared bare store such as `bare.git`, edit `bare.git/config`
instead. Configure this once in the backing Git store shared by your jj
repositories.

To help keep the long-running Gerrit event stream connected during idle periods,
add keepalives to the matching host entry in `~/.ssh/config` (replace
`gerrit.example.com` with the hostname or SSH alias used by your `origin` remote):

```sshconfig
Host gerrit.example.com
    ServerAliveInterval 30
    ServerAliveCountMax 3
```

SSH sends a keepalive after 30 seconds without receiving data and disconnects
after roughly 90 seconds if the server stops responding. See the
[OpenSSH keepalive options](https://man.openbsd.org/ssh_config#ServerAliveInterval).

Create a `gerrit-jutsu.toml`, for example:

```toml
project_root = "/path/to/project"
git_dir = ".git"
jj_dirs = ["/path/to/jj/workspace"]
```

`project_root` defaults to the current working directory, `git_dir` to `.git`,
and `jj_dirs` to an empty list. `git_dir` is resolved relative to `project_root`;
use absolute paths for `jj_dirs`. An optional `events` array overrides the
default Gerrit event subscriptions.

```sh
cargo run --locked -p gerrit_jutsu -- /path/to/gerrit-jutsu.toml
```

Without an argument, the binary reads `gerrit-jutsu.toml` in the current working
directory. It fetches Git refs and, for each configured Jujutsu workspace,
imports Git changes and updates repository configuration. When a change merges,
it also rebases eligible mutable commits onto `main@origin`. The current
implementation assumes the Gerrit branch is `main`.

## Multiple jj repositories sharing a Git store

One `gerrit_jutsu` process can keep several independent jj repositories in sync
with the same Gerrit project. Each jj repository must already use the same
backing Git store; listing a directory in the config does not create a jj
repository or change its backing store. Each repository keeps its own jj state
and working copy, while Git objects and refs are shared.

For example, suppose your repositories are arranged like this:

```text
/path/to/project/
├── bare.git/          # Shared Git store, with origin pointing to Gerrit
├── jj-2/             # Independent jj repository backed by bare.git
├── jj-3/             # Independent jj repository backed by bare.git
└── gerrit-jutsu.toml
```

Configure the shared store once and list every jj repository to synchronize:

```toml
project_root = "/path/to/project"
git_dir = "bare.git"
jj_dirs = ["/path/to/project/jj-2", "/path/to/project/jj-3"]

# Optional: subscribe only to the events needed for this setup.
events = [
  "change-abandoned",
  "change-merged",
  "change-restored",
  "dropped-output",
  "patchset-created",
]
```

`git_dir` is resolved relative to `project_root`, so this config uses
`/path/to/project/bare.git`. Relative entries such as `jj_dirs = ["jj-2", "jj-3"]`
resolve from the process's working directory, **not** from `project_root` or the
config file's directory. Absolute `jj_dirs` paths let you launch the binary
from any directory.

Start a single process with this config (from the source repository):

```sh
cargo run --locked -p gerrit_jutsu -- /path/to/project/gerrit-jutsu.toml
```

The process opens one Gerrit event stream before querying open changes on
`main`, fetches into the shared Git store, and runs `jj git import` in each
listed repository. For patch-set commits already present locally, it updates
metadata and sets `gerrit/<change-number>` bookmarks before fetching. New patch
sets and restored changes trigger a fetch and import; abandoned or merged
changes trigger `jj abandon` for the corresponding bookmark. Merged changes
also trigger a fetch of `main` and a rebase of eligible mutable commits onto
`main@origin`. A dropped-output event triggers another full synchronization.
The process also updates each repository's jj configuration.
Each repository can have its own working copy and local work while receiving
updates from the same server through this one process.

## AI usage

Envisioned and designed by human, written jointly by human and multiple clankers.

## License

See [LICENSE](LICENSE).
