//! Totality (V7, `src/bots:V38`): pure, over a listing already read.

use std::collections::BTreeSet;
use std::path::Path;

use super::finding::{Finding, Problem};
use super::listing::{BOTS, WORKFLOWS};
// `src/render`'s `File` is this one; its re-export there stays unused
// (and `expect`ed so) until `src/gen` lands, so name it at the source.
use crate::bots::File;
use crate::registry::HandWritten;

/// Every finding for `listing` against the `rendered` files and the
/// `hand_written` rows, sorted by path.
pub(crate) fn totality(
    listing: &[String],
    rendered: &[File],
    hand_written: &[HandWritten],
) -> Vec<Finding> {
    let made: BTreeSet<&str> = rendered.iter().map(|f| &*f.path).collect();
    let listed: BTreeSet<&str> = listing.iter().map(|p| &**p).collect();
    let hand: BTreeSet<&str> = hand_written.iter().map(|h| &*h.path).collect();
    let mut found: Vec<Finding> = listing
        .iter()
        .filter_map(|path| on_disk(path, &made, &hand))
        .collect();
    for row in hand_written {
        found.extend(rows(row, &made, &listed));
    }
    found.sort();
    found
}

/// The finding, if any, for one listed `path`.
fn on_disk(
    path: &str,
    made: &BTreeSet<&str>,
    hand: &BTreeSet<&str>,
) -> Option<Finding> {
    if made.contains(path) {
        return None;
    }
    let problem = if BOTS.contains(&path) {
        Problem::BotWithoutRow
    } else if is_workflow(path) && !hand.contains(path) {
        Problem::Undeclared
    } else {
        return None;
    };
    Some(finding(path, None, problem))
}

/// The findings for one `## hand-written` row.
fn rows(
    row: &HandWritten,
    made: &BTreeSet<&str>,
    listed: &BTreeSet<&str>,
) -> Vec<Finding> {
    let path = row.path.as_str();
    let checks = [
        (row.reason.trim().is_empty(), Problem::NoReason),
        (!listed.contains(path), Problem::Missing),
        (made.contains(path), Problem::AlsoRendered),
    ];
    checks
        .into_iter()
        .filter(|(hit, _)| *hit)
        .map(|(_, problem)| finding(path, Some(row.line), problem))
        .collect()
}

/// A workflow file GitHub runs: `.yml` or `.yaml` directly under the
/// directory, the extension in any case.
fn is_workflow(path: &str) -> bool {
    let Some(name) = path.strip_prefix(WORKFLOWS) else {
        return false;
    };
    let ext = Path::new(name).extension().and_then(|e| e.to_str());
    !name.contains('/')
        && ext.is_some_and(|e| {
            e.eq_ignore_ascii_case("yml") || e.eq_ignore_ascii_case("yaml")
        })
}

/// One finding on `path`.
fn finding(path: &str, line: Option<usize>, problem: Problem) -> Finding {
    Finding {
        path: path.to_owned(),
        line,
        problem,
    }
}

#[cfg(test)]
mod tests;
