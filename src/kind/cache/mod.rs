//! `src/kind/cache`: the GitHub steps for the cache a row trusts, from
//! pipelith's [`BinaryCache`](crate::core::cache::BinaryCache) only
//! (C): install-nix reading it (`nix`, V60), the cachix step (`cachix`),
//! the https push (`push`, V61, V62) and the secrets they read
//! (`secrets`).
//!
//! Composition only (src C).

// Build-order edges for `sherd wave` (src C), one per line.
#[expect(unused_imports, reason = "build-order edge")]
use crate::pin as _;
#[expect(unused_imports, reason = "build-order edge")]
use crate::yaml as _;

mod cachix;
mod nix;
mod push;
mod secrets;

pub(crate) use cachix::{
    CachixPush, PUSH_FILTER, SKIP_FORK_PR, SKIP_UNLESS_MAIN, cachix,
};
pub(crate) use nix::install_nix;
pub(crate) use push::{PUSH_SCRIPT, https_push, push_https, url};
pub(crate) use secrets::{push_secret, secrets};
