//! Kind `build` (`docs/KINDS.md`): build and cache a flake package on
//! every system, then prove the cache has it (T20, V30).

use super::jobs::{VERIFY_HTTPS, VERIFY_SCRIPT, jobs};
use super::plan::Plan;
use crate::kind::{
    BUILD, BinaryCache, CacheKind, CachixPush, Checked, Kind, KindError,
    KindSpec, PUSH_SCRIPT, Secret, on_push_and_pr, secrets, workflow,
};
use crate::pin::PinTable;
use crate::system::Runners;
use crate::yaml::Workflow;

/// The `build` kind.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Build;

impl Kind for Build {
    fn spec(&self) -> &'static KindSpec {
        &BUILD
    }

    /// The cache's secrets for the push from `main` (`src/kind:V14`); a
    /// row without a cache is refused, so it reads none.
    fn secrets(&self, row: &Checked) -> Vec<Secret> {
        secrets(row.cache.as_ref(), CachixPush::MainOnly)
    }

    /// `verify-cache.sh` for a cachix cache; for an https one, the push
    /// script and `verify-https.sh` (V64). `gen` writes them too.
    fn scripts(&self, row: &Checked) -> Vec<&'static str> {
        match row.cache.as_ref().map(BinaryCache::kind) {
            Some(CacheKind::Cachix) => vec![VERIFY_SCRIPT],
            Some(_) => vec![PUSH_SCRIPT, VERIFY_HTTPS],
            None => Vec::new(),
        }
    }

    fn build(
        &self,
        row: &Checked,
        pins: &PinTable,
        runners: &Runners,
    ) -> Result<Workflow, KindError> {
        let plan = Plan::new(row, pins, runners)?;
        Ok(workflow(&row.id, on_push_and_pr(), jobs(&plan)?))
    }
}

#[cfg(test)]
mod tests;
