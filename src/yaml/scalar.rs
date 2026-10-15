//! Scalar styles: the one place that decides how a string is written
//! (V5). RED: signatures only.

use super::text::Text;

/// The longest line the writer aims for (`.yamllint.yaml`, `src/yaml` C).
pub(crate) const WIDTH: usize = 120;

/// `text` on one line, plain or quoted.
pub(crate) fn inline(_text: &Text, _flow: bool) -> String {
    String::new()
}

/// A key or other literal string on one line, plain or quoted.
pub(crate) fn key(_raw: &str) -> String {
    String::new()
}

/// `raw` quoted.
pub(crate) fn quote(_raw: &str) -> String {
    String::new()
}

/// `raw` as a `|` literal block: header and lines.
pub(crate) fn literal(_raw: &str) -> Option<(&'static str, Vec<&str>)> {
    None
}

/// `raw` as the lines of a `>-` folded block indented by `indent`.
pub(crate) fn fold(_raw: &str, _indent: usize) -> Option<Vec<String>> {
    None
}

#[cfg(test)]
mod tests;
