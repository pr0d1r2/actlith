//! One resolved pin: `owner/repo@<sha>  # <version>` (V2).

use crate::yaml::{self, Comment, With};

/// An action pinned to a full commit SHA, with the version it pins.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Uses {
    /// `owner/repo` (or `owner/repo/path`).
    pub(crate) action: String,
    /// The 40-hex commit SHA.
    pub(crate) sha: String,
    /// The tag the SHA was resolved from, e.g. `v7.0.1`.
    pub(crate) version: String,
}

impl Uses {
    /// The `uses:` value: `owner/repo@<sha>`.
    pub(crate) fn reference(&self) -> String {
        self.action.clone()
    }

    /// The trailing comment the YAML writer puts after the reference.
    pub(crate) fn comment(&self) -> Comment {
        Comment::new(&self.sha)
    }

    /// The step action for the YAML model, with `with` inputs.
    pub(crate) fn step(&self, with: With) -> yaml::Uses {
        yaml::Uses {
            reference: self.version.clone(),
            version: None,
            with,
        }
    }
}

#[cfg(test)]
mod tests;
