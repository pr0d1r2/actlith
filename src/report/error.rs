//! Why a `.actlith` could not be reported: a row naming no v1 kind or
//! bot, or a row its kind refused (`src/registry:V10`). The caller exits
//! 2.

use std::fmt;

use crate::kind::KindError;
use crate::registry::RegistryError;

/// A refused report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ReportError {
    /// A row naming no v1 kind or bot: its line, the fix.
    Row(RegistryError),
    /// A row its kind refused, or an action with no usable pin.
    Kind(KindError),
}

impl From<KindError> for ReportError {
    fn from(error: KindError) -> Self {
        Self::Kind(error)
    }
}

impl fmt::Display for ReportError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

impl std::error::Error for ReportError {}

#[cfg(test)]
mod tests;
