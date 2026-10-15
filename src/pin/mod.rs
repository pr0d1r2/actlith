//! `src/pin`: pin table: every `uses:` resolves to a full commit SHA with
//! its version (V2).
//!
//! Composition only (src C): the built-in table and the `## actions`
//! override merge live in `table`, the rendered reference in `uses`, the
//! refusal in `error`. Dead code is `allow`ed, not `expect`ed, until a
//! consumer lands (see `src/registry`).
#![cfg_attr(
    not(test),
    allow(dead_code, reason = "used by src/kind in a later wave")
)]

#[allow(
    unused_imports,
    reason = "build-order edge for sherd wave (src C)"
)]
use crate::registry as _;
#[allow(
    unused_imports,
    reason = "build-order edge for sherd wave (src C)"
)]
use crate::yaml as _;

mod error;
mod table;
mod uses;

pub(crate) use error::PinError;
#[cfg_attr(
    not(test),
    expect(unused_imports, reason = "used by src/kind in a later wave")
)]
pub(crate) use table::PinTable;
pub(crate) use uses::Uses;
