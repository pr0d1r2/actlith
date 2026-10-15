//! The `## actions` section of `.actlith` text: its rows, and the text
//! with only that section replaced (V16).

use crate::pin::Uses;

/// The `## actions` header row (`.:I` file).
pub(crate) const HEADER: &str = "action|version|sha";

/// The header row and one `action|version|sha` row per pin, each line
/// ending in a newline.
pub(crate) fn rows(pins: &[Uses]) -> String {
    let _ = pins;
    String::new()
}

/// `text` with the rows of `## actions` replaced by `rows`; comments in
/// the section and every other line kept. No section -> appended.
pub(crate) fn replace(text: &str, rows: &str) -> String {
    let _ = rows;
    text.to_owned()
}

#[cfg(test)]
mod tests;
