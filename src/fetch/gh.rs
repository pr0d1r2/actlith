//! The real resolver: `gh api` as a subprocess (R14): no new crate, the
//! user's `gh auth` token reused.

use std::ffi::OsString;
use std::process::Command;

use super::error::FetchError;
use super::resolver::Resolver;

/// Repository variables a subprocess must not inherit.
pub(crate) const GIT_REPO_VARS: [&str; 6] = [
    "GIT_DIR",
    "GIT_WORK_TREE",
    "GIT_INDEX_FILE",
    "GIT_OBJECT_DIRECTORY",
    "GIT_ALTERNATE_OBJECT_DIRECTORIES",
    "GIT_COMMON_DIR",
];

/// `gh api` runner: `program prefix... api PATH --jq EXPR`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Gh {
    /// The program, `gh` unless a test swaps it.
    program: OsString,
    /// Arguments before `api` (a test's stand-in script).
    prefix: Vec<OsString>,
}

impl Gh {
    /// `gh` from `PATH`.
    pub(crate) fn new() -> Self {
        Self::with("gh", Vec::new())
    }

    /// Another program in place of `gh`, with leading arguments.
    pub(crate) fn with(
        program: impl Into<OsString>,
        prefix: Vec<OsString>,
    ) -> Self {
        let program = program.into();
        Self { program, prefix }
    }

    /// The command for `gh api <path> --jq <jq>`, repository vars dropped.
    pub(crate) fn command(&self, path: &str, jq: &str) -> Command {
        let _ = (path, jq);
        Command::new(&self.program)
    }
}

/// `owner/repo` of an action `owner/repo[/path]`.
///
/// # Errors
///
/// [`FetchError::Action`] when it is not that shape.
#[expect(clippy::unnecessary_wraps, reason = "RED stub")]
pub(crate) fn repo(action: &str) -> Result<String, FetchError> {
    Ok(action.to_owned())
}

impl Resolver for Gh {
    fn latest(&self, action: &str) -> Result<Option<String>, FetchError> {
        let _ = action;
        Ok(None)
    }

    fn sha(&self, action: &str, tag: &str) -> Result<String, FetchError> {
        let _ = tag;
        Ok(action.to_owned())
    }
}

#[cfg(test)]
mod tests;
