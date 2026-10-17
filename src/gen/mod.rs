//! `src/gen`: `gen` writes rendered files; `gen --check` proves they are
//! current.
//!
//! Composition only (src C): the write, the compare and the rule that a
//! rendered path stays plain and under the root are pipelith's `files`,
//! through src/core (T61). The only node besides `pin` that writes files,
//! and only rendered paths under the root (gen C).
//!
//! Until `src/cli` lands, the lib build sees the node as unused: dead code
//! is `allow`ed and the re-exports `expect` it.
#![cfg_attr(
    not(test),
    allow(dead_code, reason = "used by src/cli in a later wave")
)]

// Build-order edge for `sherd wave` (src C).
#[expect(unused_imports, reason = "build-order edge")]
use crate::render as _;

pub(crate) use crate::core::files::{Stale, check, write as r#gen};

#[cfg(test)]
mod tests;
