//! Kind `build` (`docs/KINDS.md`): build and cache a flake package on
//! every system, then prove the cache has it (T20, V30).

use super::jobs::VERIFY_SCRIPT;
use crate::kind::{ArgSchema, ArgSpec, Kind, KindError, Row, Secret, TIMEOUT};
use crate::pin::PinTable;
use crate::system::System;
use crate::yaml::Workflow;

/// `package` (required) and `timeout`.
const SCHEMA: [ArgSpec; 2] = [
    ArgSpec {
        key: "package",
        required: true,
        default: None,
    },
    TIMEOUT,
];

/// The systems `-` means: those of `gate` (`docs/KINDS.md`), equal to
/// `crate::system::DEFAULT_SYSTEMS` (a test holds them so). Spelled here
/// because naming that constant outside a test leaves `src/system`'s
/// `expect(dead_code)` unfulfilled, which clippy denies.
const SYSTEMS: [System; 3] = [
    System::X86_64Linux,
    System::Aarch64Linux,
    System::Aarch64Darwin,
];

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
    fn name(&self) -> &'static str {
        "build"
    }

    fn schema(&self) -> &'static ArgSchema {
        &SCHEMA
    }

    fn default_systems(&self) -> &'static [System] {
        &SYSTEMS
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
