//! Kind `gate` (T19): the hk gate on every system, the shape of pklith's
//! `ci.yml` (C). One job `gate` (stable, `src/kind` V9) per system;
//! checkout keeps full history, cachix pushes except from fork PRs, then
//! the one gate command runs.

use std::collections::BTreeMap;

use crate::kind::{
    ArgSchema, ArgSpec, CachixPush, Checkout, Kind, KindError, Row, Secret,
    TIMEOUT, cachix, checkout, install_nix, job, matrix_runner, on_push_and_pr,
    run, runner_matrix, timeout, workflow,
};
use crate::pin::PinTable;
use crate::system::{DEFAULT_SYSTEMS, System};
use crate::yaml::{Comment, Step, Workflow};

/// The job id and name; `pklith protect` reads it (`src/kind` V9).
const JOB: &str = "gate";

/// The gate run when the row gives no `command`.
const COMMAND: &str = "nix develop --command hk check --all --check";

const SCHEMA: [ArgSpec; 2] = [
    ArgSpec {
        key: "command",
        required: false,
        default: Some(COMMAND),
    },
    TIMEOUT,
];

const DEPTH_WHY: &str = "Full history: tdd-order reads the push range, and \
    a later semver\nstep compares with the last release tag.";

/// The kind `gate`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Gate;

impl Kind for Gate {
    fn name(&self) -> &'static str {
        "gate"
    }

    fn schema(&self) -> &'static ArgSchema {
        &SCHEMA
    }

    fn default_systems(&self) -> &'static [System] {
        &DEFAULT_SYSTEMS
    }

    fn secrets(&self, row: &Row) -> Vec<Secret> {
        let push = row.cache.as_ref().and(CachixPush::SkipForkPr.secret());
        push.into_iter().collect()
    }

    fn build(&self, row: &Row, pins: &PinTable) -> Result<Workflow, KindError> {
        let mut gate =
            job(matrix_runner(), timeout(&row.args)?, steps(row, pins)?);
        gate.name = Some(JOB.to_owned());
        gate.matrix = Some(runner_matrix(row)?);
        let jobs = BTreeMap::from([(JOB.to_owned(), gate)]);
        Ok(workflow(&row.id, on_push_and_pr(), jobs))
    }
}

/// Checkout, install-nix, cachix when the row has a cache, the gate.
fn steps(row: &Row, pins: &PinTable) -> Result<Vec<Step>, KindError> {
    let how = Checkout {
        pushes: false,
        fetch_depth: Some(0),
        why_depth: Some(Comment::new(DEPTH_WHY)),
    };
    let mut steps = vec![
        checkout(pins, &how)?,
        install_nix(pins, row.cache.as_ref())?,
    ];
    if let Some(cache) = &row.cache {
        steps.push(cachix(pins, cache, CachixPush::SkipForkPr)?);
    }
    let command = row.args.get("command").unwrap_or(COMMAND);
    steps.push(run(row.line, command, &[])?);
    Ok(steps)
}

#[cfg(test)]
mod tests;
