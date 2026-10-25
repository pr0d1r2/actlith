//! The `build` jobs (T20, V30, T76): matrix per hosted system, a job per
//! own-runner system behind the fork-PR guard (`src/kind:V54`), push only
//! from `main`, flake check over every system, verify-cache through the
//! seeded script and never inline shell.

use crate::yaml::GITHUB;
use std::collections::BTreeMap;

use super::{VERIFY_SCRIPT, build, check, jobs, matrix, verify};
use crate::build::Build;
use crate::build::plan::Plan;
use crate::kind::{Row, SKIP_UNLESS_MAIN, TRUSTED, matrix_runner};
use crate::pin::PinTable;
use crate::registry::parse;
use crate::system::{Runners, System};
use crate::yaml::{Action, Job, RunsOn, Step, Text, Value};

const ROW: &str = include_str!("../../../tests/fixtures/build/picked.actlith");

fn picked() -> Result<Row, String> {
    let registry = parse(ROW).map_err(|e| e.to_string())?;
    let row = registry.workflows.first().ok_or("no row")?;
    crate::kind::check(&Build, row).map_err(|e| e.to_string())
}

fn made(
    row: &Row,
    part: fn(&Plan) -> Result<Job, crate::kind::KindError>,
) -> Result<Job, String> {
    let pins = PinTable::builtin();
    let runners = Runners::default();
    let plan = Plan::new(row, &pins, &runners).map_err(|e| e.to_string())?;
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
    let runners = Runners::default();
    let plan = Plan::new(&row, &pins, &runners).map_err(|e| e.to_string())?;
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
    let runners = Runners::default();
    let plan = Plan::new(&row, &pins, &runners).map_err(|e| e.to_string())?;
    let got = matrix(&plan);
    let want = [
        pair("macos-latest", "aarch64-darwin"),
        pair("ubuntu-latest", "x86_64-linux"),
    ];
    assert_eq!(got.include, want);
    assert!(!got.fail_fast && got.axes.is_empty());
    Ok(())
}

#[test]
fn x86_64_darwin_builds_on_the_intel_image() -> Result<(), String> {
    let mut row = picked()?;
    row.systems = vec![System::X86_64Darwin];
    let pins = PinTable::builtin();
    let runners = Runners::default();
    let plan = Plan::new(&row, &pins, &runners).map_err(|e| e.to_string())?;
    let got = matrix(&plan);
    assert_eq!(got.include, [pair("macos-15-intel", "x86_64-darwin")]);
    Ok(())
}

fn own_labels() -> Vec<String> {
    ["self-hosted", "macOS", "X64"].map(str::to_owned).to_vec()
}

/// Every job of the picked row on `systems`, under `runners` text.
fn all_jobs(
    systems: Vec<System>,
    rows: &str,
) -> Result<BTreeMap<String, Job>, String> {
    let mut row = picked()?;
    row.systems = systems;
    let text = format!("format 1\n\n## runners\nsystem|labels\n{rows}");
    let own = parse(&text).map_err(|e| e.to_string())?.runners;
    let pins = PinTable::builtin();
    let plan = Plan::new(&row, &pins, &own).map_err(|e| e.to_string())?;
    jobs(&plan).map_err(|e| e.to_string())
}

const INTEL: &str = "x86_64-darwin|self-hosted, macOS, X64\n";

fn get<'a>(
    all: &'a BTreeMap<String, Job>,
    id: &str,
) -> Result<&'a Job, String> {
    all.get(id).ok_or_else(|| format!("no job {id}"))
}

fn inner(job: &Job) -> Option<&str> {
    job.condition.as_ref().map(crate::yaml::Expr::inner)
}

#[test]
fn an_own_runner_system_builds_in_a_job_of_its_own() -> Result<(), String> {
    let all = all_jobs(vec![System::X86_64Linux, System::X86_64Darwin], INTEL)?;
    let ids: Vec<&str> = all.keys().map(String::as_str).collect();
    assert_eq!(
        ids,
        ["build", "build-x86_64-darwin", "check", "verify-cache"]
    );
    let hosted = get(&all, "build")?;
    let matrix = hosted.matrix.as_ref().ok_or("no matrix")?;
    assert_eq!(matrix.include, [pair("ubuntu-latest", "x86_64-linux")]);
    assert_eq!(hosted.condition, None);
    let verified = get(&all, "verify-cache")?;
    assert_eq!(verified.needs, ["build", "build-x86_64-darwin"]);
    Ok(())
}

