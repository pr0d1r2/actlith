//! `src/build`: kind `build`: build & cache a flake package per system, prove the cache has it.
//!
//! Scaffold: the node's code lands here (`src/build` tasks). Each `use`
//! line below is one build-order edge for `sherd wave` (src:C); the
//! `expect` turns into a warning once the node uses them, and goes.

#![expect(unused_imports, reason = "scaffold: build-order edges")]

use crate::kind as _;
use crate::script as _;
