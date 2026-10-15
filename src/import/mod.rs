//! `src/import`: `import`: an existing workflow becomes a `.actlith` row
//! only on exact match.
//!
//! Composition only (src C): the text is normalised in `normalise`, the
//! row values it shows are read in `hints`, candidate rows built in
//! `candidates`, each rendered and compared in `matcher`; a refusal is a
//! `mismatch`. Dead code is `allow`ed until `src/cli` picks `import` up.
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

mod candidates;
mod hints;
mod matcher;
mod mismatch;
mod normalise;

#[cfg_attr(
    not(test),
    expect(unused_imports, reason = "used by src/cli in a later wave")
)]
pub(crate) use matcher::import;
#[cfg_attr(
    not(test),
    expect(unused_imports, reason = "used by src/cli in a later wave")
)]
pub(crate) use mismatch::Mismatch;
