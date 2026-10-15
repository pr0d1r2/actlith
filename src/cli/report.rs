//! `report [--format text|json|md]` (`.:I`, V34). Dispatch only: the
//! report is `src/report`'s.

use std::path::Path;

use super::exit::{Outcome, Ran, USAGE};
use super::load::registry;
use crate::pin::PinTable;
use crate::registry::Registry;
use crate::report::{Format, report as reported};

/// `report`: the `.actlith` under `root` in `format`, on stdout.
///
/// # Errors
///
/// Exit 2: `.actlith` missing or refused, an `## actions` row the pin
/// table refuses, or a row its kind refuses.
pub(crate) fn report(root: &Path, format: Format) -> Ran {
    of(&registry(root)?, format)
}

/// `registry` reported in `format`, pinned from the built-in table and
/// its `## actions` rows.
fn of(registry: &Registry, format: Format) -> Ran {
    let pins = PinTable::builtin()
        .with_overrides(&registry.actions)
        .map_err(|e| Outcome::fail(USAGE, e))?;
    let text = reported(registry, &pins, format)
        .map_err(|e| Outcome::fail(USAGE, e))?;
    Ok(Outcome::ok(text))
}

#[cfg(test)]
mod tests;
