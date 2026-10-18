//! Kind `gate` (T19): the golden against this repository's own `ci.yml`
//! (V29), and every arg, system and cache choice a row can make. The
//! `xcode` arg (T69, V52) renders its golden, which passes `actionlint`
//! and `zizmor --offline` (`tests:V4`; a missing tool fails the test).

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::gate::Gate;
use crate::kind::{Kind, Row};
use crate::pin::PinTable;
use crate::registry::parse;
use crate::yaml::{Comment, Workflow, render, write};

/// The hand-written ci.yml this repository had before actlith generated
/// it (nix T10): the golden shape the model and builders must reproduce.
const CI: &str = include_str!("../../../tests/fixtures/yaml/ci-hand.yml");
const ROW: &str = include_str!("../../../tests/fixtures/gate/ci.actlith");
const XCODE_ROW: &str =
    include_str!("../../../tests/fixtures/gate/xcode.actlith");
const XCODE_YML: &str = include_str!("../../../tests/fixtures/gate/xcode.yml");

const HEAD: &str = "format 1\n\n## workflows\nid|kind|systems|cache|args\n";

fn checked(text: &str) -> Result<Row, String> {
    let registry = parse(text).map_err(|e| e.to_string())?;
    let row = registry.workflows.first().ok_or("no row")?;
    crate::kind::check(&Gate, row).map_err(|e| e.to_string())
}

fn row(cells: &str) -> Result<Row, String> {
    checked(&format!("{HEAD}{cells}\n"))
}

fn built(row: &Row) -> Result<Workflow, String> {
    Gate.build(row, &PinTable::builtin())
        .map_err(|e| e.to_string())
}

/// The hand-written comment block at the top of `ci.yml`: every `# `
/// line between `---` and the first blank line.
fn ci_comments() -> Vec<Comment> {
    let lines: Vec<&str> = CI
        .lines()
        .skip(1)
        .take_while(|l| !l.is_empty())
        .filter_map(|l| l.strip_prefix("# "))
        .collect();
    vec![Comment::new(&lines.join("\n"))]
}

#[test]
fn the_own_row_renders_ci_yml_byte_for_byte() -> Result<(), String> {
    let mut gate = built(&checked(ROW)?)?;
    assert!(
        gate.comments.is_empty(),
        "the kind writes no header comment"
    );
    gate.comments = ci_comments();
    assert_eq!(write(&gate), CI);
    Ok(())
}

#[test]
fn the_kind_is_named_gate_and_takes_command_and_timeout() {
    assert_eq!(Gate.name(), "gate");
    let keys: Vec<&str> = Gate.spec().schema.iter().map(|s| s.key).collect();
    assert_eq!(keys, ["command", "timeout"]);
    let forge: Vec<&str> = Gate.forge_args().iter().map(|s| s.key).collect();
    assert_eq!(forge, ["xcode"]);
}

#[test]
fn the_job_is_named_gate() -> Result<(), String> {
    let gate = built(&row("ci|gate|-|-|-")?)?;
    let ids: Vec<&String> = gate.jobs.keys().collect();
    assert_eq!(ids, ["gate"]);
    let job = gate.jobs.get("gate").ok_or("no job")?;
    assert_eq!(job.name.as_deref(), Some("gate"));
    assert_eq!(job.timeout_minutes, 30);
    Ok(())
}

#[test]
fn a_cache_needs_the_cachix_secret() -> Result<(), String> {
    let names = |r: &Row| -> Vec<String> {
        Gate.secrets(r)
            .iter()
            .map(|s| s.name().to_owned())
            .collect()
    };
    assert_eq!(names(&row("ci|gate|-|pr0d1r2|-")?), ["CACHIX_AUTH_TOKEN"]);
    assert!(names(&row("ci|gate|-|-|-")?).is_empty());
    Ok(())
}

#[test]
fn no_cache_means_no_cachix_step() -> Result<(), String> {
    let text = write(&built(&row("ci|gate|-|-|-")?)?);
    assert!(!text.contains("cachix/cachix-action"), "{text}");
    assert!(!text.contains("CACHIX_AUTH_TOKEN"), "{text}");
    assert!(text.contains("install-nix-action"), "{text}");
    assert!(text.contains("fetch-depth: 0"), "{text}");
    Ok(())
}

#[test]
fn the_args_set_command_and_timeout() -> Result<(), String> {
    let gate = built(&row("ci|gate|-|-|command=just check,timeout=12")?)?;
    let text = write(&gate);
    assert!(text.contains("      - run: just check\n"), "{text}");
    assert!(text.contains("    timeout-minutes: 12\n"), "{text}");
    Ok(())
}

