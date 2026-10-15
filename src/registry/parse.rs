//! The whole file: `format` line, then `## <section>` tables, `#`
//! comments and blank lines skipped (registry C, V11, V12).

use super::error::RegistryError;
use super::model::Registry;

/// Parse `.actlith` text. Pure: the caller reads the file (registry C).
///
/// # Errors
///
/// The first problem found, with its line and a fix: a missing or newer
/// `format`, an unknown or repeated section, a bad header, a malformed
/// row, or a key given twice.
pub(crate) fn parse(text: &str) -> Result<Registry, RegistryError> {
    let _ = text;
    Err(RegistryError::default())
}

#[cfg(test)]
mod tests;
