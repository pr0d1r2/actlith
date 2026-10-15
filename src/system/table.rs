//! The one system table (V15): nix system name and GitHub-hosted runner
//! label, side by side. No runner label is spelled anywhere else.

use std::fmt;

#[cfg(test)]
mod tests;

/// A nix system actlith knows by name.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum System {
    /// `x86_64-linux`.
    X86_64Linux,
    /// `aarch64-linux`.
    Aarch64Linux,
    /// `aarch64-darwin`.
    Aarch64Darwin,
    /// `x86_64-darwin`: known, but has no hosted runner.
    X86_64Darwin,
}

/// Why a system name or system cannot be used.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum SystemError {
    /// The name is not a nix system in the table.
    Unknown(String),
    /// The system is known but GitHub hosts no runner for it.
    NoRunner(System),
}

/// Systems a kind runs on when its row says `-`.
pub(crate) const DEFAULT_SYSTEMS: [System; 3] = [System::X86_64Darwin; 3];

/// Parse a nix system name.
pub(crate) fn parse_system(name: &str) -> Result<System, SystemError> {
    Err(SystemError::Unknown(name.to_owned()))
}

/// The GitHub-hosted runner label for a system.
pub(crate) fn runner(system: System) -> Result<&'static str, SystemError> {
    Err(SystemError::NoRunner(system))
}

impl fmt::Display for System {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("")
    }
}

impl fmt::Display for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("")
    }
}

impl std::error::Error for SystemError {}
