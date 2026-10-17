//! A `build` row made ready for its jobs: the cache it must have, the
//! `package` checked as a flake attribute name before it reaches any
//! `run:` line (`src/kind:V3`), and the job timeout.

use crate::kind::{Args, Cache, KindError, Row};
use crate::pin::PinTable;

/// What every `build` job is made from.
#[derive(Clone, Debug)]
pub(crate) struct Plan<'a> {
    pub(crate) row: &'a Row,
    pub(crate) pins: &'a PinTable,
    /// The cache `build` pushes to and `verify-cache` asks.
    pub(crate) cache: &'a Cache,
    /// The flake package, `.#packages.<system>.<package>`.
    pub(crate) package: &'a str,
    /// `timeout-minutes` of every job.
    pub(crate) minutes: u32,
}

impl<'a> Plan<'a> {
    /// The plan for `row`; refused without a cache, with a `package`
    /// that is not an attribute name, or with a bad `timeout`.
    pub(crate) fn new(
        row: &'a Row,
        pins: &'a PinTable,
    ) -> Result<Self, KindError> {
        Ok(Self {
            row,
            pins,
            cache: cache(row)?,
            package: package(&row.args)?,
            minutes: row.args.timeout()?,
        })
    }
}

/// The row's `package` arg, refused unless it is an attribute name.
pub(crate) fn package(args: &Args) -> Result<&str, KindError> {
    let name = args.get("package").unwrap_or_default();
    if attribute(name) {
        return Ok(name);
    }
    Err(KindError::row(
        args.line(),
        format!(
            "package `{}` is not a flake attribute name",
            name.escape_debug()
        ),
        "use letters, digits, `_` and `-`, starting with a letter or `_`",
    ))
}

/// The row's cache, which `build` cannot do without.
pub(crate) fn cache(row: &Row) -> Result<&Cache, KindError> {
    row.cache.as_ref().ok_or_else(|| {
        KindError::row(
            row.line,
            "kind `build` needs a cache: it pushes there and verify-cache \
             asks it for every built path",
            "name a cachix cache in the cache column",
        )
    })
}

/// `name` is a letter or `_`, then letters, digits, `_` and `-`: a flake
/// attribute name that is also one shell word.
pub(crate) fn attribute(name: &str) -> bool {
    let word = |c: char| c.is_ascii_alphanumeric() || matches!(c, '_' | '-');
    let first = name.chars().next();
    first.is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && name.chars().all(word)
}

#[cfg(test)]
mod tests;
