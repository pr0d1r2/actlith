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
        format!("{}@{}", self.action, self.sha)
    }

    /// The trailing comment the YAML writer puts after the reference.
    pub(crate) fn comment(&self) -> Comment {
        Comment::new(&self.version)
    }

    /// The step action for the YAML model, with `with` inputs: the writer
    /// prints `uses: owner/repo@<sha>  # <version>`.
    pub(crate) fn step(&self, with: With) -> yaml::Uses {
        yaml::Uses {
            reference: self.reference(),
            version: Some(self.comment()),
            with,
        }
    }
}

#[cfg(test)]
mod tests;
