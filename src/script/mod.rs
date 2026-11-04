//! `src/script`: seeded shell scripts shipped in the crate, written by `gen` with header.
//!
//! The scripts are real files beside this module (`update-pins.sh`,
//! `open-pr.sh`, `verify-cache.sh`, and for https caches `push-https.sh`
//! and `verify-https.sh`), embedded by `embed` (src/script C).

mod embed;

pub(crate) use embed::scripts;
