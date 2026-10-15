//! `pin [--dry-run]` (RED stub).

use std::path::Path;

use super::exit::{Outcome, Ran};
use crate::fetch::Resolver;

/// `pin` (RED stub).
///
/// # Errors
///
/// None yet.
pub(crate) fn pin(_root: &Path, _dry_run: bool, _: &dyn Resolver) -> Ran {
    Ok(Outcome::default())
}

#[cfg(test)]
mod tests;
