//! `src/bump`: kinds `update-pins` & `upstream`: scheduled pull-model
//! bump PRs.
//!
//! Composition only (src C): the cron check lives in `cron`, what both
//! kinds share (token, runner, pushing checkout, job permissions) in
//! `shared`, the kinds in `update_pins` and `upstream`; `src/render`
//! picks them up.

// Build-order edges for `sherd wave` (src C), one per line.
#[expect(unused_imports, reason = "build-order edge")]
use crate::kind as _;
#[expect(unused_imports, reason = "build-order edge")]
use crate::script as _;

mod cron;
mod shared;
mod update_pins;
mod upstream;

pub(crate) use update_pins::UpdatePins;
pub(crate) use upstream::Upstream;
