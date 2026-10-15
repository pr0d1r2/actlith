//! `src/cli`: clap verbs, flags and exit codes over the verb nodes.
//!
//! Composition only (src C): the argv table in `args`, the exit codes
//! and printed outcome in `exit`, the entry point and dispatch in
//! `entry`; each verb group dispatches to its node: `files` (`gen`,
//! `check`), `rows` (`detect`, `seed`, `import`), `report`, `pin`,
//! `meta` (`completions`, `man`). `load` reads `.actlith`, `ls_files`
//! the tracked files. `clap` lives here only (`.:C`).
#![allow(
    dead_code,
    clippy::unnecessary_wraps,
    reason = "RED: the stubs dispatch nothing yet"
)]

// Build-order edges for `sherd wave` (src C), one per line; the
// submodules use them.
#[expect(unused_imports, reason = "build-order edge")]
use crate::check as _;
#[expect(unused_imports, reason = "build-order edge")]
use crate::detect as _;
#[expect(unused_imports, reason = "build-order edge")]
use crate::fetch as _;
#[expect(unused_imports, reason = "build-order edge")]
use crate::r#gen as _;
#[expect(unused_imports, reason = "build-order edge")]
use crate::import as _;
#[expect(unused_imports, reason = "build-order edge")]
use crate::render as _;
#[expect(unused_imports, reason = "build-order edge")]
use crate::report as _;

mod args;
mod entry;
mod exit;
mod files;
mod load;
mod ls_files;
mod meta;
mod pin;
mod report;
mod rows;
#[cfg(test)]
mod scratch;

pub use entry::actlith;
