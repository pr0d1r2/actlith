//! A workflow text normalised for comparison (V17). RED: signature only.

/// One kept line of a workflow text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Line {
    /// The 1-based line it starts on in the text.
    pub(crate) number: usize,
    /// The normalised text, indentation kept.
    pub(crate) text: String,
    /// Content of a literal block scalar: no key on it.
    pub(crate) block: bool,
}

/// `text` normalised, line by line.
pub(crate) fn normalise(_text: &str) -> Vec<Line> {
    Vec::new()
}

/// The column of the key on `raw`.
pub(crate) fn column(_raw: &str) -> usize {
    0
}

#[cfg(test)]
mod tests;
