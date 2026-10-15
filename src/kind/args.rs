//! A kind's arg schema and the row's `args` checked against it
//! (`src/registry` V10): an unknown, repeated or missing key is an error
//! naming the row's line, never ignored or silently defaulted.

use std::collections::BTreeMap;

use super::error::KindError;
use crate::registry::WorkflowRow;

/// One arg a kind takes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ArgSpec {
    pub(crate) key: &'static str,
    /// The row must give it.
    pub(crate) required: bool,
    /// The value when the row leaves it out.
    pub(crate) default: Option<&'static str>,
}

/// Every arg a kind takes, in the order its docs list them.
pub(crate) type ArgSchema = [ArgSpec];

/// `timeout` (job minutes), which every kind takes (`docs/KINDS.md`).
pub(crate) const TIMEOUT: ArgSpec = ArgSpec {
    key: "timeout",
    required: false,
    default: Some("30"),
};

/// A row's args after the schema check: every given key is known, every
/// required key is there, every default is filled in.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Args {
    line: usize,
    values: BTreeMap<&'static str, String>,
}

impl Args {
    /// The `.actlith` line of the row the args came from.
    pub(crate) fn line(&self) -> usize {
        self.line
    }

    /// The value of `key`, given or defaulted.
    pub(crate) fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(String::as_str)
    }

    /// The value of `key` as a whole number; an error when it is not one.
    pub(crate) fn number(&self, key: &str) -> Result<Option<u32>, KindError> {
        Err(KindError::row(self.line, key, "stub"))
    }
}

/// The row's args checked against `schema`.
pub(crate) fn parse_args(
    schema: &ArgSchema,
    row: &WorkflowRow,
) -> Result<Args, KindError> {
    let _ = schema;
    Err(KindError::row(row.line, "stub", "stub"))
}

#[cfg(test)]
mod tests;
