//! The pin table: built-in pins, overridden per repo by `## actions` rows
//! (C, V2). No network here: `src/fetch` resolves tags.

use std::collections::BTreeMap;

use super::error::PinError;
use super::uses::Uses;
use crate::registry::ActionPin;

/// The built-in pins: `(action, version, sha)`. Bumped by `actlith pin` on
/// actlith's own repo; the same SHAs as `.github/workflows/ci.yml`.
const BUILTIN: [(&str, &str, &str); 3] = [
    (
        "actions/checkout",
        "v7.0.1",
        "3d3c42e5aac5ba805825da76410c181273ba90b1",
    ),
    (
        "cachix/install-nix-action",
        "v31.11.0",
        "630ae543ea3a38a9a4166f03376c02c50f408342",
    ),
    (
        "cachix/cachix-action",
        "v17",
        "5f2d7c5294214f71b873db4b969586b980625e71",
    ),
];

/// Every pin a render may use, keyed by action.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct PinTable(BTreeMap<String, Uses>);

impl PinTable {
    /// The built-in table.
    pub(crate) fn builtin() -> Self {
        let pin = |&(action, version, sha): &(&str, &str, &str)| Uses {
            action: action.to_owned(),
            sha: sha.to_owned(),
            version: version.to_owned(),
        };
        Self(
            BUILTIN
                .iter()
                .map(pin)
                .map(|u| (u.action.clone(), u))
                .collect(),
        )
    }

    /// This table with each `## actions` row replacing or adding its
    /// action's pin; a row whose SHA is not 40-hex is refused.
    pub(crate) fn with_overrides(
        mut self,
        rows: &[ActionPin],
    ) -> Result<Self, PinError> {
        for row in rows {
            let uses = checked(row)?;
            self.0.insert(uses.action.clone(), uses);
        }
        Ok(self)
    }

    /// The pin of `action`, or an error naming it.
    pub(crate) fn uses(&self, action: &str) -> Result<Uses, PinError> {
        self.0
            .get(action)
            .cloned()
            .ok_or_else(|| PinError::Missing {
                action: action.to_owned(),
            })
    }
}

/// The row as a pin, when its SHA is a full commit SHA.
fn checked(row: &ActionPin) -> Result<Uses, PinError> {
    if !full_sha(&row.sha) {
        return Err(PinError::NotSha {
            line: row.line,
            action: row.action.clone(),
            sha: row.sha.clone(),
        });
    }
    Ok(Uses {
        action: row.action.clone(),
        sha: row.sha.clone(),
        version: row.version.clone(),
    })
}

/// 40 lowercase hex digits: a tag or branch never passes (V2).
fn full_sha(sha: &str) -> bool {
    let hex = |c: char| c.is_ascii_digit() || ('a'..='f').contains(&c);
    sha.len() == 40 && sha.chars().all(hex)
}

#[cfg(test)]
mod tests;
