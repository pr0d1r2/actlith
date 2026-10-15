//! `.coderabbit.yaml`: a stub that loads the fleet's central config (R10).
//!
//! `remote_config` with `repository`, `ref` and `path` is `CodeRabbit`'s
//! shared-configuration form for a repository on the same platform;
//! `inheritance: true` merges the stub with the levels above it (R10,
//! <https://docs.coderabbit.ai/getting-started/yaml-configuration>).

use super::model::File;
use crate::registry::BotRow;

use super::error::BotError;

/// Where `CodeRabbit` reads its config, in the repo and in the central repo.
pub(crate) const PATH: &str = ".coderabbit.yaml";

/// The `coderabbit` row's file: `central=<owner>/<repo>` is required.
///
/// # Errors
///
/// An arg other than `central`, no `central`, or one that is not
/// `owner/repo`.
pub(crate) fn file(row: &BotRow) -> Result<File, BotError> {
    let _ = row;
    Err(BotError::default())
}

#[cfg(test)]
mod tests;
