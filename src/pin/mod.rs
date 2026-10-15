//! `src/pin`: pin table: every `uses:` resolves to a full commit SHA with
//! its version (V2).
//!
//! Composition only (src C): the built-in table and the `## actions`
//! override merge live in `table`, the rendered reference in `uses`, the
//! refusal in `error`.

#[allow(
    unused_imports,
    reason = "build-order edge for sherd wave (src C)"
)]
use crate::registry as _;
#[allow(
    unused_imports,
    reason = "build-order edge for sherd wave (src C)"
)]
use crate::yaml as _;

mod error;
mod table;
mod uses;

pub(crate) use error::PinError;
pub(crate) use table::PinTable;
pub(crate) use uses::Uses;
