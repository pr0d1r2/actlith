//! `src/render`: `.actlith` rows -> ordered set of files to write.
//!
//! Composition only (src C): the closed list of v1 kinds and the scripts
//! each needs live in `kinds`, the refusal in `error`, the render itself
//! in `files`. One file type for every output: `src/bots`' [`File`].
//! Dead code is `allow`ed until `src/gen` picks `render` up; the `allow`
//! also keeps every node `render` reaches live.
#![cfg_attr(
    not(test),
    allow(dead_code, reason = "used by src/gen in a later wave")
)]

// Build-order edges for `sherd wave` (src C), one per line; the
// submodules use them.
#[expect(unused_imports, reason = "build-order edge")]
use crate::bots as _;
#[expect(unused_imports, reason = "build-order edge")]
use crate::build as _;
#[expect(unused_imports, reason = "build-order edge")]
use crate::bump as _;
#[expect(unused_imports, reason = "build-order edge")]
use crate::gate as _;
#[expect(unused_imports, reason = "build-order edge")]
use crate::registry as _;

mod error;
mod files;
mod kinds;

#[cfg_attr(
    not(test),
    expect(unused_imports, reason = "used by src/gen in a later wave")
)]
pub(crate) use crate::bots::File;
#[cfg_attr(
    not(test),
    expect(unused_imports, reason = "used by src/gen in a later wave")
)]
pub(crate) use error::RenderError;
pub(crate) use files::render;
