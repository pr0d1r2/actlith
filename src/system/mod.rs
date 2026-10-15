//! `src/system`: nix system name -> GitHub-hosted runner label, in one table.
//!
//! The table and its lookups live in `table` (V15); this file only
//! composes.
#![expect(
    unused_imports,
    reason = "re-exported for src/render and the kind nodes in a later wave"
)]
#![cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "used by src/render and the kind nodes in a later wave"
    )
)]

mod table;

pub(crate) use table::{
    DEFAULT_SYSTEMS, System, SystemError, parse_system, runner,
};
