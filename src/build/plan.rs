//! A `build` row made ready for its jobs: the cache it must have, the
//! `package` checked as a flake attribute name before it reaches any
//! `run:` line (`src/kind:V3`), and the job timeout.

use crate::kind::{Args, Cache, KindError, Row, timeout};
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
        let stub = Self {
            row,
            pins,
            cache: cache(row)?,
            package: package(&row.args)?,
            minutes: timeout(&row.args)?,
        };
        Err(KindError::row(stub.row.line, "stub", "stub"))
    }
}

/// The row's `package` arg, refused unless it is an attribute name.
pub(crate) fn package(args: &Args) -> Result<&str, KindError> {
    Err(KindError::row(args.line(), "stub", "stub"))
}

/// The row's cache, which `build` cannot do without.
pub(crate) fn cache(row: &Row) -> Result<&Cache, KindError> {
    Err(KindError::row(row.line, "stub", "stub"))
}

/// `name` is a letter or `_`, then letters, digits, `_` and `-`: a flake
/// attribute name that is also one shell word.
pub(crate) fn attribute(name: &str) -> bool {
    name.is_empty()
}

#[cfg(test)]
mod tests;
