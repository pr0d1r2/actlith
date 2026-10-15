//! `pin` with a fake resolver: no network (src V4, `src/fetch:V16`).

use std::fs;

use super::pin;
use crate::cli::exit::Outcome;
use crate::cli::scratch::{Fake, SHA, put, scratch};

const ROWS: &str = "format 1\n\n## workflows\nid|kind|systems|cache|args\n";

/// Either side of a verb's result.
fn ran(result: Result<Outcome, Outcome>) -> Outcome {
    result.unwrap_or_else(|outcome| outcome)
}

#[test]
fn dry_run_prints_the_rows_and_writes_nothing() {
    let root = scratch("pin-dry");
    put(&root, ".actlith", ROWS);
    let rows = ran(pin(&root, true, &Fake));
    assert_eq!(rows.code, 0);
    assert!(rows.out.starts_with("action|version|sha\n"));
    assert!(
        rows.out
            .contains(&format!("actions/checkout|v7.0.1|{SHA}\n"))
    );
    let text = fs::read_to_string(root.join(".actlith")).unwrap_or_default();
    assert_eq!(text, ROWS);
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn pin_writes_the_actions_section() {
    let root = scratch("pin-write");
    put(&root, ".actlith", ROWS);
    assert_eq!(ran(pin(&root, false, &Fake)).code, 0);
    let text = fs::read_to_string(root.join(".actlith")).unwrap_or_default();
    assert!(text.contains("## actions\naction|version|sha\n"));
    assert!(text.contains(SHA));
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn a_missing_actlith_is_exit_two_with_the_fix() {
    let root = scratch("pin-missing");
    let failed = ran(pin(&root, true, &Fake));
    assert_eq!(failed.code, 2);
    assert!(failed.err.contains("fix: "));
    let _ = fs::remove_dir_all(&root);
}
