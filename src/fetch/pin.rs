//! `actlith pin`: resolve every pinned action, rewrite `## actions`
//! (V16, T29).

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use super::error::FetchError;
use super::resolver::{Resolver, checked_sha, checked_tag};
use super::section;
use crate::pin::{PinTable, Uses};
use crate::registry::{Registry, parse};

/// The actions of the built-in pin table. `src/pin` exposes lookup, not
/// iteration, so the names are listed here; versions and SHAs come from
/// `PinTable::builtin()`, and a test checks each name resolves there.
pub(crate) const BUILTIN_ACTIONS: [&str; 3] = [
    "actions/checkout",
    "cachix/install-nix-action",
    "cachix/cachix-action",
];

/// Resolve each action of the pin table (built-in and `## actions` rows)
/// to its latest release tag, or its recorded version when it has none,
/// and that tag to a commit SHA. Returns the new `## actions` rows; writes
/// them into `<root>/.actlith` unless `dry_run` (V16: dry run writes
/// nothing). The caller prints the rows.
///
/// # Errors
///
/// `.actlith` unreadable, unparsable or unwritable; an override refused
/// by the pin table; a resolver failure or a bad tag or SHA.
pub(crate) fn pin(
    root: &Path,
    dry_run: bool,
    resolver: &dyn Resolver,
) -> Result<String, FetchError> {
    let path = root.join(".actlith");
    let text = read(&path)?;
    let reg = parse(&text).map_err(FetchError::Registry)?;
    let rows = section::rows(&resolve_all(&reg, resolver)?);
    if !dry_run {
        write(path, &section::replace(&text, &rows))?;
    }
    Ok(rows)
}

/// Every action of the pin table `reg` overrides, resolved, sorted.
fn resolve_all(
    reg: &Registry,
    resolver: &dyn Resolver,
) -> Result<Vec<Uses>, FetchError> {
    let table = PinTable::builtin()
        .with_overrides(&reg.actions)
        .map_err(FetchError::Pin)?;
    let resolve = |action| resolved(&table, action, resolver);
    names(reg).into_iter().map(resolve).collect()
}

/// Built-in actions and `## actions` rows, sorted, each once.
fn names(reg: &Registry) -> BTreeSet<&str> {
    let rows = reg.actions.iter().map(|row| row.action.as_str());
    BUILTIN_ACTIONS.into_iter().chain(rows).collect()
}

/// `action` at its latest release (or recorded version), checked.
fn resolved(
    table: &PinTable,
    action: &str,
    resolver: &dyn Resolver,
) -> Result<Uses, FetchError> {
    let recorded = table.uses(action).map_err(FetchError::Pin)?;
    let version = resolver.latest(action)?.unwrap_or(recorded.version);
    checked_tag(action, &version)?;
    let sha = checked_sha(action, &version, &resolver.sha(action, &version)?)?;
    let action = action.to_owned();
    Ok(Uses {
        action,
        sha,
        version,
    })
}

/// The text of `.actlith`.
fn read(path: &Path) -> Result<String, FetchError> {
    fs::read_to_string(path).map_err(|e| FetchError::Read {
        path: path.to_owned(),
        why: e.to_string(),
    })
}

/// Replace `.actlith` with `text`.
fn write(path: PathBuf, text: &str) -> Result<(), FetchError> {
    fs::write(&path, text).map_err(|e| FetchError::Write {
        path,
        why: e.to_string(),
    })
}

#[cfg(test)]
mod tests;
