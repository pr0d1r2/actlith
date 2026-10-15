//! `report` (T28): the `.actlith` reported in one [`Format`]. Pure: no
//! I/O (`.:V1`).

use super::error::ReportError;
use super::format::Format;
use crate::pin::PinTable;
use crate::registry::{Registry, RegistryError};

/// `registry` reported in `format`, its actions pinned from `pins`.
///
/// # Errors
///
/// The first row naming no v1 kind or bot, or one its kind refuses.
pub(crate) fn report(
    _registry: &Registry,
    _pins: &PinTable,
    _format: Format,
) -> Result<String, ReportError> {
    Err(ReportError::Row(RegistryError::default()))
}

#[cfg(test)]
mod tests;
