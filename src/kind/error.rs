//! Why a row could not become a workflow: a row error naming its line, or
//! a refused pin (`src/registry` V10, `src/pin` V2). The caller exits 2.

use std::fmt;

use crate::core::kind as neutral;
use crate::pin::PinError;
use crate::registry::RegistryError;

/// A refused row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum KindError {
    /// The row asks for something the kind does not take: `.actlith:<line>`,
    /// what is wrong, and the fix.
    Row(RegistryError),
    /// An action the kind uses has no usable pin.
    Pin(PinError),
}

impl KindError {
    /// A row error at `line`.
    pub(crate) fn row(
        line: usize,
        what: impl Into<String>,
        fix: impl Into<String>,
    ) -> Self {
        Self::Row(RegistryError::new(line, what, fix))
    }
}

impl From<PinError> for KindError {
    fn from(error: PinError) -> Self {
        Self::Pin(error)
    }
}

/// pipelith's row refusal, in actlith's `.actlith:<line>` shape.
impl From<neutral::KindError> for KindError {
    fn from(error: neutral::KindError) -> Self {
        Self::row(error.line, error.what, error.fix)
    }
}

impl fmt::Display for KindError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Row(error) => error.fmt(f),
            Self::Pin(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for KindError {}

#[cfg(test)]
mod tests;
