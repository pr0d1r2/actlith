//! `src/report`: `report`: workflows x systems x pins and the secrets each needs.
//!
//! Scaffold: the node's code lands here (`src/report` tasks). Each `use`
//! line below is one build-order edge for `sherd wave` (src:C); the
//! `expect` turns into a warning once the node uses them, and goes.

#![expect(unused_imports, reason = "scaffold: build-order edges")]

use crate::kind as _;
use crate::pin as _;
use crate::registry as _;
