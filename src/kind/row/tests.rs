//! A row checked for a kind: default and listed systems, cache, args
//! (T17, `src/registry` V10).

use crate::kind::{ArgSchema, Cache, Kind, KindError, Row, Secret, TIMEOUT};
use crate::pin::PinTable;
use crate::registry::{WorkflowRow, parse};
use crate::system::System;
use crate::yaml::Workflow;

const ROWS: &str = include_str!("../../../tests/fixtures/kind/rows.actlith");

const SCHEMA: [crate::kind::ArgSpec; 1] = [TIMEOUT];

/// A kind that takes `timeout` and defaults to one system.
struct Probe;

impl Kind for Probe {
    fn name(&self) -> &'static str {
        "probe"
    }

    fn schema(&self) -> &'static ArgSchema {
        &SCHEMA
    }

    fn default_systems(&self) -> &'static [System] {
        &[System::Aarch64Linux]
    }

    fn secrets(&self, _row: &Row) -> Vec<Secret> {
        Vec::new()
    }

    fn build(&self, row: &Row, _: &PinTable) -> Result<Workflow, KindError> {
        Err(KindError::row(row.line, "probe", "none"))
    }
}

fn workflow_row(id: &str) -> Option<WorkflowRow> {
    let registry = parse(ROWS).ok()?;
    registry.workflows.into_iter().find(|w| w.id == id)
}

fn checked(id: &str) -> Result<Row, String> {
    let row = workflow_row(id).ok_or("no such row")?;
    Row::parse(&Probe, &row).map_err(|e| e.to_string())
}

fn refused(id: &str) -> String {
    checked(id).err().unwrap_or_default()
}

#[test]
fn a_dash_row_takes_the_kind_defaults() -> Result<(), String> {
    let row = checked("plain")?;
    assert_eq!((row.id.as_str(), row.line), ("plain", 7));
    assert_eq!(row.systems, [System::Aarch64Linux]);
    assert_eq!(row.cache, None);
    assert_eq!(row.args.get("timeout"), Some("30"));
    Ok(())
}

#[test]
fn listed_systems_keep_their_order() -> Result<(), String> {
    let row = checked("picked")?;
    assert_eq!(row.systems, [System::Aarch64Darwin, System::X86_64Linux]);
    let cache = Cache::parse(8, "pr0d1r2").map_err(|e| e.to_string())?;
    assert_eq!(row.cache, Some(cache));
    assert_eq!(row.args.get("timeout"), Some("45"));
    Ok(())
}

#[test]
fn a_system_without_a_runner_is_refused() {
    assert_eq!(
        refused("intel"),
        ".actlith:9: system `x86_64-darwin` has no GitHub-hosted runner; \
         build and verify it by hand on an Intel Mac\n  fix: write systems \
         with a hosted runner, or `-`"
    );
}

#[test]
fn an_unknown_system_is_refused() {
    let error = refused("riscv");
    assert!(
        error.starts_with(".actlith:10: unknown system `riscv64-linux`"),
        "{error}"
    );
}

#[test]
fn a_repeated_system_is_refused() {
    assert_eq!(
        refused("twice"),
        ".actlith:11: system `x86_64-linux` is listed twice\n  fix: keep one \
         `x86_64-linux`"
    );
}

#[test]
fn an_unknown_cache_is_refused() {
    let error = refused("stranger");
    assert!(
        error.starts_with(".actlith:12: cache `nix-community`"),
        "{error}"
    );
}

#[test]
fn an_unknown_arg_is_refused() {
    let error = refused("extra");
    assert!(
        error.starts_with(".actlith:13: kind `probe` has no arg `colour`"),
        "{error}"
    );
}

#[test]
fn the_probe_kind_answers_its_contract() -> Result<(), String> {
    let row = checked("plain")?;
    assert_eq!(Probe.name(), "probe");
    assert!(Probe.secrets(&row).is_empty());
    let built = Probe.build(&row, &PinTable::builtin()).err();
    assert_eq!(built, Some(KindError::row(7, "probe", "none")));
    Ok(())
}
