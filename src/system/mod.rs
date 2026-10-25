//! `src/system`: nix system name -> GitHub runner: the hosted label from
//! one table, or the repository's own labels from `## runners`.
//!
//! The table and its lookups live in `table` (V15), the choice between
//! it and `## runners` in `host` (V53); this file only composes.
#![expect(
    unused_imports,
    reason = "re-exported for src/render and the kind nodes in a later wave"
)]

mod host;
mod table;

pub(crate) use host::{Host, Runners, host};
pub(crate) use table::{
    DEFAULT_SYSTEMS, System, SystemError, parse_system, runner,
};
