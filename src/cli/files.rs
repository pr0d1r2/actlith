//! `gen [--check]` and `check` (RED stub).

use std::path::Path;

use super::exit::{Outcome, Ran};

/// `gen` / `gen --check` (RED stub).
///
/// # Errors
///
/// None yet.
pub(crate) fn generate(_root: &Path, _check: bool) -> Ran {
    Ok(Outcome::default())
}

/// `check` (RED stub).
///
/// # Errors
///
/// None yet.
pub(crate) fn check(_root: &Path) -> Ran {
    Ok(Outcome::default())
}

#[cfg(test)]
mod tests;
