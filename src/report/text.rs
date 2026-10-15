//! The report as indented plain text, for a terminal.

use super::model::Report;

/// `report` as text: `workflows`, `hand-written`, `bots`.
pub(crate) fn text(_report: &Report) -> String {
    String::new()
}

#[cfg(test)]
mod tests;
