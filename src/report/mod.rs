//! `src/report`: `report`: workflows x systems x pins and the secrets each
//! needs (T28), as text, JSON (V33) or Markdown.
//!
//! Composition only (src C): the v1 kinds in `kinds`, the apps `## bots`
//! assume in `apps`, the sorted model in `model` (built by `collect`),
//! the formats in `text`, `json` with `document`, and `md`, the verb in
//! `output`, the refusal in `error`. Dead code is `allow`ed until
//! `src/cli` picks `report` up (see `src/registry`).
#![cfg_attr(
    not(test),
    allow(dead_code, reason = "used by src/cli in a later wave")
)]

// Build-order edges for `sherd wave` (src C), one per line; the
// submodules use them. `src/render` keeps its kind list private, so
// `kinds` names the v1 kinds again.
#[expect(unused_imports, reason = "build-order edge")]
use crate::build as _;
#[expect(unused_imports, reason = "build-order edge")]
use crate::bump as _;
#[expect(unused_imports, reason = "build-order edge")]
use crate::gate as _;
#[expect(unused_imports, reason = "build-order edge")]
use crate::kind as _;
#[expect(unused_imports, reason = "build-order edge")]
use crate::pin as _;
#[expect(unused_imports, reason = "build-order edge")]
use crate::registry as _;
#[expect(unused_imports, reason = "build-order edge")]
use crate::system as _;
#[expect(unused_imports, reason = "build-order edge")]
use crate::yaml as _;

mod apps;
mod collect;
mod document;
mod error;
mod format;
mod json;
mod kinds;
mod md;
mod model;
mod output;
mod text;

#[cfg_attr(
    not(test),
    expect(unused_imports, reason = "used by src/cli in a later wave")
)]
pub(crate) use error::ReportError;
pub(crate) use format::Format;
#[cfg_attr(
    not(test),
    expect(unused_imports, reason = "used by src/cli in a later wave")
)]
pub(crate) use output::report;
