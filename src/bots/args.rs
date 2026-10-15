//! A bot row's `args` cell against the bot's own schema (V38).

use super::error::BotError;
use crate::registry::BotRow;

/// Every key of `row` is one of `known`.
///
/// # Errors
///
/// The first key the bot does not take, naming the row's line.
pub(crate) fn known(row: &BotRow, known: &[&str]) -> Result<(), BotError> {
    let _ = (row, known);
    Err(BotError::default())
}

/// The value of `key`, which the bot needs.
///
/// # Errors
///
/// The row leaves `key` out; `hint` is the value to write.
pub(crate) fn required<'a>(
    row: &'a BotRow,
    key: &str,
    hint: &str,
) -> Result<&'a str, BotError> {
    let _ = (row, key, hint);
    Err(BotError::default())
}

#[cfg(test)]
mod tests;
