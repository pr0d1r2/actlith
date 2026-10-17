//! `seed --init`: write the first `.actlith`, never over an existing one
//! (detect V32).

use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

use super::text::text;
use crate::core::detect::{self, seed_init_text};
use crate::registry::WorkflowRow;

/// The file `seed --init` writes under the root.
pub(crate) const FILE: &str = ".actlith";

/// Why `seed --init` wrote nothing.
#[derive(Debug)]
pub(crate) enum SeedError {
    /// The file is already there; it is left as it was (V32, exit 1).
    Exists(PathBuf),
    /// The file could not be written (exit 2, unwritable).
    Write(PathBuf, io::Error),
}

impl SeedError {
    /// The exit code `src/cli` returns: 1 finding, 2 unwritable (`.:I`).
    #[must_use]
    pub(crate) fn code(&self) -> u8 {
        match self {
            Self::Exists(_) => 1,
            Self::Write(..) => 2,
        }
    }
}

impl fmt::Display for SeedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exists(path) => write!(
                f,
                "{}: already exists; `seed --init` never overwrites it",
                path.display()
            ),
            Self::Write(path, err) => {
                write!(f, "{}: cannot write: {err}", path.display())
            }
        }
    }
}

impl std::error::Error for SeedError {}

/// Write `<root>/.actlith` holding `rows` (from `detect`); returns its
/// path.
///
/// # Errors
///
/// [`SeedError::Exists`] if the file is there (it is not touched), else
/// [`SeedError::Write`] with the I/O error.
pub(crate) fn seed_init(
    root: &Path,
    rows: &[WorkflowRow],
) -> Result<PathBuf, SeedError> {
    seed_init_text(root, FILE, &text(rows)).map_err(|e| match e {
        detect::SeedError::Exists(path) => SeedError::Exists(path),
        detect::SeedError::Write(path, err) => SeedError::Write(path, err),
    })
}

#[cfg(test)]
mod tests;
