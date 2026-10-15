//! `src/cli`: clap verbs, flags and exit codes over the verb nodes.
//!
//! Scaffold: the node's code lands here (`src/cli` tasks). Each `use`
//! line below is one build-order edge for `sherd wave` (src:C); the
//! `expect` turns into a warning once the node uses them, and goes.

#![expect(unused_imports, reason = "scaffold: build-order edges")]

use crate::check as _;
use crate::detect as _;
use crate::fetch as _;
use crate::r#gen as _;
use crate::import as _;
use crate::render as _;
use crate::report as _;
