//! `.actlith` text to a [`Registry`] (registry T13): pipelith reads the
//! grammar against actlith's four sections, then each row is typed. Every
//! error names its line (V10, V11).

use super::error::RegistryError;
use super::model::Registry;
use super::rows::{action, bot, cache, hand};
use crate::core::kind::{self, RawRow};
use crate::core::registry::{self, Schema, SectionSpec};
use crate::core::runner::{self, Runners};

/// The file name every error and every message names.
pub(crate) const FILE: &str = ".actlith";

/// actlith's sections: pipelith's neutral `## workflows`, then GitHub's,
/// then pipelith's neutral `## runners`, each keyed by its first column.
const SCHEMA: Schema<'static> = Schema {
    sections: &[
        kind::WORKFLOWS,
        SectionSpec {
            name: "actions",
            columns: &["action", "version", "sha"],
            required: false,
            key: Some(0),
        },
        SectionSpec {
            name: "caches",
            columns: &["name", "public-key"],
            required: false,
            key: Some(0),
        },
        SectionSpec {
            name: "hand-written",
            columns: &["path", "reason"],
            required: false,
            key: Some(0),
        },
        SectionSpec {
            name: "bots",
            columns: &["bot", "args"],
            required: false,
            key: Some(0),
        },
        runner::RUNNERS,
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
        caches: rows("caches").map(cache).collect::<Result<_, _>>()?,
        hand_written: rows("hand-written")
            .map(hand)
            .collect::<Result<_, _>>()?,
        bots: rows("bots").map(bot).collect::<Result<_, _>>()?,
        runners: Runners::from_document(FILE, &doc)?,
    })
}

#[cfg(test)]
mod tests;
