//! `## bots` rows into files, one per row, in row order (V38, `.:V1`).
//! The registry already refused a bot named twice; the order of the
//! files on disk is `src/render`'s to set.

use super::error::BotError;
use super::model::File;
use crate::registry::BotRow;

/// The v1 bots, as a fix hint names them.
pub(crate) const BOTS: &str = "`coderabbit`, `greptile`, `socket`";

/// The config file of each row.
///
/// # Errors
///
/// The first row with an unknown bot, or an arg its bot refuses, naming
/// the row's line (V38: the caller exits 2).
pub(crate) fn bot_files(rows: &[BotRow]) -> Result<Vec<File>, BotError> {
    let _ = (rows, BOTS);
    Err(BotError::default())
}

#[cfg(test)]
mod tests;
