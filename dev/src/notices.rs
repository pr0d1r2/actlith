//! `actlith-dev notices [--check]`: the crate table in
//! `docs/THIRD-PARTY-NOTICES.md`, from what `Cargo.lock` resolves (T11).
//! RED: signatures only.

use crate::io::{Doc, Failed, update};
use std::path::Path;

const DOC: Doc = Doc {
    path: "docs/THIRD-PARTY-NOTICES.md",
    verb: "notices",
};

/// A crate the published crate ships: name, version, licence.
pub(crate) type Crate = (String, String, String);

/// The crates in `cargo tree` output, without the crate `own` itself.
#[must_use]
pub(crate) fn crates(_tree: &str, _own: &str) -> Vec<Crate> {
    Vec::new()
}

/// The table block, markers included.
#[must_use]
pub(crate) fn table(_crates: &[Crate]) -> String {
    String::new()
}

/// What the prose claims that the table does not bear out.
#[must_use]
pub(crate) fn stale_prose(
    _doc: &str,
    _direct: usize,
    _crates: &[Crate],
) -> Vec<String> {
    Vec::new()
}

/// Render or check.
///
/// # Errors
///
/// Stale prose or drift (1), cargo or a file failing (2).
pub(crate) fn run_verb(repo: &Path, check: bool) -> Result<(), Failed> {
    update(repo, &DOC, "", check)
}

#[cfg(test)]
mod tests;
