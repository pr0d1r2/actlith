//! `.coderabbit.yaml`: a stub that loads the fleet's central config (R10).
//!
//! `remote_config` with `repository`, `ref` and `path` is `CodeRabbit`'s
//! shared-configuration form for a repository on the same platform;
//! `inheritance: true` merges the stub with the levels above it (R10,
//! <https://docs.coderabbit.ai/getting-started/yaml-configuration>).
//!
//! Written here, not through `src/yaml`: that writer's tree is private to
//! it and shaped for workflows. The one caller value, `central`, is held
//! to `owner/repo` in plain characters, so it can sit in double quotes
//! as it is.

use super::args;
use super::error::BotError;
use super::model::File;
use crate::registry::BotRow;
use crate::yaml::HEADER;

/// Where `CodeRabbit` reads its config, in the repo and in the central repo.
pub(crate) const PATH: &str = ".coderabbit.yaml";

/// The schema modeline `CodeRabbit` documents for editors.
const SCHEMA: &str = "# yaml-language-server: \
    $schema=https://coderabbit.ai/integrations/schema.v2.json";

/// The `coderabbit` row's file: `central=<owner>/<repo>` is required.
///
/// # Errors
///
/// An arg other than `central`, no `central`, or one that is not
/// `owner/repo`.
pub(crate) fn file(row: &BotRow) -> Result<File, BotError> {
    args::known(row, &["central"])?;
    let central = repo(row, args::required(row, "central", "<owner>/<repo>")?)?;
    let bytes = format!(
        "{HEADER}\n---\n{SCHEMA}\ninheritance: true\nremote_config:\n  \
         repository: \"{central}\"\n  ref: \"main\"\n  path: \"{PATH}\"\n"
    );
    Ok(File {
        path: PATH.to_owned(),
        bytes,
    })
}

/// `value` when it is `owner/repo`, both parts `[A-Za-z0-9._-]`.
fn repo<'a>(row: &BotRow, value: &'a str) -> Result<&'a str, BotError> {
    let plain = |s: &str| {
        !s.is_empty()
            && s.chars()
                .all(|c| c.is_ascii_alphanumeric() || "._-".contains(c))
    };
    match value.split_once('/') {
        Some((owner, name)) if plain(owner) && plain(name) => Ok(value),
        _ => Err(BotError::new(
            row.line,
            format!("`central` `{value}` is not `owner/repo`"),
            "name the central config repo as `owner/repo`",
        )),
    }
}

#[cfg(test)]
mod tests;
