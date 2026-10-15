//! A data row into its section's typed row; one key per section given
//! once (registry T13).

use super::error::RegistryError;
use super::model::Registry;
use super::table::Table;

/// Type `row` (on `line`) as a row of `table` and append it to `reg`.
///
/// # Errors
///
/// A bad cell count, an empty required cell, a malformed list, arg, id
/// or SHA.
pub(crate) fn add(
    reg: &mut Registry,
    table: Table,
    line: usize,
    row: &str,
) -> Result<(), RegistryError> {
    let _ = (reg, table, row);
    Err(RegistryError::new(line, "", ""))
}

/// Each section's key (workflow id, action, path, bot) appears once.
///
/// # Errors
///
/// The second row with a key, naming the first one's line.
pub(crate) fn unique(reg: &Registry) -> Result<(), RegistryError> {
    let _ = reg;
    Err(RegistryError::default())
}

#[cfg(test)]
mod tests;
