//! `greptile.json`: skip release PRs (R11). JSON has no comments, so the
//! file carries no generated header; drift is still `gen --check`'s.
//!
//! Keys per Greptile's config reference: `disabledLabels` (`string[]`,
//! skip PRs with these labels) and `ignoreKeywords` (newline-separated
//! string, skip PRs with these in title or description).

use super::error::BotError;
use super::model::File;
use super::{args, json};
use crate::registry::BotRow;

/// Where Greptile reads its config.
pub(crate) const PATH: &str = "greptile.json";

/// Labels release tooling puts on its PRs.
pub(crate) const LABELS: [&str; 2] = ["release", "autorelease: pending"];

/// Title and body phrases of release-plz and Release Please PRs.
pub(crate) const KEYWORDS: [&str; 4] = [
    "chore: release",
    "chore(main): release",
    "This PR was generated with release-plz",
    "This PR was generated with Release Please",
];

/// The `greptile` row's file. The bot takes no args.
///
/// # Errors
///
/// Any arg.
pub(crate) fn file(row: &BotRow) -> Result<File, BotError> {
    args::known(row, &[])?;
    let bytes = format!(
        "{{\n  \"disabledLabels\": {},\n  \"ignoreKeywords\": {}\n}}\n",
        json::array(&LABELS),
        json::string(&KEYWORDS.join("\n")),
    );
    Ok(File {
        path: PATH.to_owned(),
        bytes,
    })
}

#[cfg(test)]
mod tests;
