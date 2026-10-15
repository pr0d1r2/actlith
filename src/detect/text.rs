//! Proposed rows as `.actlith` text: the `format 1` header and the
//! `## workflows` table (`.:I` file).

use crate::registry::WorkflowRow;

/// Everything above the first row; rows start on [`FIRST_ROW`].
pub(crate) const HEADER: &str =
    "format 1\n\n## workflows\nid|kind|systems|cache|args\n";

/// The 1-based line the first row lands on under [`HEADER`].
pub(crate) const FIRST_ROW: usize = 5;

/// A whole `.actlith`: [`HEADER`], then one line per row.
#[must_use]
pub(crate) fn text(rows: &[WorkflowRow]) -> String {
    let _ = rows;
    String::new()
}

/// One `id|kind|systems|cache|args` row; `-` for an empty cell, `\|` for
/// a pipe.
#[must_use]
pub(crate) fn row_line(row: &WorkflowRow) -> String {
    let _ = row;
    String::new()
}

#[cfg(test)]
mod tests;
