//! Why `actlith pin` stopped (V16): the caller exits 2 with it.

use std::fmt;
use std::path::{Path, PathBuf};

use crate::pin::PinError;
use crate::registry::RegistryError;

/// A refused or failed `pin`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum FetchError {
    /// `.actlith` could not be read.
    Read {
        /// The file.
        path: PathBuf,
        /// The I/O error.
        why: String,
    },
    /// `.actlith` could not be written.
    Write {
        /// The file.
        path: PathBuf,
        /// The I/O error.
        why: String,
    },
    /// `.actlith` does not parse.
    Registry(RegistryError),
    /// An `## actions` row is refused by the pin table.
    Pin(PinError),
    /// An action name that is not `owner/repo[/path]`.
    Action {
        /// The action as written.
        action: String,
    },
    /// A tag unsafe to put in an API path.
    Tag {
        /// The action the tag belongs to.
        action: String,
        /// The tag as resolved or recorded.
        tag: String,
    },
    /// The resolver answered something that is not a full commit SHA.
    Sha {
        /// The action.
        action: String,
        /// The tag it was resolved from.
        tag: String,
        /// What came back.
        got: String,
    },
    /// The GitHub API call failed.
    Api {
        /// The action being resolved.
        action: String,
        /// What went wrong.
        why: String,
    },
}

impl FetchError {
    /// What failed, naming the file, action, tag or answer.
    fn what(&self) -> String {
        match self {
            Self::Read { path, why } => io("read", path, why),
            Self::Write { path, why } => io("write", path, why),
            Self::Registry(e) => e.to_string(),
            Self::Pin(e) => e.to_string(),
            Self::Action { action } => format!("`{action}` is not owner/repo"),
            Self::Tag { action, tag } => {
                format!("`{action}` tag `{tag}` unsafe")
            }
            Self::Sha { action, tag, got } => not_sha(action, tag, got),
            Self::Api { action, why } => {
                format!("`{action}` unresolved: {why}")
            }
        }
    }

    /// The fix line; registry and pin errors carry their own.
    fn fix(&self) -> Option<&'static str> {
        match self {
            Self::Registry(_) | Self::Pin(_) => None,
            Self::Read { .. } => {
                Some("run in the repository root or pass --root")
            }
            Self::Write { .. } => Some("make `.actlith` writable"),
            Self::Action { .. } => Some("write it as `owner/repo[/path]`"),
            Self::Tag { .. } => {
                Some("record a tag of letters, digits, `.-_+/`")
            }
            Self::Sha { .. } => Some("check that the tag exists on GitHub"),
            Self::Api { .. } => Some("install `gh` and run `gh auth login`"),
        }
    }
}

/// The resolver's answer for `action@tag` is not a commit SHA.
fn not_sha(action: &str, tag: &str, got: &str) -> String {
    format!("`{action}@{tag}` gave `{got}`, not a commit SHA")
}

/// `cannot <verb> <path>: <why>`.
fn io(verb: &str, path: &Path, why: &str) -> String {
    format!("cannot {verb} {}: {why}", path.display())
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.fix() {
            Some(fix) => write!(f, "{}\n  fix: {fix}", self.what()),
            None => f.write_str(&self.what()),
        }
    }
}

impl std::error::Error for FetchError {}

#[cfg(test)]
mod tests;
