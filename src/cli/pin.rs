//! `pin [--dry-run]`: the one networked verb (`.:I`, `src/fetch:V16`).
//! Dispatch only: the resolve and the `## actions` rewrite are
//! `src/fetch`'s; the resolver is passed in so tests need no network.

use std::path::Path;

use super::exit::{Outcome, Ran, USAGE};
use crate::fetch::{Resolver, pin as resolve};

/// `pin`: the fresh `## actions` rows on stdout, written into
/// `.actlith` unless `dry_run`.
///
/// # Errors
///
/// Exit 2: `.actlith` unreadable, refused or unwritable, or a failed
/// resolve; each names its fix.
pub(crate) fn pin(root: &Path, dry_run: bool, resolver: &dyn Resolver) -> Ran {
    let rows = resolve(root, dry_run, resolver)
        .map_err(|e| Outcome::fail(USAGE, e))?;
    Ok(Outcome::ok(rows))
}

#[cfg(test)]
mod tests;
