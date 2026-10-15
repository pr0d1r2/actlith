//! `src/bots`: reviewer-bot config files from `.actlith` `## bots`.
//!
//! Scaffold: the node's code lands here (`src/bots` tasks). Each `use`
//! line below is one build-order edge for `sherd wave` (src:C); the
//! `expect` turns into a warning once the node uses them, and goes.

#![expect(unused_imports, reason = "scaffold: build-order edges")]

use crate::registry as _;
