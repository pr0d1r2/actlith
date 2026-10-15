//! `src/script`: seeded shell scripts shipped in the crate, written by `gen` with header.
//!
//! The scripts are real files beside this module (`update-pins.sh`,
//! `open-pr.sh`, `verify-cache.sh`), embedded by `embed` (src/script C).

#![expect(
    unused_imports,
    reason = "used by src/render and the kind nodes in a later wave"
)]
mod embed;

pub(crate) use embed::scripts;
