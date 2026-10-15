//! `detect`, `seed --init` and `import FILE...` (RED stub).

use std::path::{Path, PathBuf};

use super::exit::{Outcome, Ran};

/// `detect` (RED stub).
///
/// # Errors
///
/// None yet.
pub(crate) fn detect(_root: &Path) -> Ran {
    Ok(Outcome::default())
}

/// `seed --init` (RED stub).
///
/// # Errors
///
/// None yet.
pub(crate) fn seed(_root: &Path, _init: bool) -> Ran {
    Ok(Outcome::default())
}

/// `import FILE...` (RED stub).
pub(crate) fn import(_files: &[PathBuf]) -> Outcome {
    Outcome::default()
}

#[cfg(test)]
mod tests;
