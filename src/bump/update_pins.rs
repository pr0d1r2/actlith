//! Kind `update-pins` (T22). RED stub.

use crate::kind::{ArgSchema, Kind, KindError, Row, Secret};
use crate::pin::PinTable;
use crate::system::System;
use crate::yaml::Workflow;

/// Kind `update-pins`.
pub(crate) struct UpdatePins;

impl Kind for UpdatePins {
    fn name(&self) -> &'static str {
        ""
    }

    fn schema(&self) -> &'static ArgSchema {
        &[]
    }

    fn default_systems(&self) -> &'static [System] {
        &[]
    }

    fn secrets(&self, _row: &Row) -> Vec<Secret> {
        Vec::new()
    }

    fn build(
        &self,
        row: &Row,
        _pins: &PinTable,
    ) -> Result<Workflow, KindError> {
        Err(KindError::row(row.line, "stub", "stub"))
    }
}

#[cfg(test)]
mod tests;
