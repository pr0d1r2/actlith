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
/// `InvalidInput` naming `rel` when it is empty, absolute, or has an
/// empty, `.` or `..` segment. Checked on the `/`-separated text, as
/// render writes it, not on `Path` components, which drop an inner `.`.
pub(crate) fn target(root: &Path, rel: &str) -> io::Result<PathBuf> {
    if rel.split('/').all(plain) {
        Ok(root.join(rel))
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("`{rel}`: not a plain relative path under the root"),
        ))
    }
}

/// One segment that names a file or directory, nothing else.
fn plain(segment: &str) -> bool {
    !matches!(segment, "" | "." | "..") && !segment.contains('\\')
}
