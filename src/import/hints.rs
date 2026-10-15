//! The values a workflow text shows for a row's cells (T26). RED:
//! signatures only.

use super::normalise::Line;

/// The top-level `name:`.
pub(crate) fn id(_lines: &[Line]) -> Option<String> {
    None
}

/// Every cachix cache the text substitutes from.
pub(crate) fn caches(_lines: &[Line]) -> Vec<String> {
    Vec::new()
}

/// Every value the text shows for arg `key`.
pub(crate) fn arg_values(_key: &str, _lines: &[Line]) -> Vec<String> {
    Vec::new()
}

#[cfg(test)]
mod tests;
