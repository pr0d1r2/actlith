//! `src/kind`: what every kind shares: the `Kind` trait, arg schema,
//! hardening (V3), common steps and declared secrets (V14).
//!
//! Composition only (src C): the trait lives in `contract`, the row
//! check in `row` (the row, its args and cache are pipelith's `kind`,
//! through `src/core`), the common builders in
//! `workflow`, `job`, `step`, `checkout`, `nix`, `cachix` and `run`.
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

mod cachix;
mod checkout;
mod contract;
mod error;
mod job;
mod nix;
mod row;
mod run;
mod secret;
mod step;
mod workflow;

pub(crate) use crate::core::kind::{
    ArgDefault, ArgSpec, Args, BUILD, Cache, GATE, KindSpec, Row, TIMEOUT,
    UPDATE_PINS, UPSTREAM, parse_args, portable,
};
pub(crate) use cachix::{
    CachixPush, PUSH_FILTER, SKIP_FORK_PR, SKIP_UNLESS_MAIN, cachix,
};
pub(crate) use checkout::{Checkout, checkout};
pub(crate) use contract::Kind;
pub(crate) use error::KindError;
pub(crate) use job::{job, matrix_runner, runner_matrix, widen};
pub(crate) use nix::install_nix;
pub(crate) use row::{EXTRAS, XCODE, check};
pub(crate) use run::run;
pub(crate) use secret::Secret;
pub(crate) use step::{step, uses};
pub(crate) use workflow::{
    concurrency, on_push_and_pr, on_schedule, read_only, workflow,
};
