//! The tracked file list `detect` and `seed --init` read: `git ls-files`
//! as a subprocess (`.:C` runtime deps: `git`).

use std::io;
use std::path::Path;
use std::process::Command;

use super::exit::{Outcome, USAGE};

/// Repository variables a `git` run on another root must not inherit
/// (a hook sets them for the repository it runs in). The same list as
/// `src/fetch`'s, which keeps its own private.
pub(crate) const GIT_REPO_VARS: [&str; 6] = [
    "GIT_DIR",
    "GIT_WORK_TREE",
    "GIT_INDEX_FILE",
    "GIT_OBJECT_DIRECTORY",
    "GIT_ALTERNATE_OBJECT_DIRECTORIES",
    "GIT_COMMON_DIR",
];

/// `git ls-files -z` in `root`. The repository variables are dropped only
/// when `root` is not `.`: in `.` they name the repository meant.
pub(crate) fn command(root: &Path) -> Command {
    let mut command = Command::new("git");
    command.args(["ls-files", "-z"]).current_dir(root);
    if root != Path::new(".") {
        for var in GIT_REPO_VARS {
            command.env_remove(var);
        }
    }
    command
}

/// The tracked paths under `root`, as git lists them.
///
/// # Errors
///
/// Exit 2: `git` cannot run, or `root` is not in a git work tree.
pub(crate) fn tracked(root: &Path) -> Result<Vec<String>, Outcome> {
    let out = command(root).output().map_err(|e| spawn(root, &e))?;
    if !out.status.success() {
        return Err(failed(root, &String::from_utf8_lossy(&out.stderr)));
    }
    let text = String::from_utf8_lossy(&out.stdout);
    let paths = text.split('\0').filter(|p| !p.is_empty());
    Ok(paths.map(str::to_owned).collect())
}

/// Exit 2: `git ls-files` in `root` said `why`.
fn failed(root: &Path, why: &str) -> Outcome {
    Outcome::fail(
        USAGE,
        format!(
            "{}: `git ls-files` failed: {}\n  fix: run in a git work tree, \
             or pass --root DIR naming one",
            root.display(),
            why.trim()
        ),
    )
}

/// Exit 2: `git` could not be started in `root`.
fn spawn(root: &Path, error: &io::Error) -> Outcome {
    Outcome::fail(
        USAGE,
        format!(
            "cannot run `git` in {}: {error}\n  fix: install git, or pass \
             --root DIR naming an existing directory",
            root.display()
        ),
    )
}

#[cfg(test)]
mod tests;