#[test]
fn the_own_build_job_runs_the_same_steps_guarded() -> Result<(), String> {
    let all = all_jobs(vec![System::X86_64Linux, System::X86_64Darwin], INTEL)?;
    let mine = get(&all, "build-x86_64-darwin")?;
    assert_eq!(mine.name.as_deref(), Some("build"));
    assert_eq!(mine.runs_on, RunsOn::Labels(own_labels()));
    let only = mine.matrix.as_ref().ok_or("no matrix")?;
    let system = BTreeMap::from([("system".into(), "x86_64-darwin".into())]);
    assert_eq!(only.include, [system]);
    assert_eq!(inner(mine), Some(TRUSTED));
    assert_eq!(mine.steps, get(&all, "build")?.steps);
    Ok(())
}

#[test]
fn only_own_runner_systems_leave_no_matrix_job() -> Result<(), String> {
    let all = all_jobs(vec![System::X86_64Darwin], INTEL)?;
    let ids: Vec<&str> = all.keys().map(String::as_str).collect();
    assert_eq!(ids, ["build-x86_64-darwin", "check", "verify-cache"]);
    assert_eq!(get(&all, "verify-cache")?.needs, ["build-x86_64-darwin"]);
    Ok(())
}

const LINUX: &str = "x86_64-linux|self-hosted, linux\n";

#[test]
fn an_own_linux_runner_takes_check_and_verify_cache() -> Result<(), String> {
    let all = all_jobs(vec![System::X86_64Linux], LINUX)?;
    let labels = ["self-hosted", "linux"].map(str::to_owned).to_vec();
    let checked = get(&all, "check")?;
    assert_eq!(checked.runs_on, RunsOn::Labels(labels.clone()));
    assert_eq!(inner(checked), Some(TRUSTED));
    let verified = get(&all, "verify-cache")?;
    assert_eq!(verified.runs_on, RunsOn::Labels(labels));
    let both = format!(
        "({TRUSTED}) && (github.ref == 'refs/heads/main' && \
         github.event_name == 'push')"
    );
    assert_eq!(inner(verified), Some(both.as_str()));
    Ok(())
}

#[test]
fn build_passes_the_system_through_env() -> Result<(), String> {
    let job = made(&picked()?, build)?;
    let want = "set -o pipefail; if [ \"$GITHUB_EVENT_NAME\" = push ] && [ \"$GITHUB_REF\" = refs/heads/main ]; then nix build --print-out-paths --print-build-logs \".#packages.$SYSTEM.actlith\" | cachix push pr0d1r2; else nix build --print-build-logs \".#packages.$SYSTEM.actlith\"; fi";
    assert_eq!(commands(&job), [want]);
    let run = job.steps.last().ok_or("no step")?;
    let system = run.env.get("SYSTEM").map(|t| t.render(&GITHUB));
    assert_eq!(system.as_deref(), Some("${{ matrix.system }}"));
    assert_eq!(job.runs_on, RunsOn::One(matrix_runner()));
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
    assert_explicit_push(step)
}

fn assert_explicit_push(step: &Step) -> Result<(), String> {
    let Action::Uses(uses) = &step.action else {
        return Err("cachix is not a uses step".into());
    };
    let text = format!("{:?}", uses.with);
    assert!(text.contains("skipPush"), "{text}");
    assert!(
        !text.contains(SKIP_UNLESS_MAIN) && !text.contains("pushFilter"),
        "{text}"
    );
    Ok(())
}

#[test]
fn check_runs_the_flake_check_over_every_system() -> Result<(), String> {
    let job = made(&picked()?, check)?;
    let want = ["nix flake check --all-systems --print-build-logs"];
    assert_eq!(commands(&job), want);
    assert_eq!(job.runs_on, RunsOn::One(Text::lit("ubuntu-latest")));
    assert!(job.needs.is_empty() && job.condition.is_none());
    Ok(())
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
    assert_eq!(job.runs_on, RunsOn::One(Text::lit("ubuntu-latest")));
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
