//! `src/build`: kind `build`: build & cache a flake package per system,
//! prove the cache has it (T20, V30).
//!
//! Composition only (src C): the checked row in `plan`, the jobs
//! `build`, `check` and `verify-cache` in `jobs`, the `Kind` in `kind`.

// Build-order edges for `sherd wave` (src C), one per line; `src/script`
// ships the `verify-cache.sh` the `verify-cache` job runs (V30).
#[expect(unused_imports, reason = "build-order edge")]
use crate::kind as _;
#[expect(unused_imports, reason = "build-order edge")]
use crate::script as _;

mod jobs;
mod kind;
mod plan;

pub(crate) use kind::Build;
