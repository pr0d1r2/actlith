//! The `build` jobs (T20, V30): matrix per system, push only from
//! `main`, flake check plus eval-only systems, verify-cache through the
//! seeded script and never inline shell.

use crate::yaml::GITHUB;
use std::collections::BTreeMap;

use super::{VERIFY_SCRIPT, build, check, eval_only, jobs, matrix, verify};
use crate::build::Build;
use crate::build::plan::Plan;
use crate::kind::{Row, SKIP_UNLESS_MAIN};
use crate::pin::PinTable;
use crate::registry::parse;
use crate::system::System;
use crate::yaml::{Action, Job, Step, Value};

const ROW: &str = include_str!("../../../tests/fixtures/build/picked.actlith");

fn picked() -> Result<Row, String> {
    let registry = parse(ROW).map_err(|e| e.to_string())?;
    let row = registry.workflows.first().ok_or("no row")?;
    Row::parse(&Build, row).map_err(|e| e.to_string())
}

fn made(
    row: &Row,
    part: fn(&Plan) -> Result<Job, crate::kind::KindError>,
) -> Result<Job, String> {
    let pins = PinTable::builtin();
    let plan = Plan::new(row, &pins).map_err(|e| e.to_string())?;
    part(&plan).map_err(|e| e.to_string())
}

fn commands(job: &Job) -> Vec<&str> {
    job.steps
        .iter()
        .filter_map(|s| match &s.action {
            Action::Run(command) => Some(command.as_str()),
            Action::Uses(_) => None,
        })
        .collect()
}

fn actions(job: &Job) -> Vec<String> {
    job.steps.iter().filter_map(action).collect()
}

fn action(step: &Step) -> Option<String> {
    match &step.action {
        Action::Uses(uses) => uses.reference.split('@').next().map(Into::into),
        Action::Run(_) => None,
    }
}

#[test]
fn every_job_is_there_by_its_stable_id() -> Result<(), String> {
    let row = picked()?;
    let pins = PinTable::builtin();
    let plan = Plan::new(&row, &pins).map_err(|e| e.to_string())?;
    let all = jobs(&plan).map_err(|e| e.to_string())?;
    let ids: Vec<&str> = all.keys().map(String::as_str).collect();
    assert_eq!(ids, ["build", "check", "verify-cache"]);
    for (id, job) in &all {
        assert_eq!(job.name.as_deref(), Some(id.as_str()));
        assert_eq!(job.timeout_minutes, 60);
    }
    Ok(())
}

fn pair(os: &str, system: &str) -> BTreeMap<String, String> {
    BTreeMap::from([
        ("os".to_owned(), os.to_owned()),
        ("system".to_owned(), system.to_owned()),
    ])
}

#[test]
fn build_runs_one_runner_per_system() -> Result<(), String> {
    let row = picked()?;
    let pins = PinTable::builtin();
    let plan = Plan::new(&row, &pins).map_err(|e| e.to_string())?;
    let got = matrix(&plan).map_err(|e| e.to_string())?;
    let want = [
        pair("macos-latest", "aarch64-darwin"),
        pair("ubuntu-latest", "x86_64-linux"),
    ];
    assert_eq!(got.include, want);
    assert!(!got.fail_fast && got.axes.is_empty());
    Ok(())
}

#[test]
fn a_system_without_a_runner_is_refused_in_the_matrix() -> Result<(), String> {
    let mut row = picked()?;
    row.systems = vec![System::X86_64Darwin];
    let pins = PinTable::builtin();
    let plan = Plan::new(&row, &pins).map_err(|e| e.to_string())?;
    let error = matrix(&plan).err().map(|e| e.to_string());
    let error = error.unwrap_or_default();
    assert!(error.contains("x86_64-darwin"), "{error}");
    assert!(build(&plan).is_err());
    Ok(())
}

#[test]
fn build_passes_the_system_through_env() -> Result<(), String> {
    let job = made(&picked()?, build)?;
    let want = "nix build --print-build-logs \".#packages.$SYSTEM.actlith\"";
    assert_eq!(commands(&job), [want]);
    let run = job.steps.last().ok_or("no step")?;
    let system = run.env.get("SYSTEM").map(|t| t.render(&GITHUB));
    assert_eq!(system.as_deref(), Some("${{ matrix.system }}"));
    assert_eq!(job.runs_on.render(&GITHUB), "${{ matrix.os }}");
    Ok(())
}

#[test]
fn build_pushes_to_the_cache_only_from_main() -> Result<(), String> {
    let job = made(&picked()?, build)?;
    let want = [
        "actions/checkout",
        "cachix/install-nix-action",
        "cachix/cachix-action",
    ];
    assert_eq!(actions(&job), want);
    let step = job.steps.get(2).ok_or("no cachix")?;
    let Action::Uses(uses) = &step.action else {
        return Err("cachix is not a uses step".into());
    };
    let text = format!("{:?}", uses.with);
    assert!(text.contains(SKIP_UNLESS_MAIN), "{text}");
    Ok(())
}

#[test]
fn check_runs_the_flake_check_then_evaluates_runnerless_systems()
-> Result<(), String> {
    let job = made(&picked()?, check)?;
    let want = [
        "nix flake check --all-systems --print-build-logs",
        "nix eval --raw .#packages.x86_64-darwin.actlith.drvPath",
    ];
    assert_eq!(commands(&job), want);
    assert_eq!(job.runs_on.render(&GITHUB), "ubuntu-latest");
    assert!(job.needs.is_empty() && job.condition.is_none());
    Ok(())
}

#[test]
fn the_eval_only_systems_are_the_ones_without_a_runner() {
    assert_eq!(eval_only(), [System::X86_64Darwin]);
}

#[test]
fn verify_cache_runs_the_seeded_script_after_main_builds() -> Result<(), String>
{
    let job = made(&picked()?, verify)?;
    let want = format!(
        "bash {VERIFY_SCRIPT} pr0d1r2 actlith aarch64-darwin x86_64-linux"
    );
    assert_eq!(commands(&job), [want.as_str()]);
    assert_eq!(job.needs, ["build"]);
    let condition = job.condition.as_ref().map(crate::yaml::Expr::inner);
    let main = "github.ref == 'refs/heads/main' && \
                github.event_name == 'push'";
    assert_eq!(condition, Some(main));
    assert_eq!(job.runs_on.render(&GITHUB), "ubuntu-latest");
    Ok(())
}

#[test]
fn no_job_checks_out_with_credentials() -> Result<(), String> {
    let row = picked()?;
    for part in [build, check, verify] {
        let job = made(&row, part)?;
        let first = job.steps.first().ok_or("no step")?;
        let Action::Uses(uses) = &first.action else {
            return Err("first step is not checkout".into());
        };
        let text = format!("{:?}", uses.with);
        assert!(text.contains("persist-credentials"), "{text}");
        assert!(text.contains(&format!("{:?}", Value::Bool(false))));
        for command in commands(&job) {
            assert!(!command.contains("${{"), "{command}");
        }
    }
    Ok(())
}
