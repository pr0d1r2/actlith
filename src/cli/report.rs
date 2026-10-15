//! `report [--format text|json|md]` (RED stub).

use std::path::Path;

use super::exit::{Outcome, Ran};
use crate::registry::Registry;
use crate::report::Format;

/// `report` (RED stub).
///
/// # Errors
///
/// None yet.
pub(crate) fn report(_root: &Path, _format: Format) -> Ran {
    Ok(Outcome::default())
}

/// `registry` reported in `format` (RED stub).
fn of(_registry: &Registry, _format: Format) -> Ran {
    Ok(Outcome::default())
}

#[cfg(test)]
mod tests;
