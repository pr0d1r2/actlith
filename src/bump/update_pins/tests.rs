//! Kind `update-pins` (T22, V26, R5, `src/kind:V3`, `src/kind:V14`):
//! golden files, the declared secret, and every refused row.

use crate::bump::UpdatePins;
use crate::kind::{Kind, Row};
use crate::pin::PinTable;
use crate::registry::parse;
use crate::system::System;
use crate::yaml::render;

const ROWS: &str =
    include_str!("../../../tests/fixtures/bump/update-pins.actlith");
const DEFAULT: &str =
    include_str!("../../../tests/fixtures/bump/update-pins.yml");
const CUSTOM: &str =
    include_str!("../../../tests/fixtures/bump/pins-custom.yml");

fn row(id: &str) -> Result<Row, String> {
    let registry = parse(ROWS).map_err(|e| e.to_string())?;
    let row = registry.workflows.iter().find(|r| r.id == id);
    crate::kind::check(&UpdatePins, row.ok_or("no row")?)
        .map_err(|e| e.to_string())
}

fn built(id: &str) -> Result<String, String> {
    let workflow = UpdatePins.build(&row(id)?, &PinTable::builtin());
    workflow.map(|w| render(&w)).map_err(|e| e.to_string())
}

fn refusal(id: &str) -> Result<String, String> {
    match built(id) {
        Ok(text) => Err(format!("built:\n{text}")),
        Err(error) => Ok(error),
    }
}

#[test]
fn the_default_row_renders_the_golden_file() -> Result<(), String> {
    assert_eq!(built("update-pins")?, DEFAULT);
    Ok(())
}

#[test]
fn every_arg_and_the_cache_render_the_golden_file() -> Result<(), String> {
    assert_eq!(built("pins-custom")?, CUSTOM);
    Ok(())
}

#[test]
fn runs_are_never_cancelled() -> Result<(), String> {
    for text in [built("update-pins")?, built("pins-custom")?] {
        assert!(text.contains("cancel-in-progress: false"), "{text}");
        assert!(!text.contains("cancel-in-progress: true"), "{text}");
    }
    Ok(())
}

#[test]
fn the_seeded_script_it_calls_is_shipped() {
    // `src/script` seeds this file as `scripts/actlith/update-pins.sh`;
    // `include_str!` fails the build if it is gone.
    let seeded = include_str!("../../script/update-pins.sh");
    assert!(seeded.contains("open-pr.sh"));
    assert!(DEFAULT.contains("bash scripts/actlith/update-pins.sh"));
}

#[test]
fn it_declares_the_token_it_reads() -> Result<(), String> {
    let names = |id| -> Result<Vec<String>, String> {
        let secrets = UpdatePins.secrets(&row(id)?);
        Ok(secrets.iter().map(|s| s.name().to_owned()).collect())
    };
    assert_eq!(names("update-pins")?, ["GITHUB_TOKEN"]);
    assert_eq!(names("pins-custom")?, ["PINS_TOKEN"]);
    Ok(())
}

#[test]
fn a_bad_token_is_refused_before_it_is_declared() -> Result<(), String> {
    assert!(row("bad-token").is_err());
    assert!(refusal("bad-token")?.contains("token `1BAD`"));
    Ok(())
}

#[test]
fn a_cron_without_five_fields_is_refused() -> Result<(), String> {
    for id in ["short-cron", "odd-cron"] {
        let error = refusal(id)?;
        assert!(error.contains("not five cron fields"), "{error}");
    }
    Ok(())
}

#[test]
fn a_build_starting_with_a_dash_is_refused() -> Result<(), String> {
    assert!(refusal("dash-build")?.contains("arg `build` is `-x`"));
    Ok(())
}

#[test]
fn more_than_one_system_is_refused() -> Result<(), String> {
    assert!(refusal("two")?.contains("2 systems given"));
    Ok(())
}

#[test]
fn the_kind_names_its_schema_and_default_system() {
    let keys: Vec<&str> =
        UpdatePins.spec().schema.iter().map(|s| s.key).collect();
    assert_eq!(keys, ["cron", "build", "token", "timeout"]);
    assert_eq!(UpdatePins.name(), "update-pins");
    assert_eq!(UpdatePins.spec().default_systems, [System::X86_64Linux]);
}
