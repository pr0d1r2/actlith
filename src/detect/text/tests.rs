//! Row text round-trips through `src/registry` (detect T27, `.:I` file).

use super::{FIRST_ROW, HEADER, text};
use crate::detect::row_line;
use crate::registry::{Arg, WorkflowRow, parse};

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

fn full() -> WorkflowRow {
    WorkflowRow {
        systems: vec!["x86_64-linux".to_owned(), "aarch64-linux".to_owned()],
        cache: Some("pr0d1r2".to_owned()),
        args: vec![
            Arg {
                key: "command".to_owned(),
                value: "hk check | tee log".to_owned(),
            },
            Arg {
                key: "timeout".to_owned(),
                value: "30".to_owned(),
            },
        ],
        ..bare(FIRST_ROW, "ci", "gate")
    }
}

#[test]
fn an_empty_row_is_all_dashes() {
    assert_eq!(row_line(&bare(1, "ci", "gate")), "ci|gate|-|-|-");
}

#[test]
fn a_full_row_lists_and_escapes() {
    assert_eq!(
        row_line(&full()),
        "ci|gate|x86_64-linux, aarch64-linux|pr0d1r2|\
         command=hk check \\| tee log, timeout=30"
    );
}

#[test]
fn text_is_the_header_then_rows() {
    let rows = [bare(5, "ci", "gate"), bare(6, "update-pins", "update-pins")];
    let want = include_str!("../../../tests/fixtures/detect/both.actlith");
    assert_eq!(text(&rows), want);
    assert_eq!(text(&[]), HEADER);
}

#[test]
fn text_parses_back_to_the_rows() {
    let rows = vec![full(), bare(FIRST_ROW + 1, "update-pins", "update-pins")];
    let reg = parse(&text(&rows)).unwrap_or_default();
    assert_eq!(reg.workflows, rows);
    assert_eq!(parse(&text(&[])).map(|r| r.workflows.len()), Ok(0));
}
