//! `.actlith` under `--root`: read, parsed and rendered for the verbs
//! that need it (RED stub).

use std::path::Path;

use super::exit::Outcome;
use crate::registry::Registry;
use crate::render::File;

/// The parsed `<root>/.actlith` (RED stub: empty).
///
/// # Errors
///
/// None yet.
pub(crate) fn registry(_root: &Path) -> Result<Registry, Outcome> {
    Ok(Registry::default())
}

/// The parsed `<root>/.actlith` and its files (RED stub: none).
///
/// # Errors
///
/// None yet.
pub(crate) fn rendered(_root: &Path) -> Result<(Registry, Vec<File>), Outcome> {
    Ok((Registry::default(), Vec::new()))
}

#[cfg(test)]
mod tests;
