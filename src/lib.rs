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
