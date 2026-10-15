//! One table row into cells: `|` splits, `\|` is a literal pipe, `-` is
//! an empty cell, `,` splits a list (registry C, `.:I` file).

use super::error::RegistryError;
use super::model::Arg;
use super::table::Table;

/// The cells of a row, trimmed, `\|` unescaped, `-` made empty.
#[must_use]
pub(crate) fn split(row: &str) -> Vec<String> {
    let _ = row;
    Vec::new()
}

/// Exactly the section's cell count, or the row is refused.
///
/// # Errors
///
/// The row has more or fewer cells than the section's header.
pub(crate) fn fixed<const N: usize>(
    line: usize,
    table: Table,
    cells: Vec<String>,
) -> Result<[String; N], RegistryError> {
    let _ = (table, cells);
    Err(RegistryError::new(line, "", ""))
}

/// The header row is the section's own, cell by cell.
///
/// # Errors
///
/// Any other header.
pub(crate) fn header(
    line: usize,
    table: Table,
    row: &str,
) -> Result<(), RegistryError> {
    let _ = (table, row);
    Err(RegistryError::new(line, "", ""))
}

/// A cell that may not be empty.
///
/// # Errors
///
/// The cell is empty (or `-`).
pub(crate) fn required(
    line: usize,
    column: &str,
    value: String,
) -> Result<String, RegistryError> {
    let _ = (column, value);
    Err(RegistryError::new(line, "", ""))
}

/// A cell that may be empty: `None` then.
#[must_use]
pub(crate) fn optional(value: &str) -> Option<String> {
    let _ = value;
    None
}

/// A comma-separated list; an empty cell is an empty list.
///
/// # Errors
///
/// An empty item (`a,,b`).
pub(crate) fn list(
    line: usize,
    column: &str,
    cell: &str,
) -> Result<Vec<String>, RegistryError> {
    let _ = (column, cell);
    Err(RegistryError::new(line, "", ""))
}

/// Comma-separated `key=value` pairs, each key once.
///
/// # Errors
///
/// An item that is not `key=value`, or a key given twice.
pub(crate) fn args(line: usize, cell: &str) -> Result<Vec<Arg>, RegistryError> {
    let _ = cell;
    Err(RegistryError::new(line, "", ""))
}

#[cfg(test)]
mod tests;
