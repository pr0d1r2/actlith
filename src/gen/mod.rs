//! `src/gen`: `gen` writes rendered files; `gen --check` proves they are current.
//!
//! Scaffold: the node's code lands here (`src/gen` tasks). Each `use`
//! line below is one build-order edge for `sherd wave` (src:C); the
//! `expect` turns into a warning once the node uses them, and goes.

#![expect(unused_imports, reason = "scaffold: build-order edges")]

use crate::render as _;
