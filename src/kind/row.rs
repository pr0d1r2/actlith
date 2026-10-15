//! A `## workflows` row checked for its kind: systems resolved to ones
//! with a runner, the cache to a known key, the args to the schema
//! (`src/registry` V10). A kind builds only from a checked row.

use super::args::{Args, parse_args};
use super::cache::Cache;
use super::contract::Kind;
use super::error::KindError;
use crate::registry::WorkflowRow;
use crate::system::{System, parse_system, runner};

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
        let cache = row.cache.as_deref();
        Ok(Self {
            id: row.id.clone(),
            line: row.line,
            systems: systems(kind.default_systems(), row)?,
            cache: cache.map(|c| Cache::parse(row.line, c)).transpose()?,
            args: parse_args(kind.schema(), row)?,
        })
    }
}

fn systems(
    defaults: &[System],
    row: &WorkflowRow,
) -> Result<Vec<System>, KindError> {
    if row.systems.is_empty() {
        return Ok(defaults.to_vec());
    }
    let mut out = Vec::new();
    for name in &row.systems {
        let system = hosted(row.line, name)?;
        if out.contains(&system) {
            return Err(twice(row.line, name));
        }
        out.push(system);
    }
    Ok(out)
}

fn hosted(line: usize, name: &str) -> Result<System, KindError> {
    let fix = "write systems with a hosted runner, or `-`";
    let system = parse_system(name)
        .map_err(|e| KindError::row(line, e.to_string(), fix))?;
    runner(system).map_err(|e| KindError::row(line, e.to_string(), fix))?;
    Ok(system)
}

fn twice(line: usize, name: &str) -> KindError {
    KindError::row(
        line,
        format!("system `{name}` is listed twice"),
        format!("keep one `{name}`"),
    )
}

#[cfg(test)]
mod tests;
