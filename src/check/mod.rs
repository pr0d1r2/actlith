//! `src/check`: `check`: every workflow file is generated or declared hand-written.
//!
//! Scaffold: the node's code lands here (`src/check` tasks). Each `use`
//! line below is one build-order edge for `sherd wave` (src:C); the
//! `expect` turns into a warning once the node uses them, and goes.

#![expect(unused_imports, reason = "scaffold: build-order edges")]

use crate::registry as _;
use crate::render as _;
