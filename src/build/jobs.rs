//! The three `build` jobs (`src/build` I, nix-hk `build.yml`): `build`
//! per system, `check` over every declared system, `verify-cache` asking
//! the cache for every built path (V30). Job ids are stable
//! (`src/kind:V9`); every value GitHub evaluates reaches a command
//! through `env:` (`src/kind:V3`).

use crate::yaml::GITHUB;
use std::collections::BTreeMap;

use super::plan::Plan;
use crate::kind::{
    CachixPush, Checkout, KindError, cachix, checkout, install_nix, job,
    matrix_runner, run,
};
use crate::system::{System, runner};
use crate::yaml::{Comment, Expr, Job, Matrix, Step, Text};

/// The seeded script the `verify-cache` job runs (V30).
pub(crate) const VERIFY_SCRIPT: &str = "scripts/actlith/verify-cache.sh";

/// `verify-cache` runs on a push to `main` only.
const MAIN_PUSH: &str =
    "github.ref == 'refs/heads/main' && github.event_name == 'push'";

/// Every system actlith knows, runner or not.
const KNOWN: [System; 4] = [
    System::X86_64Linux,
    System::Aarch64Linux,
    System::Aarch64Darwin,
    System::X86_64Darwin,
];

const BUILD_WHY: &str = "One native runner per system: a cross-built path \
    is a different store\npath, never the one consumers substitute.";

const CHECK_WHY: &str = "--all-systems is the point: a bare `nix flake \
    check` skips every system\nthe runner cannot build. Systems without a \
    runner are evaluated, so\nthey cannot rot silently.";

const VERIFY_WHY: &str = "A failed cachix push does not fail its job, so \
    this asks the cache\nitself for every built path, after the builds' \
    post-job pushes.";

/// Every job, by id.
pub(crate) fn jobs(plan: &Plan) -> Result<BTreeMap<String, Job>, KindError> {
    Ok(BTreeMap::from([
        ("build".to_owned(), build(plan)?),
        ("check".to_owned(), check(plan)?),
        ("verify-cache".to_owned(), verify(plan)?),
    ]))
}

/// `build`: one runner per system, cachix pushing only from `main`.
pub(crate) fn build(plan: &Plan) -> Result<Job, KindError> {
    let command = format!(
        "nix build --print-build-logs \".#packages.$SYSTEM.{}\"",
        plan.package
    );
    let env: Vec<_> = Expr::new(&GITHUB, "matrix.system")
        .into_iter()
        .map(|e| ("SYSTEM", e))
        .collect();
    let mut steps = setup(plan)?;
    steps.push(cachix(plan.pins, plan.cache, CachixPush::MainOnly)?);
    steps.push(run(plan.row.line, &command, &env)?);
    let mut built = named("build", matrix_runner(), plan, steps);
    built.comments = vec![Comment::new(BUILD_WHY)];
    built.matrix = Some(matrix(plan)?);
    Ok(built)
}

/// `check`: `nix flake check --all-systems`, then an eval of the package
/// on each system with no runner.
pub(crate) fn check(plan: &Plan) -> Result<Job, KindError> {
    let line = plan.row.line;
    let mut steps = setup(plan)?;
    let flake = "nix flake check --all-systems --print-build-logs";
    steps.push(run(line, flake, &[])?);
    for system in eval_only() {
        let package = plan.package;
        let eval =
            format!("nix eval --raw .#packages.{system}.{package}.drvPath");
        steps.push(run(line, &eval, &[])?);
    }
    let mut checked = named("check", linux(line)?, plan, steps);
    checked.comments = vec![Comment::new(CHECK_WHY)];
    Ok(checked)
}

/// `verify-cache`: after `build`, on a push to `main` only, the seeded
/// script asks the cache for every built system's path (V30).
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
    let mut verified =
        named("verify-cache", linux(plan.row.line)?, plan, steps);
    verified.comments = vec![Comment::new(VERIFY_WHY)];
    verified.needs = vec!["build".to_owned()];
    verified.condition = Expr::new(&GITHUB, MAIN_PUSH);
    Ok(verified)
}

/// The `build` matrix: an `os` and `system` pair per row system.
pub(crate) fn matrix(plan: &Plan) -> Result<Matrix, KindError> {
    let line = plan.row.line;
    let pair = |system: &System| -> Result<_, KindError> {
        let os = label(line, *system)?;
        Ok(BTreeMap::from([
            ("os".to_owned(), os),
            ("system".to_owned(), system.to_string()),
        ]))
    };
    let include = plan.row.systems.iter().map(pair);
    Ok(Matrix {
        fail_fast: false,
        axes: BTreeMap::new(),
        include: include.collect::<Result<_, _>>()?,
    })
}

/// Systems actlith knows that have no hosted runner: evaluated only.
pub(crate) fn eval_only() -> Vec<System> {
    KNOWN.into_iter().filter(|&s| runner(s).is_err()).collect()
}

/// Checkout without credentials, then install-nix trusting the cache.
fn setup(plan: &Plan) -> Result<Vec<Step>, KindError> {
    Ok(vec![
        checkout(plan.pins, &Checkout::default())?,
        install_nix(plan.pins, Some(plan.cache))?,
    ])
}

/// Job `id`, named `id` (`src/kind:V9`), on `runs_on`.
fn named(id: &str, runs_on: Text, plan: &Plan, steps: Vec<Step>) -> Job {
    let mut made = job(runs_on, plan.minutes, steps);
    made.name = Some(id.to_owned());
    made
}

/// The runner of the single-runner jobs: the `x86_64-linux` one.
fn linux(line: usize) -> Result<Text, KindError> {
    label(line, System::X86_64Linux).map(|l| Text::lit(&l))
}

fn label(line: usize, system: System) -> Result<String, KindError> {
    runner(system)
        .map(str::to_owned)
        .map_err(|e| KindError::row(line, e.to_string(), "remove the system"))
}

#[cfg(test)]
mod tests;
