//! `gen` (T24): write each rendered file under the root, only where its
//! bytes differ.
//!
//! Files are written with the default mode (0644 under a usual umask),
//! seeded scripts included: workflows run them through `bash`, so they
//! need no executable bit.

use std::io;
use std::path::Path;

use crate::render::File;

#[cfg(test)]
mod tests;

/// Write each of `files` under `root`, creating parent directories,
/// skipping a file whose bytes already match; returns the paths written,
/// in the order of `files`.
///
/// # Errors
///
/// A path that is not plain and relative (before anything is written),
/// or the first failed read, directory creation or write.
pub(crate) fn r#gen(root: &Path, files: &[File]) -> io::Result<Vec<String>> {
    let _ = (root, files);
    Err(io::ErrorKind::Unsupported.into())
}
