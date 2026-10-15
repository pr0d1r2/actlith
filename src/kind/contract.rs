//! The `Kind` trait: what a kind is (C). A kind is typed Rust data, never
//! a text template; it builds a [`Workflow`] from a checked [`Row`].

use super::args::ArgSchema;
use super::error::KindError;
use super::row::Row;
use super::secret::Secret;
use crate::pin::PinTable;
use crate::system::System;
use crate::yaml::Workflow;

/// A workflow kind (`docs/KINDS.md`).
pub(crate) trait Kind {
    /// The name a `## workflows` row writes in its `kind` cell.
    fn name(&self) -> &'static str;

    /// Every arg the kind takes; any other key is refused.
    fn schema(&self) -> &'static ArgSchema;

    /// The systems a row's `-` means.
    fn default_systems(&self) -> &'static [System];

    /// Every secret the workflow built from `row` reads (V14).
    fn secrets(&self, row: &Row) -> Vec<Secret>;

    /// The workflow for `row`, its actions pinned from `pins`.
    fn build(&self, row: &Row, pins: &PinTable) -> Result<Workflow, KindError>;
}
