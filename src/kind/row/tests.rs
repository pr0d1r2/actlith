//! A row checked for a kind: default and listed systems, cache, args,
//! in actlith's `.actlith:<line>` words (T17, T65, `src/registry` V10).

use crate::kind::{
    Cache, EXTRAS, Kind, KindError, KindSpec, Row, Secret, TIMEOUT, portable,
};
use crate::pin::PinTable;
use crate::registry::{WorkflowRow, parse};
use crate::system::System;
use crate::yaml::Workflow;

const ROWS: &str = include_str!("../../../tests/fixtures/kind/rows.actlith");

/// Every kind with every arg and system shape actlith renders.
const ALL: &str = include_str!("../../../tests/fixtures/render/all.actlith");

/// A kind that takes `timeout` and defaults to one system.
const PROBE: KindSpec = KindSpec {
    name: "probe",
    schema: &[TIMEOUT],
    default_systems: &[System::Aarch64Linux],
    secrets: &[],
};

struct Probe;

impl Kind for Probe {
    fn spec(&self) -> &'static KindSpec {
        &PROBE
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
    crate::kind::check(&Probe, &row).map_err(|e| e.to_string())
}

fn refused_args(cell: &str) -> String {
    let text = format!(
        "format 1\n## workflows\nid|kind|systems|cache|args\n\
         w|probe|-|-|{cell}\n"
    );
    let row = parse(&text)
        .ok()
        .and_then(|r| r.workflows.into_iter().next());
    let row = row.unwrap_or_default();
    crate::kind::check(&Probe, &row)
        .err()
        .map(|e| e.to_string())
        .unwrap_or_default()
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
fn an_unknown_arg_is_refused() {
    assert_eq!(
        refused("extra"),
        ".actlith:13: kind `probe` has no arg `colour`\n  fix: remove \
         `colour` or use one of: timeout"
    );
}

#[test]
fn a_timeout_out_of_range_is_refused() {
    for minutes in ["0", "361"] {
        let want = format!(
            ".actlith:4: timeout `{minutes}` is not 1 to 360 minutes\n  fix: \
             write `timeout=<1..360>`"
        );
        assert_eq!(refused_args(&format!("timeout={minutes}")), want);
    }
    assert_eq!(
        refused_args("timeout=soon"),
        ".actlith:4: arg `timeout` is `soon`, not a whole number\n  fix: \
         write `timeout=<number>`"
    );
}

#[test]
fn an_unknown_cache_names_the_known_ones() {
    assert_eq!(
        refused("stranger"),
        ".actlith:12: cache `nix-community` has no known public key\n  fix: \
         use one of: pr0d1r2, or `-` for no cache"
    );
}

#[test]
fn every_rendered_row_renders_on_every_forge() -> Result<(), String> {
    let registry = parse(ALL).map_err(|e| e.to_string())?;
    assert!(!registry.workflows.is_empty());
    for row in &registry.workflows {
        assert_eq!(portable(row, &EXTRAS), [], "{}", row.id);
    }
    assert!(EXTRAS.kinds.is_empty());
    Ok(())
}

#[test]
fn xcode_and_cache_push_are_the_github_only_keys() -> Result<(), String> {
    let keys: Vec<&str> = EXTRAS.keys.iter().map(|s| s.key).collect();
    assert_eq!(keys, ["xcode", "cache-push"]);
    let text = "format 1\n\n## workflows\nid|kind|systems|cache|args\n\
                ci|gate|aarch64-darwin|-|xcode=26.4\n";
    let registry = parse(text).map_err(|e| e.to_string())?;
    let row = registry.workflows.first().ok_or("no row")?;
    let found: Vec<String> = portable(row, &EXTRAS)
        .iter()
        .map(ToString::to_string)
        .collect();
    assert_eq!(
        found,
        ["line 5: arg `xcode` is forge-only; the other forge refuses this row"]
    );
    Ok(())
}

#[test]
fn the_probe_kind_answers_its_contract() -> Result<(), String> {
    let row = checked("plain")?;
    assert_eq!(Probe.name(), "probe");
    assert_eq!(Probe.spec().default_systems, [System::Aarch64Linux]);
    assert!(Probe.secrets(&row).is_empty());
    assert!(Probe.forge_args().is_empty(), "no forge key by default");
    let built = Probe.build(&row, &PinTable::builtin()).err();
    assert_eq!(built, Some(KindError::row(7, "probe", "none")));
    Ok(())
}
