//! The one runner table (V15): each nix system, beside the label of the
//! GitHub-hosted runner it runs on. No runner label is spelled anywhere else. The systems
//! themselves, their names and the defaults are pipelith's (src/core).

use std::fmt;

use crate::core::system as neutral;
pub(crate) use neutral::{DEFAULT_SYSTEMS, System};

#[cfg(test)]
mod tests;

/// Why a system name cannot be used. Every system has a runner (V53),
/// so a known one never can be refused.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum SystemError {
    /// The name is not a nix system in the table.
    Unknown(String),
}

/// Parse a nix system name, exactly as nix spells it.
pub(crate) fn parse_system(name: &str) -> Result<System, SystemError> {
    neutral::parse_system(name)
        .map_err(|_| SystemError::Unknown(name.to_owned()))
}

/// The table (V15): the GitHub-hosted runner label of each system, one
/// arm per system, so the compiler proves it covers them all (V53).
/// `x86_64-darwin` runs on `macos-15-intel`, GitHub's last Intel image,
/// available until August 2027 (R18); a `## runners` row moves it to the
/// repository's own Mac.
pub(crate) const fn runner(system: System) -> &'static str {
    match system {
        System::X86_64Linux => "ubuntu-latest",
        System::Aarch64Linux => "ubuntu-24.04-arm",
        System::Aarch64Darwin => "macos-latest",
        System::X86_64Darwin => "macos-15-intel",
    }
}

impl fmt::Display for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self::Unknown(name) = self;
        write!(f, "unknown system `{name}`; known: ")?;
        let names: Vec<&str> = neutral::ALL.iter().map(|s| s.name()).collect();
        f.write_str(&names.join(", "))
    }
}

impl std::error::Error for SystemError {}
