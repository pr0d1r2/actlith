//! `src/pin`: pin table: every `uses:` resolves to a full commit SHA with its version.
//!
//! Scaffold: the node's code lands here (`src/pin` tasks). Each `use`
//! line below is one build-order edge for `sherd wave` (src:C); the
//! `expect` turns into a warning once the node uses them, and goes.

#![expect(unused_imports, reason = "scaffold: build-order edges")]

use crate::registry as _;
