//! `gen --check` (T24, V6): compare each rendered file with the one under
//! the root; writes nothing.

use std::fmt;
use std::io;
use std::path::Path;

use crate::render::File;

#[cfg(test)]
mod tests;

/// A rendered file the root does not hold as rendered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Stale {
    /// No file at this path.
    Missing(String),
    /// A file whose bytes differ from the rendered ones.
    Changed(String),
}

impl fmt::Display for Stale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = f;
        Ok(())
    }
}

/// Every one of `files` missing or changed under `root`, in the order of
/// `files`; empty when all are current. Reads only (V6).
///
/// # Errors
///
/// A path that is not plain and relative, or a read failing other than
/// for a missing file (a directory at the path, no permission).
pub(crate) fn check(root: &Path, files: &[File]) -> io::Result<Vec<Stale>> {
    let _ = (root, files);
    Err(io::ErrorKind::Unsupported.into())
}
