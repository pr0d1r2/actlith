//! `src/kind`: what every kind shares: the `Kind` trait, arg schema,
//! hardening (V3), common steps and declared secrets (V14).
//!
//! Composition only (src C): the trait lives in `contract`, the row
//! check in `row` (the row, its args and cache are pipelith's `kind`,
//! through `src/core`), the common builders in
//! `workflow`, `job`, `step`, `checkout` and `run`, and the cache steps
//! in the child node `cache`.
//! Dead code is `allow`ed, not `expect`ed, until a consumer lands (see
//! `src/registry`).
#![cfg_attr(
    not(test),
    allow(
        dead_code,
        reason = "used by src/gate, src/build and src/bump in a later wave"
    )
)]
#![allow(
    unused_imports,
    reason = "re-exported for src/gate, src/build and src/bump in a later \
              wave"
)]

// Build-order edges for `sherd wave` (src C), one per line.
use crate::pin as _;
use crate::registry as _;
use crate::system as _;
use crate::yaml as _;

mod cache;
mod checkout;
mod contract;
mod emulation;
mod error;
mod job;
mod row;
mod run;
mod secret;
mod step;
mod workflow;

pub(crate) use crate::core::cache::{BinaryCache, CacheKind, Caches, Push};
pub(crate) use crate::core::kind::{
    ArgDefault, ArgSpec, Args, BUILD, GATE, KindSpec, Row, TIMEOUT,
    UPDATE_PINS, UPSTREAM, parse_args, portable,
};
pub(crate) use cache::{
    CachixPush, PUSH_FILTER, PUSH_SCRIPT, READ_SCRIPT, SKIP_FORK_PR,
    SKIP_UNLESS_MAIN, cachix, https_env, https_push, install_nix, push_https,
    push_secret, read_secret, secrets, url,
};
pub(crate) use checkout::{Checkout, checkout};
pub(crate) use contract::Kind;
pub(crate) use emulation::{RUNNER_SYSTEM, emulated, guard, refuse_silent};
pub(crate) use error::KindError;
pub(crate) use job::{
    Hosts, TRUSTED, hosts, job, matrix_runner, own, place, runner_matrix, widen,
};
pub(crate) use row::{CACHE_PUSH, Checked, EMULATE, EXTRAS, XCODE, check};
pub(crate) use run::run;
pub(crate) use secret::Secret;
pub(crate) use step::{step, uses};
pub(crate) use workflow::{
    concurrency, on_push_and_pr, on_schedule, read_only, workflow,
};
