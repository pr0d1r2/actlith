//! `report` (T28): the `.actlith` reported in one [`Format`]. Pure: no
//! I/O (`.:V1`).

use super::collect::collect;
use super::document::document;
use super::error::ReportError;
use super::format::Format;
use super::json::write;
use super::md::md;
use super::text::text;
use crate::pin::PinTable;
use crate::registry::Registry;

/// `registry` reported in `format`, its actions pinned from `pins`.
///
/// # Errors
///
/// The first row naming no v1 kind or bot, or one its kind refuses.
pub(crate) fn report(
    registry: &Registry,
    pins: &PinTable,
    format: Format,
) -> Result<String, ReportError> {
    let report = collect(registry, pins)?;
    Ok(match format {
        Format::Text => text(&report),
        Format::Json => write(&document(&report)),
        Format::Md => md(&report),
    })
}

#[cfg(test)]
mod tests;
