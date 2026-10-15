//! `src/gate`: kind `gate`: the hk gate on every CI system (fleet `ci.yml`).
//!
//! Scaffold: the node's code lands here (`src/gate` tasks). Each `use`
//! line below is one build-order edge for `sherd wave` (src:C); the
//! `expect` turns into a warning once the node uses them, and goes.

#![expect(unused_imports, reason = "scaffold: build-order edges")]

use crate::kind as _;
