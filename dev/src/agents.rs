//! `actlith-dev agents [--check]`: the step table in AGENTS.md, from the
//! steps hk.pkl gives each hook (T11). RED: signatures only.

use crate::io::{Failed, read};
use std::path::Path;

/// The table block, markers included.
#[must_use]
pub(crate) fn table(_rows: &str, _commit: usize, _push: usize) -> String {
    String::new()
}

/// Render or check.
///
/// # Errors
///
/// Drift (1), `pkl` or a file failing (2).
pub(crate) fn run_verb(repo: &Path, _check: bool) -> Result<(), Failed> {
    read(repo, "AGENTS.md").map(|_| ())
}

#[cfg(test)]
mod tests;
