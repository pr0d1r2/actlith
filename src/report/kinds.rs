//! The v1 kinds by name: the same closed list `src/render` renders (C),
//! named here because `src/render` keeps its own private.

use crate::kind::Kind;

/// The kind a row's `kind` cell names, if it is a v1 kind.
pub(crate) fn lookup(_name: &str) -> Option<&'static dyn Kind> {
    None
}

/// The v1 kind names, as a fix hint lists them.
pub(crate) fn names() -> String {
    String::new()
}

#[cfg(test)]
mod tests;
