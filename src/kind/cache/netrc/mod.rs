//! `src/kind/cache/netrc`: the netrc rules every cache step that hands
//! one to nix or curl shares (V65), and the read netrc of a private https
//! cache (`read`, V66).
//!
//! Composition only (src C).

mod read;

pub(crate) use read::{READ_SCRIPT, netrc_file, read_netrc, read_secret};
