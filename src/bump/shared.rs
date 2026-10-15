//! What both bump kinds share: token, runner, pushing checkout, job
//! permissions. RED stub.

use crate::kind::{ArgSpec, Args, KindError, Row, Secret};
use crate::pin::PinTable;
use crate::system::System;
use crate::yaml::{Job, Step, Text};

/// The secret a row means when it gives no `token`.
pub(crate) const GITHUB_TOKEN: &str = "GITHUB_TOKEN";

/// `token`, the secret that pushes the branch and opens the PR (R5).
pub(crate) const TOKEN: ArgSpec = ArgSpec {
    key: "token",
    required: false,
    default: Some(GITHUB_TOKEN),
};

/// A bump kind runs one job on one Linux runner.
pub(crate) const SYSTEMS: [System; 1] = [System::X86_64Linux];

fn stub(line: usize) -> KindError {
    KindError::row(line, "stub", "stub")
}

/// The secret the row's `token` names.
pub(crate) fn token(args: &Args) -> Result<Secret, KindError> {
    Err(stub(args.line()))
}

/// The runner label of the row's one system.
pub(crate) fn runs_on(row: &Row) -> Result<Text, KindError> {
    Err(stub(row.line))
}

/// The checkout of a job that pushes.
pub(crate) fn pushing_checkout(
    _pins: &PinTable,
    _token: &Secret,
) -> Result<Step, KindError> {
    Err(stub(0))
}

/// The one bump job.
pub(crate) fn bump_job(row: &Row, _steps: Vec<Step>) -> Result<Job, KindError> {
    Err(stub(row.line))
}

/// The row's `key` as a plain word.
pub(crate) fn plain<'a>(
    args: &'a Args,
    _key: &str,
) -> Result<&'a str, KindError> {
    Err(stub(args.line()))
}

#[cfg(test)]
mod tests;
