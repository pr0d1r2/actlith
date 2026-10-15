//! `src/check`: `check`: every workflow file is generated or declared
//! hand-written.
//!
//! Composition only (src C): the finding shape lives in `finding`, the
//! one disk read in `listing`, the pure comparison in `totality` (V7,
//! `src/bots:V38`).
//!
//! Until `src/cli` lands, the lib build sees the node as unused: dead code
//! is `allow`ed, as in `src/registry`, and the re-exports `expect` it.
#![cfg_attr(
    not(test),
    allow(dead_code, reason = "used by src/cli in a later wave")
)]

// Build-order edges for `sherd wave` (src C), one per line; the
// submodules use them.
#[expect(unused_imports, reason = "build-order edge")]
use crate::registry as _;
#[expect(unused_imports, reason = "build-order edge")]
use crate::render as _;

mod finding;
mod listing;
mod totality;

#[cfg_attr(
    not(test),
    expect(unused_imports, reason = "used by src/cli in a later wave")
)]
pub(crate) use finding::{Finding, Problem};
pub(crate) use listing::listing;
pub(crate) use totality::totality;
