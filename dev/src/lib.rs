//! actlith-dev: repository tooling: commit-message and changelog rules,
//! README badges, the third-party notices' crate table and the AGENTS.md
//! step table, each with a drift check. Never published (SPEC.md C).
//! Re-exports only.

mod agents;
mod badges;
mod block;
mod changelog;
mod cli;
mod doctest;
mod facts;
mod hooks;
mod io;
mod message;
mod notices;
mod readme;

pub use cli::run;
