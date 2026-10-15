//! `src/bots`: reviewer-bot config files from `.actlith` `## bots`.
//!
//! Composition only (src C): the logic lives in the submodules. One file
//! per v1 bot (`coderabbit`, `greptile`, `socket`); `bot_files` turns the
//! rows into files and refuses an unknown bot or arg, naming its line
//! (V38).
//!
//! Until `src/render` lands, the lib build sees the node as unused: dead
//! code is `allow`ed, as in `src/registry`, and the re-exports carry
//! `expect(unused_imports)`.
#![cfg_attr(
    not(test),
    allow(dead_code, reason = "used by src/render in a later wave")
)]

mod args;
mod coderabbit;
mod error;
mod files;
mod greptile;
mod json;
mod model;
mod socket;

// Build-order edges for `sherd wave` (src C); the submodules use them.
#[expect(unused_imports, reason = "build-order edge")]
use crate::registry as _;
#[expect(unused_imports, reason = "build-order edge")]
use crate::yaml as _;

pub(crate) use error::BotError;
#[cfg_attr(
    not(test),
    expect(unused_imports, reason = "used by src/render in a later wave")
)]
pub(crate) use files::bot_files;
pub(crate) use model::File;
