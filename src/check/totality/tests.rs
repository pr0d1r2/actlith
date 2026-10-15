//! `totality` (V7, `src/bots:V38`): each finding, and none when total.

use crate::check::{Finding, Problem, totality};
use crate::registry::HandWritten;
use crate::render::File;

fn paths(list: &[&str]) -> Vec<String> {
    list.iter().map(|p| (*p).to_owned()).collect()
}

fn rendered(list: &[&str]) -> Vec<File> {
    list.iter()
        .map(|p| File {
            path: (*p).to_owned(),
            bytes: "x\n".to_owned(),
        })
        .collect()
}

fn hand(line: usize, path: &str, reason: &str) -> HandWritten {
    HandWritten {
        line,
        path: path.to_owned(),
        reason: reason.to_owned(),
    }
}

fn found(path: &str, line: Option<usize>, problem: Problem) -> Finding {
    Finding {
        path: path.to_owned(),
        line,
        problem,
    }
}

#[test]
fn a_total_repository_has_no_findings() {
    let listing = paths(&[
        ".coderabbit.yaml",
        ".github/workflows/ci.yml",
        ".github/workflows/release.yml",
    ]);
    let files = rendered(&[
        ".coderabbit.yaml",
        ".github/workflows/ci.yml",
        "scripts/actlith/update-pins.sh",
    ]);
    let hw = [hand(9, ".github/workflows/release.yml", "release-please")];
    assert_eq!(totality(&listing, &files, &hw), []);
}

#[test]
fn an_undeclared_workflow_is_a_finding() {
    let listing = paths(&[
        ".github/workflows/ci.yml",
        ".github/workflows/odd.yaml",
        ".github/workflows/stray.yml",
    ]);
    let files = rendered(&[".github/workflows/ci.yml"]);
    let want = [
        found(".github/workflows/odd.yaml", None, Problem::Undeclared),
        found(".github/workflows/stray.yml", None, Problem::Undeclared),
    ];
    assert_eq!(totality(&listing, &files, &[]), want);
}

#[test]
fn a_non_workflow_file_is_ignored() {
    let listing = paths(&[
        ".github/workflows/README.md",
        ".github/workflows/deep/x.yml",
        ".github/workflows/noext",
        "README.md",
    ]);
    assert_eq!(totality(&listing, &[], &[]), []);
}

#[test]
fn the_extension_matches_in_any_case() {
    let listing = paths(&[".github/workflows/CI.YML"]);
    let want = [found(".github/workflows/CI.YML", None, Problem::Undeclared)];
    assert_eq!(totality(&listing, &[], &[]), want);
}

#[test]
fn a_listed_but_missing_path_is_a_finding() {
    let hw = [hand(4, ".github/workflows/gone.yml", "old")];
    let want = [found(
        ".github/workflows/gone.yml",
        Some(4),
        Problem::Missing,
    )];
    assert_eq!(totality(&[], &[], &hw), want);
}

#[test]
fn a_hand_written_path_also_rendered_is_a_finding() {
    let listing = paths(&[".github/workflows/ci.yml"]);
    let files = rendered(&[".github/workflows/ci.yml"]);
    let hw = [hand(5, ".github/workflows/ci.yml", "mine")];
    let want = [found(
        ".github/workflows/ci.yml",
        Some(5),
        Problem::AlsoRendered,
    )];
    assert_eq!(totality(&listing, &files, &hw), want);
}

#[test]
fn a_hand_written_row_without_reason_is_a_finding() {
    let listing = paths(&[".github/workflows/r.yml"]);
    let hw = [hand(6, ".github/workflows/r.yml", " ")];
    let want = [found(".github/workflows/r.yml", Some(6), Problem::NoReason)];
    assert_eq!(totality(&listing, &[], &hw), want);
}

#[test]
fn a_bot_file_without_a_row_is_a_finding() {
    let listing = paths(&[".coderabbit.yaml", "greptile.json", "socket.yml"]);
    let files = rendered(&["greptile.json"]);
    let want = [
        found(".coderabbit.yaml", None, Problem::BotWithoutRow),
        found("socket.yml", None, Problem::BotWithoutRow),
    ];
    assert_eq!(totality(&listing, &files, &[]), want);
}

/// The paths of `findings_are_sorted_by_path`, in the order wanted.
const SORTED: [&str; 4] = [
    ".github/workflows/a.yml",
    ".github/workflows/b.yml",
    ".github/workflows/z.yml",
    "socket.yml",
];

#[test]
fn findings_are_sorted_by_path() {
    let listing = paths(&["socket.yml", ".github/workflows/b.yml"]);
    let hw = [
        hand(3, ".github/workflows/z.yml", "x"),
        hand(2, ".github/workflows/a.yml", "x"),
    ];
    let got: Vec<String> = totality(&listing, &[], &hw)
        .into_iter()
        .map(|f| f.path)
        .collect();
    assert_eq!(got, SORTED);
}
