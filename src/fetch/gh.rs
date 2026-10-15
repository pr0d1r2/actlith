//! The real resolver: `gh api` as a subprocess (R14): no new crate, the
//! user's `gh auth` token reused. `commits/<tag>` dereferences annotated
//! tags in one call; `releases/latest` answers HTTP 404 when a repository
//! has no release.

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
        let mut command = Command::new(&self.program);
        command.args(&self.prefix).args(["api", path, "--jq", jq]);
        for var in GIT_REPO_VARS {
            command.env_remove(var);
        }
        command
    }

    /// The trimmed answer of `gh api <path>`; `None` on HTTP 404.
    fn api(
        &self,
        action: &str,
        path: &str,
        jq: &str,
    ) -> Result<Option<String>, FetchError> {
        let program = self.program.display();
        let out = self.command(path, jq).output().map_err(|e| {
            failed(action, format!("cannot run `{program}`: {e}"))
        })?;
        let text =
            |bytes: &[u8]| String::from_utf8_lossy(bytes).trim().to_owned();
        if out.status.success() {
            return Ok(Some(text(&out.stdout)));
        }
        let why = text(&out.stderr);
        if why.contains("HTTP 404") {
            return Ok(None);
        }
        Err(failed(action, why))
    }
}

/// An API failure while resolving `action`.
fn failed(action: &str, why: String) -> FetchError {
    let action = action.to_owned();
    FetchError::Api { action, why }
}

/// `owner/repo` of an action `owner/repo[/path]`.
///
/// # Errors
///
/// [`FetchError::Action`] when it is not that shape.
pub(crate) fn repo(action: &str) -> Result<String, FetchError> {
    let mut parts = action.split('/');
    match (parts.next(), parts.next()) {
        (Some(owner), Some(name)) if segment(owner) && segment(name) => {
            Ok(format!("{owner}/{name}"))
        }
        _ => Err(FetchError::Action {
            action: action.to_owned(),
        }),
    }
}

/// A GitHub owner or repository name.
fn segment(name: &str) -> bool {
    let ok = |c: char| c.is_ascii_alphanumeric() || "-_.".contains(c);
    !name.is_empty() && name != "." && name != ".." && name.chars().all(ok)
}

impl Resolver for Gh {
    fn latest(&self, action: &str) -> Result<Option<String>, FetchError> {
        let path = format!("repos/{}/releases/latest", repo(action)?);
        self.api(action, &path, ".tag_name")
    }

    fn sha(&self, action: &str, tag: &str) -> Result<String, FetchError> {
        let path = format!("repos/{}/commits/{tag}", repo(action)?);
        let found = self.api(action, &path, ".sha")?;
        found.ok_or_else(|| failed(action, format!("no tag `{tag}`")))
    }
}

#[cfg(test)]
mod tests;
