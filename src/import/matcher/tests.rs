//! `import` (T26, V17): this repository's own `ci.yml` and a reshaped
//! hand-written gate import; every kind's render imports back to its
//! row; drift and foreign shapes are refused at their first differing
//! key, never with a partial row.

use crate::import::{Mismatch, import};
use crate::registry::{WorkflowRow, parse};
use crate::render::render;

const CI: &str = include_str!("../../../.github/workflows/ci.yml");
const RESHAPED: &str =
    include_str!("../../../tests/fixtures/import/reshaped.yml");
const FOREIGN: &str =
    include_str!("../../../tests/fixtures/import/foreign.yml");

const HEAD: &str = "format 1\n\n## workflows\nid|kind|systems|cache|args\n";

/// `cells` parsed as `import` parses its candidates.
fn row(cells: &str) -> Result<WorkflowRow, String> {
    let registry =
        parse(&format!("{HEAD}{cells}\n")).map_err(|e| e.to_string())?;
    registry
        .workflows
        .first()
        .cloned()
        .ok_or_else(|| "no row".into())
}

/// The workflow file `cells` renders.
fn rendered(cells: &str) -> Result<String, String> {
    let registry =
        parse(&format!("{HEAD}{cells}\n")).map_err(|e| e.to_string())?;
    let files = render(&registry).map_err(|e| e.to_string())?;
    let file = files
        .into_iter()
        .find(|f| f.path.starts_with(".github/workflows/"));
    file.map(|f| f.bytes).ok_or_else(|| "no workflow".into())
}

fn refused(text: &str) -> Result<Mismatch, String> {
    import(text).err().ok_or_else(|| "imported".into())
}

#[test]
fn the_own_ci_yml_imports_as_its_row() -> Result<(), String> {
    assert_eq!(import(CI), Ok(row("ci|gate|-|pr0d1r2|-")?));
    Ok(())
}

#[test]
fn a_reshaped_gate_imports_with_its_systems_and_args() -> Result<(), String> {
    let cells = "lint|gate|aarch64-darwin, x86_64-linux|-|command=nix flake \
                 check --all-systems, timeout=45";
    assert_eq!(import(RESHAPED), Ok(row(cells)?));
    Ok(())
}

#[test]
fn layout_alone_does_not_stop_an_import() -> Result<(), String> {
    let flat = CI
        .replace("  # v", " # v")
        .replace(">-\n            ${{", "${{")
        .replace(" &&\n            github", " && github")
        .replace("---\n", "");
    let spaced = flat.replace('\n', "  \n\n");
    assert_eq!(import(&spaced), Ok(row("ci|gate|-|pr0d1r2|-")?));
    Ok(())
}

#[test]
fn every_kind_imports_back_to_its_row() -> Result<(), String> {
    for cells in [
        "lint|gate|aarch64-linux|-|command=nix flake check, timeout=10",
        "build|build|-|pr0d1r2|package=hk",
        "pkg|build|x86_64-linux, aarch64-linux|pr0d1r2|package=tool, timeout=60",
        "update-pins|update-pins|-|-|-",
        "pins|update-pins|-|pr0d1r2|cron=0 5 * * 1, build=.#hk, \
         token=PINS_TOKEN, timeout=45",
        "upstream|upstream|-|-|script=scripts/bump-hk.sh",
        "up|upstream|aarch64-linux|pr0d1r2|script=scripts/bump.sh, \
         cron=15 */6 * * *, branch-prefix=bump/hk-, token=BUMP_TOKEN",
    ] {
        assert_eq!(import(&rendered(cells)?), Ok(row(cells)?), "{cells}");
    }
    Ok(())
}

#[test]
fn a_row_spelling_out_defaults_imports_as_the_shortest() -> Result<(), String> {
    let long = "ci|gate|x86_64-linux, aarch64-linux, aarch64-darwin|pr0d1r2|\
                command=nix develop --command hk check --all --check, \
                timeout=30";
    assert_eq!(import(&rendered(long)?), Ok(row("ci|gate|-|pr0d1r2|-")?));
    Ok(())
}

#[test]
fn drift_is_refused_at_its_key() -> Result<(), String> {
    let drifted = CI.replace("fail-fast: false", "fail-fast: true");
    let line = CI
        .lines()
        .position(|l| l.contains("fail-fast"))
        .map(|at| at.saturating_add(1));
    let mismatch = refused(&drifted)?;
    assert_eq!(Some(mismatch.line), line);
    assert_eq!(mismatch.key, "jobs.gate.strategy.fail-fast");
    assert_eq!(mismatch.kind.as_deref(), Some("gate"));
    assert_eq!(mismatch.expected.as_deref(), Some("fail-fast: false"));
    assert_eq!(mismatch.found.as_deref(), Some("fail-fast: true"));
    Ok(())
}

#[test]
fn a_foreign_shape_is_refused_against_the_closest_kind() -> Result<(), String> {
    let mismatch = refused(FOREIGN)?;
    assert_eq!(mismatch.line, 9);
    assert_eq!(mismatch.key, "on.pull_request.branches");
    assert_eq!(mismatch.kind.as_deref(), Some("gate"));
    assert_eq!(mismatch.expected.as_deref(), Some("workflow_dispatch:"));
    Ok(())
}

#[test]
fn a_trailing_extra_line_is_refused() -> Result<(), String> {
    let mismatch = refused(&format!("{CI}      - run: echo more\n"))?;
    assert_eq!(mismatch.key, "jobs.gate.steps.run");
    assert_eq!(mismatch.expected, None);
    Ok(())
}

#[test]
fn a_name_no_row_id_can_be_is_refused() -> Result<(), String> {
    let mismatch = refused(&CI.replace("name: ci", "name: C I"))?;
    assert_eq!((mismatch.key.as_str(), mismatch.kind), ("name", None));
    let unnamed = refused("on: push\n")?;
    assert_eq!((unnamed.line, unnamed.kind), (1, None));
    Ok(())
}
