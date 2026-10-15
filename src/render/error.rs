//! Why a `.actlith` could not be rendered: a row or pin a kind refused,
//! an unknown kind, a refused bot row, or two files at one path (V31,
//! `src/registry:V10`). The caller exits 2.

use std::fmt;

use crate::bots::BotError;
use crate::kind::KindError;
use crate::pin::PinError;
use crate::registry::RegistryError;

/// A refused render.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RenderError {
    /// A `## workflows` row naming no v1 kind: its line, the fix.
    Row(RegistryError),
    /// A row its kind refused, or an action with no usable pin.
    Kind(KindError),
    /// An `## actions` row the pin table refused.
    Pin(PinError),
    /// A `## bots` row `src/bots` refused.
    Bot(BotError),
    /// Two outputs at one path.
    Duplicate(String),
}

impl From<KindError> for RenderError {
    fn from(error: KindError) -> Self {
        Self::Kind(error)
    }
}

impl From<PinError> for RenderError {
    fn from(error: PinError) -> Self {
        Self::Pin(error)
    }
}

impl From<BotError> for RenderError {
    fn from(error: BotError) -> Self {
        Self::Bot(error)
    }
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Row(error) => error.fmt(f),
            Self::Kind(error) => error.fmt(f),
            Self::Pin(error) => error.fmt(f),
            Self::Bot(error) => error.fmt(f),
            Self::Duplicate(path) => write!(
                f,
                "two files would be written at `{path}`\n  fix: give each \
                 `## workflows` row its own id"
            ),
        }
    }
}

impl std::error::Error for RenderError {}

#[cfg(test)]
mod tests;
