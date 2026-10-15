//! The v1 kinds by name: the same closed list `src/render` renders (C),
//! named here because `src/render` keeps its own private.

use crate::build::Build;
use crate::bump::{UpdatePins, Upstream};
use crate::gate::Gate;
use crate::kind::Kind;

/// Every v1 kind, in `docs/KINDS.md` order.
const KINDS: [&dyn Kind; 4] = [&Gate, &Build, &UpdatePins, &Upstream];

/// The kind a row's `kind` cell names, if it is a v1 kind.
pub(crate) fn lookup(name: &str) -> Option<&'static dyn Kind> {
    KINDS.into_iter().find(|k| k.name() == name)
}

/// The v1 kind names, as a fix hint lists them.
pub(crate) fn names() -> String {
    let quoted: Vec<String> =
        KINDS.iter().map(|k| format!("`{}`", k.name())).collect();
    quoted.join(", ")
}

#[cfg(test)]
mod tests;
