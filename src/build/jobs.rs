//! The `build` jobs (`src/build` I, nix-hk `build.yml`): `build` per
//! hosted system and `build-<system>` per own-runner one (T76), `check`
//! over every declared system, `verify-cache` asking the cache for every
//! built path (V30). Job ids are stable (`src/kind:V9`); every value
//! GitHub evaluates reaches a command through `env:` (`src/kind:V3`).

use crate::yaml::GITHUB;
use std::collections::BTreeMap;

use super::plan::Plan;
use crate::kind::{
    CachixPush, Checkout, KindError, cachix, checkout, hosts, install_nix, job,
    matrix_runner, own, place, run,
};
use crate::system::{Host, System, host};
use crate::yaml::{Comment, Expr, Job, Matrix, Step, Text};

/// The seeded script the `verify-cache` job runs (V30).
pub(crate) const VERIFY_SCRIPT: &str = "scripts/actlith/verify-cache.sh";

/// `verify-cache` runs on a push to `main` only.
const MAIN_PUSH: &str =
    "github.ref == 'refs/heads/main' && github.event_name == 'push'";

const BUILD_WHY: &str = "One native runner per system: a cross-built path \
    is a different store\npath, never the one consumers substitute.";

const CHECK_WHY: &str = "--all-systems is the point: a bare `nix flake \
    check` skips every system\nthe runner cannot build.";

const VERIFY_WHY: &str = "A failed cachix push does not fail its job, so \
    this asks the cache\nitself for every built path, after the builds' \
    post-job pushes.";

/// Every job, by id: `build` over the hosted systems, `build-<system>`
/// per system on the repository's own runner (`src/kind:V54`), `check`
/// and `verify-cache`.
pub(crate) fn jobs(plan: &Plan) -> Result<BTreeMap<String, Job>, KindError> {
    let hosts = hosts(plan.row, plan.runners);
    let mut all = BTreeMap::new();
    if !hosts.hosted.is_empty() {
        all.insert("build".to_owned(), build(plan)?);
    }
    for &(system, labels) in &hosts.own {
        all.insert(format!("build-{system}"), mine(plan, system, labels)?);
    }
    all.insert("check".to_owned(), check(plan)?);
    all.insert("verify-cache".to_owned(), verify(plan)?);
    Ok(all)
}

/// `build`: one hosted runner per system, cachix pushing only from
/// `main`.
pub(crate) fn build(plan: &Plan) -> Result<Job, KindError> {
    Ok(Job {
        runs_on: matrix_runner().into(),
        matrix: Some(matrix(plan)),
        ..unplaced(plan)?
    })
}

/// `build-<system>`, named `build` so its check reads `build (<system>)`
/// as the matrix's do: the same steps over a one-entry `include`, on the
/// repository's own runner, guarded. A job of its own, since a job's
/// `if:` cannot see the matrix (`src/kind:R19`).
fn mine(
    plan: &Plan,
    system: System,
    labels: &[String],
) -> Result<Job, KindError> {
    let only = BTreeMap::from([("system".to_owned(), system.to_string())]);
    let one = Matrix {
        fail_fast: false,
        axes: BTreeMap::new(),
        include: vec![only],
    };
    let job = Job {
        matrix: Some(one),
        ..unplaced(plan)?
    };
    Ok(own(job, labels))
}

/// The build job before it is given a runner.
fn unplaced(plan: &Plan) -> Result<Job, KindError> {
    let command = build_command(plan);
    let env: Vec<_> = Expr::new(&GITHUB, "matrix.system")
        .into_iter()
        .map(|e| ("SYSTEM", e))
        .collect();
    let mut steps = setup(plan)?;
    steps.push(cachix(plan.pins, plan.cache, CachixPush::ExplicitMain)?);
    steps.push(run(plan.row.line, &command, &env)?);
    let mut built = named("build", plan, steps);
    built.comments = vec![Comment::new(BUILD_WHY)];
    Ok(built)
}

fn build_command(plan: &Plan) -> String {
    format!(
        "set -o pipefail; if [ \"$GITHUB_EVENT_NAME\" = push ] && [ \"$GITHUB_REF\" = refs/heads/main ]; then nix build --print-out-paths --print-build-logs \".#packages.$SYSTEM.{}\" | cachix push {}; else nix build --print-build-logs \".#packages.$SYSTEM.{}\"; fi",
        plan.package,
        plan.cache.name(),
        plan.package
    )
}

/// `check`: `nix flake check --all-systems`, on `x86_64-linux`'s host.
pub(crate) fn check(plan: &Plan) -> Result<Job, KindError> {
    let mut steps = setup(plan)?;
    let flake = "nix flake check --all-systems --print-build-logs";
    steps.push(run(plan.row.line, flake, &[])?);
    let mut checked = named("check", plan, steps);
    checked.comments = vec![Comment::new(CHECK_WHY)];
    Ok(place(checked, linux(plan)))
}

/// `verify-cache`: after every build job, on a push to `main` only, the
/// seeded script asks the cache for every built system's path (V30).
pub(crate) fn verify(plan: &Plan) -> Result<Job, KindError> {
    let systems: Vec<String> =
        plan.row.systems.iter().map(ToString::to_string).collect();
    let (cache, package) = (plan.cache.name(), plan.package);
    let command = format!(
        "bash {VERIFY_SCRIPT} {cache} {package} {}",
        systems.join(" ")
    );
    let mut steps = setup(plan)?;
    steps.push(run(plan.row.line, &command, &[])?);
    let mut verified = named("verify-cache", plan, steps);
    verified.comments = vec![Comment::new(VERIFY_WHY)];
    verified.needs = built(plan);
    verified.condition = Expr::new(&GITHUB, MAIN_PUSH);
    Ok(place(verified, linux(plan)))
}

/// The `build` matrix: an `os` and `system` pair per hosted system.
pub(crate) fn matrix(plan: &Plan) -> Matrix {
    let pair = |&(system, os): &(System, &str)| {
        BTreeMap::from([
            ("os".to_owned(), os.to_owned()),
            ("system".to_owned(), system.to_string()),
        ])
    };
    let hosted = hosts(plan.row, plan.runners).hosted;
    Matrix {
        fail_fast: false,
        axes: BTreeMap::new(),
        include: hosted.iter().map(pair).collect(),
    }
}

/// The ids of the build jobs, in id order: `build` when a system is
/// hosted, then `build-<system>` per own-runner system.
fn built(plan: &Plan) -> Vec<String> {
    let hosts = hosts(plan.row, plan.runners);
    let hosted = (!hosts.hosted.is_empty()).then(|| "build".to_owned());
    let own = hosts
        .own
        .iter()
        .map(|(system, _)| format!("build-{system}"));
    hosted.into_iter().chain(own).collect()
}

/// Checkout without credentials, then install-nix trusting the cache.
fn setup(plan: &Plan) -> Result<Vec<Step>, KindError> {
    Ok(vec![
        checkout(plan.pins, &Checkout::default())?,
        install_nix(plan.pins, Some(plan.cache))?,
    ])
}

/// Job `id`, named `id` (`src/kind:V9`), before it is given a runner.
fn named(id: &str, plan: &Plan, steps: Vec<Step>) -> Job {
    let mut made = job(Text::default(), plan.minutes, steps);
    made.name = Some(id.to_owned());
    made
}

/// The host of the single-runner jobs: `x86_64-linux`'s.
fn linux<'a>(plan: &Plan<'a>) -> Host<'a> {
    host(plan.runners, System::X86_64Linux)
}

#[cfg(test)]
mod tests;
