//! Where a rendered path lands under the root (gen C): relative, plain
//! components only, so nothing `gen` writes or `gen --check` reads can
//! leave the root.

use std::io;
use std::path::{Path, PathBuf};

#[cfg(test)]
mod tests;

/// `rel` joined under `root`.
///
/// # Errors
///
/// `InvalidInput` naming `rel` when it is empty, absolute, or has a `.`,
/// `..` or prefix component.
pub(crate) fn target(root: &Path, rel: &str) -> io::Result<PathBuf> {
    let _ = (root, rel);
    Err(io::ErrorKind::Unsupported.into())
}
