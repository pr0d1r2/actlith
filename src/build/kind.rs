//! Kind `build` (`docs/KINDS.md`): build and cache a flake package on
//! every system, then prove the cache has it (T20, V30).

use super::jobs::{VERIFY_SCRIPT, jobs};
use super::plan::Plan;
use crate::kind::{
    BUILD, CachixPush, Kind, KindError, KindSpec, Row, Secret, on_push_and_pr,
    workflow,
};
use crate::pin::PinTable;
use crate::system::Runners;
use crate::yaml::Workflow;

/// The seeded scripts a `build` workflow runs: `gen` writes them too.
pub(crate) const SCRIPTS: [&str; 1] = [VERIFY_SCRIPT];

/// The `build` kind.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Build;

impl Build {
    /// The seeded scripts its workflows run.
    pub(crate) fn scripts() -> &'static [&'static str] {
        &SCRIPTS
    }
}

impl Kind for Build {
    fn spec(&self) -> &'static KindSpec {
        &BUILD
    }

    /// `CACHIX_AUTH_TOKEN` for the push from `main` (`src/kind:V14`); a
    /// row without a cache is refused, so it reads none.
    fn secrets(&self, row: &Row) -> Vec<Secret> {
        let push = row.cache.as_ref().and(CachixPush::MainOnly.secret());
        push.into_iter().collect()
    }

    fn build(
        &self,
        row: &Row,
        pins: &PinTable,
        runners: &Runners,
    ) -> Result<Workflow, KindError> {
        let plan = Plan::new(row, pins, runners)?;
        Ok(workflow(&row.id, on_push_and_pr(), jobs(&plan)?))
    }
}

#[cfg(test)]
mod tests;
