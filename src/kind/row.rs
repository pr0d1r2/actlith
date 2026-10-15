//! A `## workflows` row checked for its kind: systems resolved to ones
//! with a runner, the cache to a known key, the args to the schema
//! (`src/registry` V10). A kind builds only from a checked row.

use super::args::Args;
use super::cache::Cache;
use super::contract::Kind;
use super::error::KindError;
use crate::registry::WorkflowRow;
use crate::system::System;

/// A row a kind can build from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Row {
    /// The workflow id: file stem and `name`.
    pub(crate) id: String,
    /// The `.actlith` line of the row.
    pub(crate) line: usize,
    /// The systems, each with a hosted runner; the kind's default for `-`.
    pub(crate) systems: Vec<System>,
    pub(crate) cache: Option<Cache>,
    pub(crate) args: Args,
}

impl Row {
    /// `row` checked against `kind`.
    pub(crate) fn parse(
        kind: &dyn Kind,
        row: &WorkflowRow,
    ) -> Result<Self, KindError> {
        let _ = (kind.schema(), kind.default_systems(), row);
        Err(KindError::row(0, "stub", "stub"))
    }
}

#[cfg(test)]
mod tests;
