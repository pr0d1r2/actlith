//! A row error: where, what, and how to fix it (registry G, I).

use std::fmt;

/// Why a `.actlith` was refused. The caller exits 2 with it (`.:I` exit).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct RegistryError {
    /// The 1-based line at fault.
    pub(crate) line: usize,
    /// What is wrong, naming the offending text.
    pub(crate) what: String,
    /// What to write instead.
    pub(crate) fix: String,
}

impl RegistryError {
    /// An error at `line`.
    #[must_use]
    pub(crate) fn new(
        line: usize,
        what: impl Into<String>,
        fix: impl Into<String>,
    ) -> Self {
        Self {
            line,
            what: what.into(),
            fix: fix.into(),
        }
    }
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            ".actlith:{}: {}\n  fix: {}",
            self.line, self.what, self.fix
        )
    }
}

impl std::error::Error for RegistryError {}

#[cfg(test)]
mod tests;
