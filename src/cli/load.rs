//! `.actlith` under `--root`: read, parsed and rendered for the verbs
//! that need it; a missing file is exit 2, never an empty render
//! (`src/registry:V12`, V34).

use std::fs;
use std::io;
use std::path::Path;

use super::exit::{Outcome, USAGE};
use crate::registry::{Registry, parse};
use crate::render::{File, render};

/// The registry file under the root (`.:I` file).
pub(crate) const FILE: &str = ".actlith";

/// The parsed `<root>/.actlith`.
///
/// # Errors
///
/// Exit 2: the file is missing or unreadable, or a row is refused.
pub(crate) fn registry(root: &Path) -> Result<Registry, Outcome> {
    let path = root.join(FILE);
    let text = fs::read_to_string(&path).map_err(|e| unreadable(&path, &e))?;
    parse(&text).map_err(|e| Outcome::fail(USAGE, e))
}

/// The parsed `<root>/.actlith` and the files it renders.
///
/// # Errors
///
/// As [`registry`], or exit 2 for a row or pin render refuses.
pub(crate) fn rendered(root: &Path) -> Result<(Registry, Vec<File>), Outcome> {
    let registry = registry(root)?;
    let files = render(&registry).map_err(|e| Outcome::fail(USAGE, e))?;
    Ok((registry, files))
}

/// Exit 2 for `.actlith` at `path`: missing (seed it) or unreadable.
fn unreadable(path: &Path, error: &io::Error) -> Outcome {
    if error.kind() != io::ErrorKind::NotFound {
        return Outcome::io(path, "read", error);
    }
    Outcome::fail(
        USAGE,
        format!(
            "{}: not found\n  fix: run `actlith seed --init`, or pass \
             --root DIR naming the repository",
            path.display()
        ),
    )
}

#[cfg(test)]
mod tests;
