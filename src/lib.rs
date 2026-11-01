//! actlith: materialize GitHub Actions workflows from a small config.
//!
//! This file only re-exports node modules (src:V1, src:V3); every node is
//! a private module whose public surface is re-exported here as it is
//! built. The design lives in `SPEC.md` and the `SPEC.md` of each node.

#![allow(unused_imports, reason = "node surfaces are consumed by later waves")]

mod bots;
mod build;
mod bump;
mod check;
mod cli;
mod core;
mod detect;
mod fetch;
mod gate;
mod r#gen;
mod import;
mod kind;
mod pin;
mod registry;
mod render;
mod report;
mod script;
mod system;
mod yaml;

pub use cli::actlith;

// Keep every node's public surface available from the hub while retaining
// each node's private module boundary (src:V3).  The aliases are namespaced
// because several nodes intentionally expose the same helper names.
pub(crate) use bots as bots_api;
pub(crate) use build as build_api;
pub(crate) use bump as bump_api;
pub(crate) use check as check_api;
pub(crate) use core as core_api;
pub(crate) use detect as detect_api;
pub(crate) use fetch as fetch_api;
pub(crate) use gate as gate_api;
pub(crate) use r#gen as gen_api;
pub(crate) use import as import_api;
pub(crate) use kind as kind_api;
pub(crate) use pin as pin_api;
pub(crate) use registry as registry_api;
pub(crate) use render as render_api;
pub(crate) use report as report_api;
pub(crate) use script as script_api;
pub(crate) use system as system_api;
pub(crate) use yaml as yaml_api;
