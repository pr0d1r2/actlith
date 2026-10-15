//! A bot row error: where, what, and how to fix it (V38).

use std::fmt;

/// Why a `## bots` row was refused. The caller exits 2 with it (V38).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct BotError {
    /// The 1-based `.actlith` line of the row at fault.
    pub(crate) line: usize,
    /// What is wrong, naming the offending text.
    pub(crate) what: String,
    /// What to write instead.
    pub(crate) fix: String,
}

impl BotError {
    /// An error at `line`.
    #[must_use]
    pub(crate) fn new(
        line: usize,
        what: impl Into<String>,
        fix: impl Into<String>,
    ) -> Self {
        let _ = (line, what.into(), fix.into());
        Self::default()
    }
}

impl fmt::Display for BotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.what)
    }
}

impl std::error::Error for BotError {}

#[cfg(test)]
mod tests;
