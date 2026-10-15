//! Job-level hardening (V3): every job has `timeout-minutes`, and a job
//! widens permissions only by the scopes it declares.

use std::collections::BTreeMap;

use super::args::Args;
use super::error::KindError;
use super::row::Row;
use crate::system::runner;
use crate::yaml::{
    Axis, AxisName, Comment, Env, Expr, Job, Level, Matrix, Permissions, Scope,
    Step, Text,
};

/// Minutes a job runs when the row gives no `timeout`.
pub(crate) const DEFAULT_TIMEOUT: u32 = 30;

/// The longest job GitHub-hosted runners allow, in minutes.
pub(crate) const MAX_TIMEOUT: u32 = 360;

/// A job on `runs_on` with `steps`, stopped after `timeout` minutes.
pub(crate) fn job(runs_on: Text, timeout: u32, steps: Vec<Step>) -> Job {
    Job {
        comments: Vec::new(),
        name: None,
        needs: Vec::new(),
        condition: None,
        runs_on,
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

/// The row's `timeout` arg, or [`DEFAULT_TIMEOUT`]; refused outside
/// 1 to [`MAX_TIMEOUT`].
pub(crate) fn timeout(args: &Args) -> Result<u32, KindError> {
    let minutes = args.number("timeout")?.unwrap_or(DEFAULT_TIMEOUT);
    if (1..=MAX_TIMEOUT).contains(&minutes) {
        return Ok(minutes);
    }
    Err(KindError::row(
        args.line(),
        format!("timeout `{minutes}` is not 1 to {MAX_TIMEOUT} minutes"),
        format!("write `timeout=<1..{MAX_TIMEOUT}>`"),
    ))
}

/// `fail-fast: false` over an `os` axis of the row's runner labels, the
/// systems named in a comment above it.
pub(crate) fn runner_matrix(row: &Row) -> Result<Matrix, KindError> {
    let systems: Vec<String> =
        row.systems.iter().map(ToString::to_string).collect();
    let os = Axis {
        comments: vec![Comment::new(&systems.join(", "))],
        values: labels(row)?,
    };
    let axes = AxisName::new("os").into_iter().zip([os]).collect();
    Ok(Matrix {
        fail_fast: false,
        axes,
        include: Vec::new(),
    })
}

fn labels(row: &Row) -> Result<Vec<String>, KindError> {
    let label = |system| {
        runner(system).map(str::to_owned).map_err(|e| {
            KindError::row(row.line, e.to_string(), "remove the system")
        })
    };
    row.systems.iter().copied().map(label).collect()
}

/// `${{ matrix.os }}`, the `runs-on` of a [`runner_matrix`] job.
pub(crate) fn matrix_runner() -> Text {
    Expr::new("matrix.os")
        .into_iter()
        .fold(Text::default(), Text::then_expr)
}

#[cfg(test)]
mod tests;
