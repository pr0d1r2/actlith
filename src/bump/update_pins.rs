//! Kind `update-pins` (T22): follow `nixpkgs-lock` and propose the bump
//! as a PR, with `actlith pin` and `actlith gen` run in the same branch.
//! Shape: nix-hk `update-pins.yml` (`.:R3`); the logic is the seeded
//! `update-pins.sh` (`src/script:V13`), idempotent per V26.

use std::collections::BTreeMap;

use super::cron::cron;
use super::shared::{LIMITS, bump_job, plain, pushing_checkout, token};
use crate::kind::{
    Kind, KindError, KindSpec, Row, Secret, UPDATE_PINS, install_nix,
    on_schedule, run, workflow,
};
use crate::pin::PinTable;
use crate::system::Runners;
use crate::yaml::{Comment, Step, Text, Workflow};

const WHY: &str = "Follow nixpkgs-lock and propose the bump as a PR. Pull \
    model: this repo\npolls; nixpkgs-lock holds no cross-repo token. \
    `actlith pin` and\n`actlith gen` run in the same branch, so action pins \
    and a new actlith\narrive through the same reviewed PR.";

/// The dev shell brings `actlith`; `gh` and `git` come from the runner.
const COMMAND: &str =
    "nix develop --command bash scripts/actlith/update-pins.sh \"$BUILD\"";

/// Kind `update-pins`.
pub(crate) struct UpdatePins;

impl Kind for UpdatePins {
    fn spec(&self) -> &'static KindSpec {
        &UPDATE_PINS
    }

    fn secrets(&self, row: &Row) -> Vec<Secret> {
        token(&row.args).into_iter().collect()
    }

    fn build(
        &self,
        row: &Row,
        pins: &PinTable,
        runners: &Runners,
    ) -> Result<Workflow, KindError> {
        let update = bump_job(row, runners, steps(row, pins)?)?;
        let jobs = BTreeMap::from([("update".to_owned(), update)]);
        let mut built = workflow(&row.id, on_schedule(&cron(&row.args)?), jobs);
        built.comments = vec![Comment::new(&format!("{WHY}\n\n{LIMITS}"))];
        Ok(built)
    }
}

fn steps(row: &Row, pins: &PinTable) -> Result<Vec<Step>, KindError> {
    let token = token(&row.args)?;
    let build = plain(&row.args, "build")?;
    let mut update = run(row.line, COMMAND, &[("GH_TOKEN", token.expr())])?;
    update.env.insert("BUILD".to_owned(), Text::lit(build));
    Ok(vec![
        pushing_checkout(pins, &token)?,
        install_nix(pins, row.cache.as_ref())?,
        update,
    ])
}

#[cfg(test)]
mod tests;
