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
    rows.iter().fold(String::from(HEADER), |mut out, row| {
        out.push_str(&row_line(row));
        out.push('\n');
        out
    })
}

/// One `id|kind|systems|cache|args` row; `-` for an empty cell, `\|` for
/// a pipe.
#[must_use]
pub(crate) fn row_line(row: &WorkflowRow) -> String {
    let systems = list(row.systems.iter().map(String::as_str));
    let cache = list(row.cache.as_deref());
    let args = row.args.iter().map(|a| format!("{}={}", a.key, a.value));
    let cells = [cell(&row.id), cell(&row.kind), systems, cache, list(args)];
    cells.join("|")
}

/// Items joined by `, `, each escaped; `-` when there are none.
fn list<T: AsRef<str>>(items: impl IntoIterator<Item = T>) -> String {
    let cells: Vec<String> =
        items.into_iter().map(|item| cell(item.as_ref())).collect();
    if cells.is_empty() {
        return "-".to_owned();
    }
    cells.join(", ")
}

/// A literal pipe written as `\|`.
fn cell(text: &str) -> String {
    text.replace('|', "\\|")
}

#[cfg(test)]
mod tests;
