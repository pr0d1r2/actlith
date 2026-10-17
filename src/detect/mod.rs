//! `src/detect`: `detect` & `seed --init`: proposed rows from tracked files.
//!
//! Composition only (src C): `detect` and the row text are pipelith's
//! (src/core); the off-by-default rule lives in `text`, the write in
//! `seed`. Pure except `seed`, which writes the one `.actlith` (detect C,
//! V32).
//!
//! Until `src/cli` lands, the lib build sees the node as unused: dead code
//! is `allow`ed, as in `src/registry`, and the re-exports `expect` it.
#![cfg_attr(
    not(test),
    allow(dead_code, reason = "used by src/cli in a later wave")
)]

mod seed;
mod text;

pub(crate) use crate::core::detect::{detect, row_line};
pub(crate) use seed::{SeedError, seed_init};
pub(crate) use text::proposal;
