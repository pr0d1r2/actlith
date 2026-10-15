//! `src/detect`: `detect` & `seed --init`: proposed rows from tracked files.
//!
//! Composition only (src C): the logic lives in the submodules. Pure
//! except `seed`, which writes the one `.actlith` (detect C, V32).
//!
//! Until `src/cli` lands, the lib build sees the node as unused: dead code
//! is `allow`ed, as in `src/registry`, and the re-exports `expect` it.
#![cfg_attr(
    not(test),
    allow(dead_code, reason = "used by src/cli in a later wave")
)]

mod lock;
mod rows;
mod seed;
mod text;

pub(crate) use rows::detect;
pub(crate) use seed::{SeedError, seed_init};
pub(crate) use text::row_line;
