//! `src/gate`: kind `gate`: the hk gate on every CI system (fleet
//! `ci.yml`).
//!
//! Composition only (src C): the kind lives in `shape`. Dead code is
//! `allow`ed until `src/render` picks the kind up.
#![cfg_attr(
    not(test),
    allow(dead_code, reason = "used by src/render in a later wave")
)]
#![allow(unused_imports, reason = "re-exported for src/render in a later wave")]

// Build-order edges for `sherd wave` (src C), one per line.
use crate::kind as _;

mod shape;

pub(crate) use shape::Gate;
