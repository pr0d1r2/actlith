//! `detect`: proposed `## workflows` rows from tracked paths and
//! `flake.lock` text, in fixed order (detect C, `.:V1`).

use super::lock::has_node;
use super::text::FIRST_ROW;
use crate::registry::WorkflowRow;

/// Proposed rows: `flake.nix` tracked gives `ci|gate`; a `nixpkgs-lock` node
/// in `flake.lock` gives `update-pins|update-pins`, which `text` writes
/// commented out (V50). Always that order; `line` counts one per row.
#[must_use]
pub(crate) fn detect<S: AsRef<str>>(
    tracked: &[S],
    flake_lock: Option<&str>,
) -> Vec<WorkflowRow> {
    let gate = tracked.iter().any(|path| path.as_ref() == "flake.nix");
    let pins = flake_lock.is_some_and(|lock| has_node(lock, "nixpkgs-lock"));
    [(gate, "ci", "gate"), (pins, "update-pins", "update-pins")]
        .into_iter()
        .filter(|&(found, _, _)| found)
        .zip(FIRST_ROW..)
        .map(|((_, id, kind), line)| bare(line, id, kind))
        .collect()
}

/// A row with every optional cell empty: the kind's defaults.
fn bare(line: usize, id: &str, kind: &str) -> WorkflowRow {
    WorkflowRow {
        line,
        id: id.to_owned(),
        kind: kind.to_owned(),
        systems: Vec::new(),
        cache: None,
        args: Vec::new(),
    }
}

#[cfg(test)]
mod tests;
