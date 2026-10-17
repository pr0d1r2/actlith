//! `.actlith` text to a [`Registry`] (registry T13): pipelith reads the
//! grammar against actlith's four sections, then each row is typed. Every
//! error names its line (V10, V11).

use super::error::RegistryError;
use super::model::Registry;
use super::rows::{action, bot, hand};
use crate::core::kind::{self, RawRow};
use crate::core::registry::{self, Schema, SectionSpec};

/// The file name every error and every message names.
pub(crate) const FILE: &str = ".actlith";

/// A section keyed by its first column, as every actlith section is.
const fn keyed(
    name: &'static str,
    columns: &'static [&'static str],
) -> SectionSpec<'static> {
    SectionSpec {
        name,
        columns,
        required: false,
        key: Some(0),
    }
}

/// actlith's sections: pipelith's neutral `## workflows`, then GitHub's.
const SCHEMA: Schema<'static> = Schema {
    sections: &[
        kind::WORKFLOWS,
        keyed("actions", &["action", "version", "sha"]),
        keyed("hand-written", &["path", "reason"]),
        keyed("bots", &["bot", "args"]),
    ],
};

/// Parse a whole `.actlith`.
///
/// # Errors
///
/// The first problem found, with its line, what is wrong and the fix.
pub(crate) fn parse(text: &str) -> Result<Registry, RegistryError> {
    let doc = registry::parse(FILE, text, &SCHEMA)?;
    let rows = |name| doc.section(name).iter();
    Ok(Registry {
        workflows: rows(kind::SECTION)
            .map(|r| RawRow::from_cells(FILE, r))
            .collect::<Result<_, _>>()?,
        actions: rows("actions").map(action).collect::<Result<_, _>>()?,
        hand_written: rows("hand-written")
            .map(hand)
            .collect::<Result<_, _>>()?,
        bots: rows("bots").map(bot).collect::<Result<_, _>>()?,
    })
}

#[cfg(test)]
mod tests;
