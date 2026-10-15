//! `## actions` rows and the one-section rewrite (V16, T29).

use crate::fetch::section::{HEADER, replace, rows};
use crate::pin::Uses;
use crate::registry::parse;

const FULL: &str = include_str!("../../../tests/fixtures/fetch/full.actlith");
const NONE: &str =
    include_str!("../../../tests/fixtures/fetch/no-actions.actlith");
const EMPTY: &str =
    include_str!("../../../tests/fixtures/fetch/empty-actions.actlith");

const OLD_ROWS: &str = "action|version|sha
actions/checkout|v4.2.2|11bd71901bbe5b1630ceea73d27597364c9af683
example/action|v1.0.0|0123456789abcdef0123456789abcdef01234567
";

const SHA: &str = "0123456789abcdef0123456789abcdef01234567";

fn pins() -> Vec<Uses> {
    let uses = |action: &str, version: &str| Uses {
        action: action.to_owned(),
        sha: SHA.to_owned(),
        version: version.to_owned(),
    };
    vec![uses("actions/checkout", "v7.0.2"), uses("cachix/x", "v1")]
}

fn new_rows() -> String {
    format!("{HEADER}\nactions/checkout|v7.0.2|{SHA}\ncachix/x|v1|{SHA}\n")
}

#[test]
fn rows_are_the_header_and_one_row_per_pin() {
    assert_eq!(rows(&pins()), new_rows());
}

#[test]
fn no_pins_is_the_header_alone() {
    assert_eq!(rows(&[]), format!("{HEADER}\n"));
}

#[test]
fn replace_touches_only_the_rows_of_the_actions_section() {
    let want = FULL.replacen(OLD_ROWS, &new_rows(), 1);
    assert_ne!(want, FULL);
    assert_eq!(replace(FULL, &new_rows()), want);
}

#[test]
fn replaced_text_parses_to_the_new_rows() {
    let reg = parse(&replace(FULL, &new_rows())).unwrap_or_default();
    let got: Vec<(String, String)> = reg
        .actions
        .into_iter()
        .map(|a| (a.action, a.version))
        .collect();
    let want = [("actions/checkout", "v7.0.2"), ("cachix/x", "v1")]
        .map(|(a, v)| (a.to_owned(), v.to_owned()));
    assert_eq!(got, want);
    assert_eq!(reg.hand_written.len(), 1);
}

#[test]
fn an_empty_section_gets_the_rows_before_the_next_heading() {
    let want = EMPTY.replacen(
        "## actions\n",
        &format!("## actions\n{}", new_rows()),
        1,
    );
    assert_eq!(replace(EMPTY, &new_rows()), want);
}

#[test]
fn a_missing_section_is_appended() {
    let want = format!("{NONE}\n## actions\n{}", new_rows());
    assert_eq!(replace(NONE, &new_rows()), want);
}

#[test]
fn a_section_ending_the_file_is_replaced_in_place() {
    let text = "format 1\n\n##  actions \naction|version|sha\na/b|v0|x\n";
    let want = format!("format 1\n\n##  actions \n{}", new_rows());
    assert_eq!(replace(text, &new_rows()), want);
}
