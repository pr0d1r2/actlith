//! `src/render`: `.actlith` rows -> ordered set of files to write.
//!
//! Scaffold: the node's code lands here (`src/render` tasks). Each `use`
//! line below is one build-order edge for `sherd wave` (src:C); the
//! `expect` turns into a warning once the node uses them, and goes.

#![expect(unused_imports, reason = "scaffold: build-order edges")]

use crate::bots as _;
use crate::build as _;
use crate::bump as _;
use crate::gate as _;
use crate::registry as _;
