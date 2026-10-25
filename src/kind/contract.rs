//! The `Kind` trait: what a kind is (C). A kind is typed Rust data, never
//! a text template: its pipelith catalogue entry, the secrets it reads,
//! and the [`Workflow`] it builds from a checked [`Row`].

use super::error::KindError;
use super::secret::Secret;
use crate::core::kind::{ArgSpec, KindSpec, Row};
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
    fn secrets(&self, row: &Row) -> Vec<Secret>;

    /// The workflow for `row`, its actions pinned from `pins`, each job
    /// on the host `runners` gives its system (`src/system` V53).
    fn build(
        &self,
        row: &Row,
        pins: &PinTable,
        runners: &Runners,
    ) -> Result<Workflow, KindError>;
}

#[cfg(test)]
mod tests;
