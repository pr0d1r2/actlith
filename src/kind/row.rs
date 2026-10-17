//! A `## workflows` row checked for its kind by pipelith's
//! `Row::check` (systems, cache, args, timeout and row rules;
//! `src/registry` V10), after actlith names any system without a
//! GitHub-hosted runner in its own words. A kind builds only from a
//! checked row.

use super::contract::Kind;
use super::error::KindError;
use crate::core::kind::{Extras, Row};
use crate::registry::WorkflowRow;
use crate::system::{parse_system, runner};

/// The kinds and arg keys only actlith knows: none today, so every row
/// actlith renders renders on every forge (V49).
pub(crate) const EXTRAS: Extras<'static> = Extras {
    kinds: &[],
    keys: &[],
};

/// `row` checked against `kind`.
pub(crate) fn check(
    kind: &dyn Kind,
    row: &WorkflowRow,
) -> Result<Row, KindError> {
    hosted(row)?;
    Ok(Row::check(kind.spec(), row, &EXTRAS)?)
}

/// Each listed system up to the first repeat (pipelith names that one)
/// is a nix system with a GitHub-hosted runner.
fn hosted(row: &WorkflowRow) -> Result<(), KindError> {
    let fix = "write systems with a hosted runner, or `-`";
    let refuse = |e: crate::system::SystemError| {
        KindError::row(row.line, e.to_string(), fix)
    };
    for (n, name) in row.systems.iter().enumerate() {
        if row.systems.iter().take(n).any(|seen| seen == name) {
            return Ok(());
        }
        runner(parse_system(name).map_err(refuse)?).map_err(refuse)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests;
