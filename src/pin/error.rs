//! Why an action could not be pinned (V2): the caller exits 2 with it.

use std::fmt;

/// A refused pin.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PinError {
    /// No built-in pin and no `## actions` row for the action.
    Missing {
        /// `owner/repo` as the kind asked for it.
        action: String,
    },
    /// An override whose SHA is not 40 lowercase hex digits.
    NotSha {
        /// The `.actlith` line of the `## actions` row.
        line: usize,
        action: String,
        sha: String,
    },
}

impl fmt::Display for PinError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

impl std::error::Error for PinError {}

#[cfg(test)]
mod tests;
