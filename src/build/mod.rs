//! `src/build`: kind `build`: build & cache a flake package per system,
//! prove the cache has it (T20, V30).
//!
//! Composition only (src C): the checked row in `plan`, the jobs
//! `build`, `check` and `verify-cache` in `jobs`, the `Kind` in `kind`.
#![cfg_attr(
    not(test),
    allow(dead_code, reason = "used by src/render in a later wave")
)]
#![allow(unused_imports, reason = "re-exported for src/render in a later wave")]

// Build-order edges for `sherd wave` (src C), one per line; `src/script`
// ships the `verify-cache.sh` the `verify-cache` job runs (V30).
use crate::kind as _;
use crate::script as _;

mod jobs;
mod kind;
mod plan;

pub(crate) use jobs::VERIFY_SCRIPT;
pub(crate) use kind::Build;