#[test]
fn the_systems_pick_the_runners() -> Result<(), String> {
    let text = write(&built(&row("ci|gate|x86_64-linux|-|-")?)?);
    assert!(text.contains("os: [ubuntu-latest]"), "{text}");
    Ok(())
}

#[test]
fn a_bad_arg_is_a_row_error() -> Result<(), String> {
    assert!(row("ci|gate|-|-|timeout=soon").is_err());
    let command = row("ci|gate|-|-|command=a ${{ b }}")?;
    assert!(built(&command).is_err());
    Ok(())
}

/// A hook's git environment, dropped from every tool the tests run.
const GIT_ENV: [&str; 6] = [
    "GIT_DIR",
    "GIT_WORK_TREE",
    "GIT_INDEX_FILE",
    "GIT_OBJECT_DIRECTORY",
    "GIT_ALTERNATE_OBJECT_DIRECTORIES",
    "GIT_COMMON_DIR",
];

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/gate")
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

/// The refusal of `cells`, as `.actlith:<line>: ...` words.
fn refused(cells: &str) -> Result<String, String> {
    match row(cells).and_then(|r| built(&r)) {
        Ok(_) => Err(format!("`{cells}` rendered")),
        Err(error) => Ok(error),
    }
}

#[test]
fn the_xcode_row_renders_its_golden_file() -> Result<(), String> {
    assert_eq!(render(&built(&checked(XCODE_ROW)?)?), XCODE_YML);
    Ok(())
}

#[test]
fn the_xcode_golden_passes_actionlint() -> Result<(), String> {
    lint("actionlint", &["-no-color"], &fixture("xcode.yml"))
}

#[test]
fn the_xcode_golden_passes_zizmor_offline() -> Result<(), String> {
    lint(
        "zizmor",
        &["--offline", "--no-progress"],
        &fixture("xcode.yml"),
    )
}

fn rendered_text(text: &crate::yaml::Text) -> String {
    text.render(&crate::yaml::GITHUB)
}

#[test]
fn xcode_sets_developer_dir_on_the_job() -> Result<(), String> {
    let gate = built(&row("ci|gate|aarch64-darwin|-|xcode=16.4")?)?;
    let job = gate.jobs.get("gate").ok_or("no job")?;
    let dir = job.env.get("DEVELOPER_DIR").map(rendered_text);
    assert_eq!(
        dir.as_deref(),
        Some("/Applications/Xcode_16.4.app/Contents/Developer")
    );
    Ok(())
}

#[test]
fn xcode_is_checked_by_the_first_step() -> Result<(), String> {
    let text = write(&built(&row("ci|gate|aarch64-darwin|-|xcode=16.4")?)?);
    let steps = text.split("    steps:\n").nth(1).unwrap_or_default();
    assert!(
        steps.starts_with("      # xcode=16.4: this Xcode"),
        "{text}"
    );
    let first = steps.lines().find(|l| l.starts_with("      - "));
    let checks =
        first.is_some_and(|l| l.contains("test -d \"$DEVELOPER_DIR\""));
    assert!(checks, "{text}");
    Ok(())
}

#[test]
fn rows_without_xcode_have_no_developer_dir() -> Result<(), String> {
    for cells in ["ci|gate|-|-|-", "ci|gate|aarch64-darwin|pr0d1r2|-"] {
        let text = write(&built(&row(cells)?)?);
        assert!(!text.contains("DEVELOPER_DIR"), "{text}");
        assert!(!text.contains("xcodebuild"), "{text}");
    }
    Ok(())
}

#[test]
fn xcode_on_a_linux_system_is_refused() -> Result<(), String> {
    for cells in [
        "ci|gate|-|-|xcode=26.4",
        "ci|gate|aarch64-darwin, x86_64-linux|-|xcode=26.4",
        "ci|gate|aarch64-linux|-|xcode=26.4",
    ] {
        let error = refused(cells)?;
        assert!(error.starts_with(".actlith:5: "), "{error}");
        assert!(error.contains("-linux` is Linux"), "{error}");
        assert!(error.contains("fix: list only darwin systems"), "{error}");
    }
    Ok(())
}

#[test]
fn xcode_must_be_a_version_number() -> Result<(), String> {
    for bad in ["latest", "26.", ".4", "26..4", "26.4-beta", "2 6.4"] {
        let error = refused(&format!("ci|gate|aarch64-darwin|-|xcode={bad}"))?;
        assert!(error.contains("is not an Xcode version"), "{bad}: {error}");
    }
    for good in ["26.4", "16.4", "26.0.1"] {
        row(&format!("ci|gate|aarch64-darwin|-|xcode={good}"))
            .and_then(|r| built(&r))?;
    }
    Ok(())
}
