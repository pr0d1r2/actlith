//! The secrets a cache's steps read (`src/kind:V14`): a `## caches`
//! row's url or key secret wherever the cache is read (V60), and its
//! push secret where the job pushes (V61), so `report` lists them.

use super::cachix::CachixPush;
use crate::core::cache::{BinaryCache, CacheKind, CacheValue, Push};
use crate::kind::Secret;
use crate::yaml::Expr;

/// What a url or key secret is read for.
const READ: &str = "cache read";

/// What a cachix token is read for, built in or named by a row.
const CACHIX: &str = "cachix push";

/// What an https cache's netrc secret is read for.
const NETRC: &str = "cache push (netrc)";

/// The secret a push to `cache` reads: `CACHIX_AUTH_TOKEN` for the
/// built-in cache, else the one its row names; `None` when read-only.
pub(crate) fn push_secret(cache: &BinaryCache) -> Option<Secret> {
    let purpose = match cache.kind() {
        CacheKind::Https => NETRC,
        _ => CACHIX,
    };
    match cache.push() {
        Push::Forge => Secret::cachix(),
        Push::Secret(name) => Secret::new(name, purpose),
        _ => None,
    }
}

/// Every secret the steps for `cache` read in `mode`: the read secrets,
/// then the push secret when `mode` pushes.
pub(crate) fn secrets(
    cache: Option<&BinaryCache>,
    mode: CachixPush,
) -> Vec<Secret> {
    let Some(cache) = cache else {
        return Vec::new();
    };
    let read = cache.read_secrets().into_iter();
    let read = read.filter_map(|name| Secret::new(name, READ));
    let push = push_secret(cache).filter(|_| mode.pushes());
    read.chain(push).collect()
}

/// `${{ secrets.<name> }}` for a cell `value` held by a secret; `None`
/// for a literal.
pub(super) fn secret_expr(value: &CacheValue) -> Option<Expr> {
    let name = value.secret()?;
    Secret::new(name, READ).map(|secret| secret.expr())
}

#[cfg(test)]
mod tests;
