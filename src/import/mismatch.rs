//! Why a workflow text is not imported (V17). RED: signatures only.

use std::fmt;

use super::normalise::Line;

/// A refused import: where the text and the closest render part.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Mismatch {
    /// The 1-based text line.
    pub(crate) line: usize,
    /// Dotted path of the YAML key that differs.
    pub(crate) key: String,
    /// The kind of the closest render.
    pub(crate) kind: Option<String>,
    /// The render's line there.
    pub(crate) expected: Option<String>,
    /// The text's line there.
    pub(crate) found: Option<String>,
}

impl Mismatch {
    /// Where `text` parts from `render` of `kind`.
    pub(crate) fn at(_text: &[Line], _render: &[Line], _kind: &str) -> Self {
        Self::default()
    }

    /// No candidate rendered.
    pub(crate) fn unnamed(_text: &[Line]) -> Self {
        Self::default()
    }
}

/// How many leading kept lines `a` and `b` share.
pub(crate) fn common(_a: &[Line], _b: &[Line]) -> usize {
    0
}

/// The dotted key path of kept line `at`.
pub(crate) fn path(_lines: &[Line], _at: usize) -> String {
    String::new()
}

impl fmt::Display for Mismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("")
    }
}

impl std::error::Error for Mismatch {}

#[cfg(test)]
mod tests;
