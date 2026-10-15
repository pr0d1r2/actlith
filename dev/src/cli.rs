//! Argument dispatch for `actlith-dev`.
//!
//! ```text
//! actlith-dev commit-msg [FILE]   the commit message style rule
//! actlith-dev changelog [FILE]    a behaviour change adds an entry
//! ```
//!
//! Exit 0 clean, 1 a broken rule, 2 usage or I/O. RED: signatures only.

use std::path::Path;

/// Run one verb against the repository at `repo`; the exit code.
#[must_use]
pub fn run(_args: &[String], _repo: &Path) -> u8 {
    0
}

#[cfg(test)]
mod tests;
