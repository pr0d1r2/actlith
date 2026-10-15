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
        let Some(value) = self.get(key) else {
            return Ok(None);
        };
        value.parse().map(Some).map_err(|_| {
            KindError::row(
                self.line,
                format!("arg `{key}` is `{value}`, not a whole number"),
                format!("write `{key}=<number>`"),
            )
        })
    }
}

/// The row's args checked against `schema`.
pub(crate) fn parse_args(
    schema: &ArgSchema,
    row: &WorkflowRow,
) -> Result<Args, KindError> {
    let mut values = BTreeMap::new();
    for arg in &row.args {
        let spec = known(schema, row, &arg.key)?;
        if values.insert(spec.key, arg.value.clone()).is_some() {
            return Err(repeated(row.line, spec.key));
        }
    }
    for spec in schema {
        fill(&mut values, spec, row)?;
    }
    Ok(Args {
        line: row.line,
        values,
    })
}

fn known<'a>(
    schema: &'a ArgSchema,
    row: &WorkflowRow,
    key: &str,
) -> Result<&'a ArgSpec, KindError> {
    schema
        .iter()
        .find(|s| s.key == key)
        .ok_or_else(|| unknown(schema, row, key))
}

fn fill(
    values: &mut BTreeMap<&'static str, String>,
    spec: &ArgSpec,
    row: &WorkflowRow,
) -> Result<(), KindError> {
    if values.contains_key(spec.key) {
        return Ok(());
    }
    match spec.default {
        Some(value) => {
            values.insert(spec.key, value.to_owned());
            Ok(())
        }
        None if spec.required => Err(missing(row, spec.key)),
        None => Ok(()),
    }
}

fn unknown(schema: &ArgSchema, row: &WorkflowRow, key: &str) -> KindError {
    let what = format!("kind `{}` has no arg `{key}`", row.kind);
    if schema.is_empty() {
        return KindError::row(row.line, what, "write `-` in args");
    }
    let keys: Vec<&str> = schema.iter().map(|s| s.key).collect();
    let fix = format!("remove `{key}` or use one of: {}", keys.join(", "));
    KindError::row(row.line, what, fix)
}

fn repeated(line: usize, key: &str) -> KindError {
    KindError::row(
        line,
        format!("arg `{key}` is given twice"),
        format!("keep one `{key}=`"),
    )
}

fn missing(row: &WorkflowRow, key: &str) -> KindError {
    KindError::row(
        row.line,
        format!("kind `{}` needs arg `{key}`", row.kind),
        format!("add `{key}=<value>` to args"),
    )
}

#[cfg(test)]
mod tests;
