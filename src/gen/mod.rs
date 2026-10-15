//! `src/gen`: `gen` writes rendered files; `gen --check` proves they are
//! current.
//!
//! Composition only (src C): where a rendered path lands under the root
//! lives in `target`, the write in `write`, the compare in `stale`. The
//! only node besides `pin` that writes files, and only rendered paths
//! under the root (gen C).
//!
//! Until `src/cli` lands, the lib build sees the node as unused: dead code
//! is `allow`ed and the re-exports `expect` it.
#![cfg_attr(
    not(test),
    allow(dead_code, reason = "used by src/cli in a later wave")
)]

// Build-order edge for `sherd wave` (src C); the submodules use it.
#[expect(unused_imports, reason = "build-order edge")]
use crate::render as _;

mod stale;
mod target;
mod write;

pub(crate) use stale::{Stale, check};
pub(crate) use write::r#gen;
