//! The closed list of v1 kinds (C): `gate`, `build`, `update-pins`,
//! `upstream`, each with the seeded scripts its workflows run (V31).

use crate::build::Build;
use crate::bump::{UpdatePins, Upstream};
use crate::gate::Gate;
use crate::kind::Kind;

/// `scripts/actlith/open-pr.sh`: opens the bump PR.
const OPEN_PR: &str = "scripts/actlith/open-pr.sh";

/// `scripts/actlith/update-pins.sh`: runs `open-pr.sh` itself.
const UPDATE_PINS: &str = "scripts/actlith/update-pins.sh";

/// A v1 kind and the seeded scripts (published paths) it needs.
#[derive(Clone, Copy)]
pub(crate) struct Entry {
    pub(crate) kind: &'static dyn Kind,
    pub(crate) scripts: &'static [&'static str],
}

/// Every v1 kind, in `docs/KINDS.md` order.
pub(crate) fn entries() -> [Entry; 4] {
    [
        entry(&Gate, &[]),
        entry(&Build, Build::scripts()),
        entry(&UpdatePins, &[UPDATE_PINS, OPEN_PR]),
        entry(&Upstream, &[OPEN_PR]),
    ]
}

const fn entry(
    kind: &'static dyn Kind,
    scripts: &'static [&'static str],
) -> Entry {
    Entry { kind, scripts }
}

/// The kind a row's `kind` cell names, if it is a v1 kind.
pub(crate) fn lookup(_name: &str) -> Option<Entry> {
    None
}

/// The v1 kind names, as a fix hint lists them.
pub(crate) fn names() -> String {
    String::new()
}

#[cfg(test)]
mod tests;
