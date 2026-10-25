//! Job-level hardening (V3): every job has `timeout-minutes`, a job
//! widens permissions only by the scopes it declares, and a job on the
//! repository's own runner runs only for trusted events (V54).

use crate::yaml::GITHUB;
use std::collections::BTreeMap;

use crate::core::kind::Row;
use crate::system::{Host, Runners, System, host};
use crate::yaml::{
    Axis, AxisName, Comment, Env, Expr, Job, Level, Matrix, Permissions,
    RunsOn, Scope, Step, Text,
};

/// A job on `runs_on` with `steps`, stopped after `timeout` minutes.
pub(crate) fn job(runs_on: Text, timeout: u32, steps: Vec<Step>) -> Job {
    Job {
        comments: Vec::new(),
        name: None,
        needs: Vec::new(),
        condition: None,
        runs_on: runs_on.into(),
        permissions: None,
        matrix: None,
        timeout_minutes: timeout,
        env: Env::new(),
        steps,
    }
}

/// Job permissions: `contents: read` plus exactly the scopes in `need`
/// (a job's `permissions:` replaces the top-level one, so the read the
/// checkout needs is kept unless `need` sets `contents` itself).
pub(crate) fn widen(need: &[(Scope, Level)]) -> Permissions {
    let mut scopes = BTreeMap::from([(Scope::Contents, Level::Read)]);
    scopes.extend(need.iter().copied());
    Permissions(scopes)
}

/// The fork-PR guard (V54): a job on the repository's own runner runs
/// for a push, a schedule, a manual run or a pull request from a branch
/// of this repository, never for a fork's pull request, whose code would
/// otherwise run on the owner's machine (pipelith `Runner::trusted_only`).
pub(crate) const TRUSTED: &str = "github.event_name != 'pull_request' || \
    github.event.pull_request.head.repo.full_name == github.repository";

/// A checked row's systems by where they run, each in row order: the
/// GitHub-hosted ones with their label, and those `## runners` puts on
/// the repository's own runner, with its labels.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Hosts<'a> {
    pub(crate) hosted: Vec<(System, &'static str)>,
    pub(crate) own: Vec<(System, &'a [String])>,
}

/// The row's systems split by host (`src/system` V53).
pub(crate) fn hosts<'a>(row: &Row, runners: &'a Runners) -> Hosts<'a> {
    let mut split = Hosts::default();
    for &system in &row.systems {
        match host(runners, system) {
            Host::Hosted(label) => split.hosted.push((system, label)),
            Host::Own(labels) => split.own.push((system, labels)),
        }
    }
    split
}

/// `fail-fast: false` over an `os` axis of the `hosted` runner labels,
/// the systems named in a comment above it.
pub(crate) fn runner_matrix(hosted: &[(System, &str)]) -> Matrix {
    let systems: Vec<String> =
        hosted.iter().map(|(s, _)| s.to_string()).collect();
    let os = Axis {
        comments: vec![Comment::new(&systems.join(", "))],
        values: hosted.iter().map(|&(_, l)| l.to_owned()).collect(),
    };
    let axes = AxisName::new("os").into_iter().zip([os]).collect();
    Matrix {
        fail_fast: false,
        axes,
        include: Vec::new(),
    }
}

/// `job` on `host`: its hosted label, or the repository's own runner
/// ([`own`]).
pub(crate) fn place(job: Job, host: Host<'_>) -> Job {
    match host {
        Host::Hosted(label) => Job {
            runs_on: RunsOn::One(Text::lit(label)),
            ..job
        },
        Host::Own(labels) => own(job, labels),
    }
}

/// `job` on the repository's own runner (V54): `runs-on` every one of
/// its labels, and only for trusted events: its `if:` is [`TRUSTED`],
/// joined to the `if:` the job already has.
pub(crate) fn own(job: Job, labels: &[String]) -> Job {
    let guard = match &job.condition {
        None => TRUSTED.to_owned(),
        Some(own) => format!("({TRUSTED}) && ({})", own.inner()),
    };
    Job {
        runs_on: RunsOn::Labels(labels.to_vec()),
        condition: Expr::new(&GITHUB, &guard),
        ..job
    }
}

/// `${{ matrix.os }}`, the `runs-on` of a [`runner_matrix`] job.
pub(crate) fn matrix_runner() -> Text {
    Expr::new(&GITHUB, "matrix.os")
        .into_iter()
        .fold(Text::default(), Text::then_expr)
}

#[cfg(test)]
mod tests;
