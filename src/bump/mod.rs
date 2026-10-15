//! `src/bump`: kinds `update-pins` & `upstream`: scheduled pull-model bump PRs.
//!
//! Scaffold: the node's code lands here (`src/bump` tasks). Each `use`
//! line below is one build-order edge for `sherd wave` (src:C); the
//! `expect` turns into a warning once the node uses them, and goes.

#![expect(unused_imports, reason = "scaffold: build-order edges")]

use crate::kind as _;
use crate::script as _;
