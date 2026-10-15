//! `actlith-dev readme [--check]`: the README badge block, and the numbers
//! `docs/LLM-DISCLAIMER.md` quotes, from the files that own them (T11).
//! RED: signatures only.

use crate::io::{Failed, hooks};
use std::path::Path;

/// Render or check.
///
/// # Errors
///
/// Stale disclaimer prose or drift (1), an unreadable source (2).
pub(crate) fn run_verb(repo: &Path, _check: bool) -> Result<(), Failed> {
    hooks(repo).map(|_| ())
}

#[cfg(test)]
mod tests;
