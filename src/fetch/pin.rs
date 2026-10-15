//! `actlith pin`: resolve every pinned action, rewrite `## actions`
//! (V16, T29).

use std::path::Path;

use super::error::FetchError;
use super::resolver::Resolver;

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
#[expect(clippy::unnecessary_wraps, reason = "RED stub")]
pub(crate) fn pin(
    root: &Path,
    dry_run: bool,
    resolver: &dyn Resolver,
) -> Result<String, FetchError> {
    let _ = (root, dry_run, resolver);
    Ok(String::new())
}

#[cfg(test)]
mod tests;
