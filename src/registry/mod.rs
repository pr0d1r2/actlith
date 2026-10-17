//! `src/registry`: parse `.actlith` into typed rows; every row error named.
//!
//! Composition only (src C): the logic lives in the submodules.
//!
//! Until a consumer lands, the lib build sees the node as unused. The
//! re-exports carry `expect(unused_imports)`; dead code is `allow`ed, not
//! `expect`ed, because rustc 1.95 treats every item under a module-level
//! `expect(dead_code)` as live and then reports the expectation unfulfilled.
#![cfg_attr(
    not(test),
    allow(
        dead_code,
        reason = "used by src/render and src/bots in a later wave"
    )
)]

mod error;
mod model;
mod parse;
mod rows;

pub(crate) use error::RegistryError;
#[cfg_attr(
    not(test),
    expect(unused_imports, reason = "used by src/render in a later wave")
)]
pub(crate) use model::{
    ActionPin, Arg, BotRow, HandWritten, Registry, WorkflowRow,
};
pub(crate) use parse::parse;
