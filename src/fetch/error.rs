//! Why `actlith pin` stopped (V16): the caller exits 2 with it.

use std::fmt;
use std::path::PathBuf;

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

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("")
    }
}

impl std::error::Error for FetchError {}

#[cfg(test)]
mod tests;
