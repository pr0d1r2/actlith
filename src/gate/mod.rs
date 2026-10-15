//! `src/gate`: kind `gate`: the hk gate on every CI system (fleet
//! `ci.yml`).
//!
//! Composition only (src C): the kind lives in `shape`; `src/render`
//! picks it up.

// Build-order edges for `sherd wave` (src C), one per line.
#[expect(unused_imports, reason = "build-order edge")]
use crate::kind as _;

mod shape;

pub(crate) use shape::Gate;
