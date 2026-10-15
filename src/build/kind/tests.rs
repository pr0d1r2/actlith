//! Kind `build` end to end (T20, V30): each fixture row renders its
//! golden file byte for byte, and every golden file passes `actionlint`
//! and `zizmor --offline`, which the dev shell carries (`.:C` runtime
//! deps: test deps). A missing tool fails the test, never skips it.

use std::path::{Path, PathBuf};
use std::process::Command;

use super::Build;
use crate::kind::{Kind, Row};
use crate::pin::PinTable;
use crate::registry::parse;
use crate::yaml::render;

/// `(row file, golden file)` under `tests/fixtures/build/`.
const GOLDEN: [(&str, &str); 2] = [
    (
        include_str!("../../../tests/fixtures/build/build.actlith"),
        include_str!("../../../tests/fixtures/build/build.yml"),
    ),
    (
        include_str!("../../../tests/fixtures/build/picked.actlith"),
        include_str!("../../../tests/fixtures/build/picked.yml"),
    ),
];

const FILES: [&str; 2] = ["build.yml", "picked.yml"];

/// A hook's git environment, dropped from every tool the tests run.
const GIT_ENV: [&str; 6] = [
    "GIT_DIR",
    "GIT_WORK_TREE",
    "GIT_INDEX_FILE",
    "GIT_OBJECT_DIRECTORY",
    "GIT_ALTERNATE_OBJECT_DIRECTORIES",
    "GIT_COMMON_DIR",
];

fn row(text: &str) -> Result<Row, String> {
    let registry = parse(text).map_err(|e| e.to_string())?;
    let row = registry.workflows.first().ok_or("no row")?;
    Row::parse(&Build, row).map_err(|e| e.to_string())
}

fn rendered(text: &str) -> Result<String, String> {
    let built = Build.build(&row(text)?, &PinTable::builtin());
    built.map(|w| render(&w)).map_err(|e| e.to_string())
}

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/build")
        .join(name)
}

/// `tool args... file`, without the hook's git environment.
fn lint(tool: &str, args: &[&str], file: &Path) -> Result<(), String> {
    let mut command = Command::new(tool);
    for var in GIT_ENV {
        command.env_remove(var);
    }
    let out = command
        .args(args)
        .arg(file)
        .output()
        .map_err(|e| format!("{tool}: {e} (run inside `nix develop`)"))?;
    if out.status.success() {
        return Ok(());
    }
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    Err(format!("{tool} {}:\n{stdout}{stderr}", file.display()))
}

#[test]
fn every_fixture_row_renders_its_golden_file() -> Result<(), String> {
    for (text, golden) in GOLDEN {
        assert_eq!(rendered(text)?, golden);
    }
    Ok(())
}

#[test]
fn every_golden_file_passes_actionlint() -> Result<(), String> {
    for name in FILES {
        lint("actionlint", &["-no-color"], &fixture(name))?;
    }
    Ok(())
}

#[test]
fn every_golden_file_passes_zizmor_offline() -> Result<(), String> {
    for name in FILES {
        lint("zizmor", &["--offline", "--no-progress"], &fixture(name))?;
    }
    Ok(())
}

#[test]
fn the_kind_says_its_name_and_schema() {
    assert_eq!(Build.name(), "build");
    let keys: Vec<_> = Build.schema().iter().map(|s| s.key).collect();
    assert_eq!(keys, ["package", "timeout"]);
}

#[test]
fn the_kind_declares_the_cachix_secret() -> Result<(), String> {
    let (text, _) = GOLDEN.first().ok_or("no fixture")?;
    let secrets = Build.secrets(&row(text)?);
    let names: Vec<_> = secrets.iter().map(|s| s.name().to_owned()).collect();
    assert_eq!(names, ["CACHIX_AUTH_TOKEN"]);
    Ok(())
}

#[test]
fn every_script_it_runs_is_seeded_by_src_script() {
    assert!(!Build::scripts().is_empty());
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    for path in Build::scripts() {
        let name = path.strip_prefix("scripts/actlith/").unwrap_or_default();
        assert!(root.join("src/script").join(name).is_file(), "{path}");
    }
}

#[test]
fn a_refused_row_builds_nothing() -> Result<(), String> {
    let mut row = row(GOLDEN.first().ok_or("no fixture")?.0)?;
    row.cache = None;
    assert!(Build.build(&row, &PinTable::builtin()).is_err());
    assert!(Build.secrets(&row).is_empty());
    Ok(())
}
