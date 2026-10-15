//! `report` over a temp root: the golden JSON, and refusals (V34).

use std::fs;

use super::{of, report};
use crate::cli::exit::Outcome;
use crate::cli::scratch::{put, scratch};
use crate::registry::{ActionPin, Registry};
use crate::report::Format;

const ALL: &str = include_str!("../../../tests/fixtures/render/all.actlith");
const JSON: &str = include_str!("../../../tests/fixtures/report/all.json");
const ROWS: &str = "format 1\n\n## workflows\nid|kind|systems|cache|args\n";

/// Either side of a verb's result.
fn ran(result: Result<Outcome, Outcome>) -> Outcome {
    result.unwrap_or_else(|outcome| outcome)
}

#[test]
fn json_is_the_golden_document() {
    let root = scratch("report-json");
    put(&root, ".actlith", ALL);
    assert_eq!(ran(report(&root, Format::Json)), Outcome::ok(JSON));
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn a_missing_actlith_is_exit_two() {
    let root = scratch("report-missing");
    assert_eq!(ran(report(&root, Format::Text)).code, 2);
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn a_refused_pin_is_exit_two() {
    // `parse` refuses such a SHA first; the pin table checks again.
    let pin = ActionPin {
        line: 7,
        action: "actions/checkout".to_owned(),
        version: "v1".to_owned(),
        sha: "XYZ".to_owned(),
    };
    let actions = vec![pin];
    let registry = Registry {
        actions,
        ..Registry::default()
    };
    let failed = ran(of(&registry, Format::Text));
    assert_eq!(failed.code, 2);
    assert!(failed.err.contains(".actlith:7: pin `XYZ`"));
}

#[test]
fn an_unknown_kind_is_exit_two() {
    let root = scratch("report-kind");
    put(&root, ".actlith", &format!("{ROWS}x|nope|-|-|-\n"));
    let failed = ran(report(&root, Format::Md));
    assert_eq!(failed.code, 2);
    assert!(failed.err.contains("fix: "));
    let _ = fs::remove_dir_all(&root);
}
