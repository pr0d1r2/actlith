//! actlith: materialize GitHub Actions workflows from a small config.
//!
//! This file only re-exports node modules (src:V1, src:V3); every node is
//! a private module whose public surface is re-exported here as it is
//! built. The design lives in `SPEC.md` and the `SPEC.md` of each node.

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
#[allow(
    unused_imports,
    reason = "node surfaces are consumed by later waves"
)]
pub(crate) use {
    bots as bots_api, build as build_api, bump as bump_api, check as check_api,
    core as core_api, detect as detect_api, fetch as fetch_api,
    gate as gate_api, r#gen as gen_api, import as import_api, kind as kind_api,
    pin as pin_api, registry as registry_api, render as render_api,
    report as report_api, script as script_api, system as system_api,
    yaml as yaml_api,
};
