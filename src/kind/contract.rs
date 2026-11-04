//! The `Kind` trait: what a kind is (C). A kind is typed Rust data, never
//! a text template: its pipelith catalogue entry, the secrets it reads,
//! and the [`Workflow`] it builds from a [`Checked`] row.

use super::error::KindError;
use super::row::Checked;
use super::secret::Secret;
use crate::core::kind::{ArgSpec, KindSpec};
use crate::pin::PinTable;
use crate::system::Runners;
use crate::yaml::Workflow;

/// A workflow kind (`docs/KINDS.md`).
pub(crate) trait Kind {
    /// The kind's pipelith catalogue entry (V49): its name, every arg it
    /// takes (any other key is refused) and the systems a row's `-` means.
    fn spec(&self) -> &'static KindSpec;

    /// The GitHub-only arg keys the kind takes beyond its catalogue
    /// schema (V49): each one of `EXTRAS.keys`. None by default, so any
    /// other kind refuses them as unknown.
    fn forge_args(&self) -> &'static [ArgSpec] {
        &[]
    }

    /// The name a `## workflows` row writes in its `kind` cell.
    fn name(&self) -> &'static str {
        self.spec().name
    }

    /// Every secret the workflow built from `row` reads (V14).
    fn secrets(&self, row: &Checked) -> Vec<Secret>;

    /// The seeded scripts the workflow built from `row` runs beyond the
    /// ones its kind always runs: those its cache needs (`src/kind/cache`).
    /// `render` adds the read netrc one for every kind itself.
    fn scripts(&self, _row: &Checked) -> Vec<&'static str> {
        Vec::new()
    }

    /// The workflow for `row`, its actions pinned from `pins`, each job
    /// on the host `runners` gives its system (`src/system` V53).
    fn build(
        &self,
        row: &Checked,
        pins: &PinTable,
        runners: &Runners,
    ) -> Result<Workflow, KindError>;
}

#[cfg(test)]
mod tests;
