//! `report` end to end (T28, V33): the render fixture `.actlith` (one row
//! of every v1 kind, a hand-written file, every v1 bot) reports the
//! golden text (below), JSON and Markdown (under
//! `tests/fixtures/report/`); a refused row is the error.

use crate::pin::PinTable;
use crate::registry::parse;
use crate::report::{Format, report};

const ALL: &str = include_str!("../../../tests/fixtures/render/all.actlith");
/// The golden text. Inline, not a fixture file: `.pklith` has no `txt`
/// type, and `pklith check` refuses a tracked file type without one.
const TEXT: &str = "workflows\n";
const JSON: &str = include_str!("../../../tests/fixtures/report/all.json");
const MD: &str = include_str!("../../../tests/fixtures/report/all.md");

fn reported(text: &str, format: Format) -> Result<String, String> {
    let registry = parse(text).map_err(|e| e.to_string())?;
    let pins = PinTable::builtin()
        .with_overrides(&registry.actions)
        .map_err(|e| e.to_string())?;
    report(&registry, &pins, format).map_err(|e| e.to_string())
}

#[test]
fn the_fixture_reports_the_golden_text() -> Result<(), String> {
    assert_eq!(reported(ALL, Format::Text)?, TEXT);
    Ok(())
}

#[test]
fn the_fixture_reports_the_golden_json() -> Result<(), String> {
    assert_eq!(reported(ALL, Format::Json)?, JSON);
    Ok(())
}

#[test]
fn the_fixture_reports_the_golden_markdown() -> Result<(), String> {
    assert_eq!(reported(ALL, Format::Md)?, MD);
    Ok(())
}

#[test]
fn a_refused_row_is_the_error() {
    let text = "format 1\n\n## workflows\nid|kind|systems|cache|args\n\
                ci|deploy|-|-|-\n";
    let got = reported(text, Format::Json);
    assert!(got.is_err_and(|e| e.starts_with(".actlith:5: unknown kind")));
}
