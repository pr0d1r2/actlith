//! `src/fetch`: GitHub API tag -> commit SHA for `actlith pin`; the only
//! network code (V16).
//!
//! Composition only (src C): the resolver seam in `resolver`, the `gh api`
//! subprocess in `gh` (R14), the `## actions` rewrite in `section`, the
//! verb in `pin`, the refusal in `error`. Dead code is `allow`ed, not
//! `expect`ed, until `src/cli` lands (see `src/registry`).
#![cfg_attr(
    not(test),
    allow(dead_code, reason = "used by src/cli in a later wave")
)]

#[allow(
    unused_imports,
    reason = "build-order edge for sherd wave (src C)"
)]
use crate::pin as _;
#[allow(
    unused_imports,
    reason = "build-order edge for sherd wave (src C)"
)]
use crate::registry as _;

mod error;
mod gh;
mod pin;
mod resolver;
mod section;

#[cfg_attr(
    not(test),
    expect(unused_imports, reason = "used by src/cli in a later wave")
)]
pub(crate) use error::FetchError;
pub(crate) use gh::Gh;
#[cfg_attr(
    not(test),
    expect(unused_imports, reason = "used by src/cli in a later wave")
)]
pub(crate) use pin::pin;
pub(crate) use resolver::Resolver;
