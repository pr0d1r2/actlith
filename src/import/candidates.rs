//! Candidate `## workflows` rows for a workflow text (T26). RED:
//! signature only.

use super::normalise::Line;

/// Every candidate row for `lines`, as `.actlith` row text.
pub(crate) fn candidates(_lines: &[Line]) -> Vec<String> {
    Vec::new()
}

#[cfg(test)]
mod tests;
