//! GitHub's sections, typed from the cells pipelith read (registry T13):
//! `## actions`, `## hand-written`, `## bots`.

use super::error::RegistryError;
use super::model::{ActionPin, BotRow, Cache, HandWritten};
use super::parse::FILE;
use crate::core::registry::{Row, args, required};

#[cfg(test)]
mod tests;

/// An `action|version|sha` row; the SHA is a full commit SHA.
pub(super) fn action(row: &Row) -> Result<ActionPin, RegistryError> {
    Ok(ActionPin {
        line: row.line,
        action: required(FILE, row, 0, "action")?,
        version: required(FILE, row, 1, "version")?,
        sha: full_sha(row.line, required(FILE, row, 2, "sha")?)?,
    })
}

pub(super) fn cache(row: &Row) -> Result<Cache, RegistryError> {
    Ok(Cache {
        line: row.line,
        name: required(FILE, row, 0, "name")?,
        public_key: required(FILE, row, 1, "public-key")?,
    })
}

/// A `path|reason` row; both are required.
pub(super) fn hand(row: &Row) -> Result<HandWritten, RegistryError> {
    Ok(HandWritten {
        line: row.line,
        path: required(FILE, row, 0, "path")?,
        reason: required(FILE, row, 1, "reason")?,
    })
}

/// A `bot|args` row.
pub(super) fn bot(row: &Row) -> Result<BotRow, RegistryError> {
    Ok(BotRow {
        line: row.line,
        bot: required(FILE, row, 0, "bot")?,
        args: args(FILE, row, 1, "args")?,
    })
}

/// `sha` when it is 40 lowercase hex digits.
fn full_sha(line: usize, sha: String) -> Result<String, RegistryError> {
    let hex = |c: char| c.is_ascii_digit() || ('a'..='f').contains(&c);
    if sha.len() == 40 && sha.chars().all(hex) {
        return Ok(sha);
    }
    let what = format!("sha `{sha}` is not a full commit SHA");
    let fix = "write the 40-hex commit SHA (`actlith pin` resolves it)";
    Err(RegistryError::new(line, what, fix))
}
