//! Test-only temp repositories for the verb tests: a fresh directory
//! under the system temp dir (never inside this work tree), files put in
//! it, and real `git` run there with the repository variables dropped
//! and hooks off.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::ls_files::GIT_REPO_VARS;
use crate::fetch::{FetchError, Resolver};

/// A fresh, empty directory for one test.
pub(crate) fn scratch(name: &str) -> PathBuf {
    let id = std::process::id();
    let dir = std::env::temp_dir().join(format!("actlith-cli-{id}-{name}"));
    let _ = fs::remove_dir_all(&dir);
    assert!(fs::create_dir_all(&dir).is_ok());
    dir
}

/// Write `text` at `rel` under `root`, creating parents.
pub(crate) fn put(root: &Path, rel: &str, text: &str) {
    let path = root.join(rel);
    assert!(path.parent().is_some_and(|p| fs::create_dir_all(p).is_ok()));
    assert!(fs::write(path, text).is_ok());
}

/// Run `git args...` in `root`; true when it succeeded.
pub(crate) fn git(root: &Path, args: &[&str]) -> bool {
    let mut command = Command::new("git");
    command.args(["-c", "core.hooksPath=/dev/null"]).args(args);
    for var in GIT_REPO_VARS {
        command.env_remove(var);
    }
    let status = command.current_dir(root).output();
    status.is_ok_and(|out| out.status.success())
}

/// The SHA every tag resolves to under [`Fake`].
pub(crate) const SHA: &str = "0123456789abcdef0123456789abcdef01234567";

/// A resolver with no network: no action has a release (the recorded
/// tag stays) and every tag resolves to [`SHA`].
pub(crate) struct Fake;

impl Resolver for Fake {
    fn latest(&self, _action: &str) -> Result<Option<String>, FetchError> {
        Ok(None)
    }

    fn sha(&self, _action: &str, _tag: &str) -> Result<String, FetchError> {
        Ok(SHA.to_owned())
    }
}

#[cfg(test)]
mod tests;
