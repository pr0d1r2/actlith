//! The three `build` jobs (`src/build` I, nix-hk `build.yml`): `build`
//! per system, `check` over every declared system, `verify-cache` asking
//! the cache for every built path (V30). Job ids are stable
//! (`src/kind:V9`); every value GitHub evaluates reaches a command
//! through `env:` (`src/kind:V3`).

use std::collections::BTreeMap;

use super::plan::Plan;
use crate::kind::KindError;
use crate::system::System;
use crate::yaml::{Job, Matrix};

/// The seeded script the `verify-cache` job runs (V30).
pub(crate) const VERIFY_SCRIPT: &str = "scripts/actlith/verify-cache.sh";

/// Every job, by id.
pub(crate) fn jobs(plan: &Plan) -> Result<BTreeMap<String, Job>, KindError> {
    let _ = plan.pins;
    Err(KindError::row(plan.row.line, "stub", "stub"))
}

/// `build`: one runner per system, cachix pushing only from `main`.
pub(crate) fn build(plan: &Plan) -> Result<Job, KindError> {
    Err(KindError::row(plan.row.line, "stub", "stub"))
}

/// `check`: `nix flake check --all-systems`, then an eval of the package
/// on each system with no runner.
pub(crate) fn check(plan: &Plan) -> Result<Job, KindError> {
    Err(KindError::row(plan.row.line, "stub", "stub"))
}

/// `verify-cache`: after `build`, on a push to `main` only, the seeded
/// script asks the cache for every built system's path (V30).
pub(crate) fn verify(plan: &Plan) -> Result<Job, KindError> {
    Err(KindError::row(plan.row.line, "stub", "stub"))
}

/// The `build` matrix: an `os` and `system` pair per row system.
pub(crate) fn matrix(plan: &Plan) -> Result<Matrix, KindError> {
    Err(KindError::row(plan.row.line, "stub", "stub"))
}

/// Systems actlith knows that have no hosted runner: evaluated only.
pub(crate) fn eval_only() -> Vec<System> {
    Vec::new()
}

#[cfg(test)]
mod tests;
