//! actlith-dev: repository tooling: commit-message and changelog rules,
//! README badges, the third-party notices' crate table and the AGENTS.md
//! step table, each with a drift check. Never published (SPEC.md C).
//! Re-exports only.

#[allow(dead_code, reason = "RED: wired in GREEN")]
mod agents;
#[allow(dead_code, reason = "RED: wired in GREEN")]
mod badges;
#[allow(dead_code, reason = "RED: wired in GREEN")]
mod block;
mod changelog;
mod cli;
#[allow(dead_code, reason = "RED: wired in GREEN")]
mod facts;
mod hooks;
mod io;
mod message;
#[allow(dead_code, reason = "RED: wired in GREEN")]
mod notices;
mod readme;

pub use cli::run;
