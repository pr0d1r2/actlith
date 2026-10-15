//! The report as Markdown tables, for a PR or an issue.

use super::model::Report;

/// `report` as a Markdown page.
pub(crate) fn md(_report: &Report) -> String {
    String::new()
}

/// `raw` as a code span in a table cell.
pub(crate) fn code(_raw: &str) -> String {
    String::new()
}

#[cfg(test)]
mod tests;
