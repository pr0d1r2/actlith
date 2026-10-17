//! Proposed rows as `.actlith` text: the `format 1` header and the
//! `## workflows` table (`.:I` file).

use crate::registry::WorkflowRow;

/// Everything above the first row; rows start on [`FIRST_ROW`].
pub(crate) const HEADER: &str =
    "format 1\n\n## workflows\nid|kind|systems|cache|args\n";

/// The 1-based line the first row lands on under [`HEADER`].
pub(crate) const FIRST_ROW: usize = 5;

/// A whole `.actlith`: [`HEADER`], then each row's [`proposal`].
#[must_use]
pub(crate) fn text(rows: &[WorkflowRow]) -> String {
    rows.iter().fold(String::from(HEADER), |mut out, row| {
        out.push_str(&proposal(row));
        out.push('\n');
        out
    })
}

/// The line above a row proposed off by default (V50).
pub(crate) const OFF_NOTE: &str =
    "# off by default: uncomment to get daily nixpkgs-lock bump PRs";

/// Kinds proposed commented out: they push branches and open PRs on a
/// schedule, so a repository opts in by uncommenting (V50).
const OFF_BY_DEFAULT: [&str; 1] = ["update-pins"];

/// A proposed row as `detect` and `seed` write it: the row, or for a kind
/// off by default, [`OFF_NOTE`] and the row commented out.
#[must_use]
pub(crate) fn proposal(row: &WorkflowRow) -> String {
    let line = row_line(row);
    if OFF_BY_DEFAULT.contains(&row.kind.as_str()) {
        return format!("{OFF_NOTE}\n# {line}");
    }
    line
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
