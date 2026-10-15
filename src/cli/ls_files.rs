//! The tracked file list `detect` and `seed --init` read (RED stub).

use std::path::Path;
use std::process::Command;

use super::exit::Outcome;

/// Repository variables a `git` run on another root must not inherit.
pub(crate) const GIT_REPO_VARS: [&str; 6] = [
    "GIT_DIR",
    "GIT_WORK_TREE",
    "GIT_INDEX_FILE",
    "GIT_OBJECT_DIRECTORY",
    "GIT_ALTERNATE_OBJECT_DIRECTORIES",
    "GIT_COMMON_DIR",
];

/// `git ls-files -z` in `root` (RED stub: bare `git`).
pub(crate) fn command(_root: &Path) -> Command {
    Command::new("git")
}

/// The tracked paths under `root` (RED stub: none).
///
/// # Errors
///
/// None yet.
pub(crate) fn tracked(_root: &Path) -> Result<Vec<String>, Outcome> {
    Ok(Vec::new())
}

#[cfg(test)]
mod tests;
