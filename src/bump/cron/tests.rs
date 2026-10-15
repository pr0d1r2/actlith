//! The `cron` arg: five fields or refused (T22, T23).

use crate::bump::cron::cron;
use crate::kind::{ArgSpec, Args, parse_args};
use crate::registry::parse;

const SCHEMA: [ArgSpec; 1] = [ArgSpec {
    key: "cron",
    required: false,
    default: Some("50 6 * * *"),
}];

fn args(cell: &str) -> Result<Args, String> {
    let text = format!(
        "format 1\n## workflows\nid|kind|systems|cache|args\nw|k|-|-|{cell}\n"
    );
    let registry = parse(&text).map_err(|e| e.to_string())?;
    let row = registry.workflows.first().ok_or("no row")?;
    parse_args(&SCHEMA, row).map_err(|e| e.to_string())
}

fn checked(cell: &str) -> Result<Result<String, String>, String> {
    Ok(cron(&args(cell)?).map_err(|e| e.to_string()))
}

#[test]
fn the_default_is_five_fields() -> Result<(), String> {
    assert_eq!(checked("-")?, Ok("50 6 * * *".to_owned()));
    Ok(())
}

#[test]
fn ranges_steps_lists_of_names_are_fields() -> Result<(), String> {
    let want = "*/15 6-8 1 JAN MON-FRI";
    assert_eq!(checked(&format!("cron={want}"))?, Ok(want.to_owned()));
    Ok(())
}

#[test]
fn runs_of_spaces_become_one() -> Result<(), String> {
    assert_eq!(checked("cron=0  5   * * 1")?, Ok("0 5 * * 1".to_owned()));
    Ok(())
}

#[test]
fn four_or_six_fields_are_refused() -> Result<(), String> {
    for cell in ["cron=50 6 * *", "cron=0 50 6 * * *"] {
        let error = checked(cell)?.err().unwrap_or_default();
        assert!(error.contains(".actlith:4"), "{error}");
        assert!(error.contains("not five cron fields"), "{error}");
    }
    Ok(())
}

#[test]
fn a_field_with_another_character_is_refused() -> Result<(), String> {
    let error = checked("cron=50 6 * * ?")?.err().unwrap_or_default();
    assert!(error.contains("cron `50 6 * * ?`"), "{error}");
    Ok(())
}
