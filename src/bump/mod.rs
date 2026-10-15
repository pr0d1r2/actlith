//! `src/bump`: kinds `update-pins` & `upstream`: scheduled pull-model
//! bump PRs.
//!
//! Composition only (src C): the cron check lives in `cron`, what both
//! kinds share (token, runner, pushing checkout, job permissions) in
//! `shared`, the kinds in `update_pins` and `upstream`. Dead code is
//! `allow`ed, not `expect`ed, until `src/render` lands (see
//! `src/registry`).
#![cfg_attr(
    not(test),
    allow(dead_code, reason = "used by src/render in a later wave")
)]
#![allow(
    unused_imports,
    reason = "build-order edges, and re-exported for src/render in a later \
              wave"
)]

// Build-order edges for `sherd wave` (src C), one per line.
use crate::kind as _;
use crate::script as _;

mod cron;
mod shared;
mod update_pins;
mod upstream;

pub(crate) use update_pins::UpdatePins;
pub(crate) use upstream::Upstream;
