//! Kind `gate` (T19): the golden against this repository's own `ci.yml`
//! (V29), and every arg, system and cache choice a row can make.

use crate::gate::Gate;
use crate::kind::{Kind, Row};
use crate::pin::PinTable;
use crate::registry::parse;
use crate::system::DEFAULT_SYSTEMS;
use crate::yaml::{Comment, Workflow, write};

const CI: &str = include_str!("../../../.github/workflows/ci.yml");
const ROW: &str = include_str!("../../../tests/fixtures/gate/ci.actlith");

const HEAD: &str = "format 1\n\n## workflows\nid|kind|systems|cache|args\n";

fn checked(text: &str) -> Result<Row, String> {
    let registry = parse(text).map_err(|e| e.to_string())?;
    let row = registry.workflows.first().ok_or("no row")?;
    Row::parse(&Gate, row).map_err(|e| e.to_string())
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
fn the_kind_is_named_gate_and_runs_on_three_systems() {
    assert_eq!(Gate.name(), "gate");
    assert_eq!(Gate.default_systems(), DEFAULT_SYSTEMS);
    let keys: Vec<&str> = Gate.schema().iter().map(|s| s.key).collect();
    assert_eq!(keys, ["command", "timeout"]);
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
    let timeout = row("ci|gate|-|-|timeout=soon")?;
    assert!(built(&timeout).is_err());
    let command = row("ci|gate|-|-|command=a ${{ b }}")?;
    assert!(built(&command).is_err());
    Ok(())
}
