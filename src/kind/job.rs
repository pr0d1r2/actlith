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
    let _ = need;
    Permissions::default()
}

/// The row's `timeout` arg, or [`DEFAULT_TIMEOUT`]; refused outside
/// 1 to [`MAX_TIMEOUT`].
pub(crate) fn timeout(args: &Args) -> Result<u32, KindError> {
    Err(KindError::row(args.line(), "stub", "stub"))
}

/// `fail-fast: false` over an `os` axis of the row's runner labels, the
/// systems named in a comment above it.
pub(crate) fn runner_matrix(row: &Row) -> Result<Matrix, KindError> {
    Err(KindError::row(row.line, "stub", "stub"))
}

/// `${{ matrix.os }}`, the `runs-on` of a [`runner_matrix`] job.
pub(crate) fn matrix_runner() -> Text {
    Text::default()
}

#[cfg(test)]
mod tests;
