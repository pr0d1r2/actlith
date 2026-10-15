//! `.actlith` rows into the sorted [`Report`] (T28, C): each workflow row
//! checked and built by its v1 kind.

use super::error::ReportError;
use super::model::Report;
use crate::pin::PinTable;
use crate::registry::{Registry, RegistryError};

/// The report of `registry`, its actions pinned from `pins`.
///
/// # Errors
///
/// The first row naming no v1 kind or bot, or one its kind refuses.
pub(crate) fn collect(
    _registry: &Registry,
    _pins: &PinTable,
) -> Result<Report, ReportError> {
    Err(ReportError::Row(RegistryError::default()))
}

#[cfg(test)]
mod tests;
