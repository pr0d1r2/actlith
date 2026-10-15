//! `socket.yml`: Socket's config, schema version 2 (R11). Every field but
//! `version` is optional, so the file is the header and `version: 2`.

use super::error::BotError;
use super::model::File;
use crate::registry::BotRow;

/// Where Socket reads its config.
pub(crate) const PATH: &str = "socket.yml";

/// The `socket` row's file. The bot takes no args.
///
/// # Errors
///
/// Any arg.
pub(crate) fn file(row: &BotRow) -> Result<File, BotError> {
    let _ = row;
    Err(BotError::default())
}

#[cfg(test)]
mod tests;
