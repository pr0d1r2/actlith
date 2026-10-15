//! Kind `gate` (T19): the hk gate on every system, the shape of pklith's
//! `ci.yml` (C). One job `gate` (stable, `src/kind` V9) per system.

use std::collections::BTreeMap;

use crate::kind::{
    ArgSchema, ArgSpec, Kind, KindError, Row, Secret, TIMEOUT, on_push_and_pr,
    workflow,
};
use crate::pin::PinTable;
use crate::system::System;
use crate::yaml::Workflow;

const SCHEMA: [ArgSpec; 1] = [TIMEOUT];

/// The kind `gate`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Gate;

impl Kind for Gate {
    fn name(&self) -> &'static str {
        ""
    }

    fn schema(&self) -> &'static ArgSchema {
        &SCHEMA
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
        Ok(workflow(&row.id, on_push_and_pr(), BTreeMap::new()))
    }
}

#[cfg(test)]
mod tests;
