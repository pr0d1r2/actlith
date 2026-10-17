//! The one runner table (V15): each nix system GitHub hosts, beside its
//! runner label. No runner label is spelled anywhere else. The systems
//! themselves, their names and the defaults are pipelith's (src/core).

use std::fmt;

use crate::core::system as neutral;
pub(crate) use neutral::{DEFAULT_SYSTEMS, System};

#[cfg(test)]
mod tests;

/// Why a system name or system cannot be used.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum SystemError {
    /// The name is not a nix system in the table.
    Unknown(String),
    /// The system is known but GitHub hosts no runner for it.
    NoRunner(System),
}

/// The table (V15). `x86_64-darwin` has no row: it has no hosted runner
/// and is built and verified by hand on an Intel Mac (`src/registry` V10).
const RUNNERS: [(System, &str); 3] = [
    (System::X86_64Linux, "ubuntu-latest"),
    (System::Aarch64Linux, "ubuntu-24.04-arm"),
    (System::Aarch64Darwin, "macos-latest"),
];

/// Parse a nix system name, exactly as nix spells it.
pub(crate) fn parse_system(name: &str) -> Result<System, SystemError> {
    neutral::parse_system(name)
        .map_err(|_| SystemError::Unknown(name.to_owned()))
}

/// The GitHub-hosted runner label for a system; refused when GitHub
/// hosts none.
pub(crate) fn runner(system: System) -> Result<&'static str, SystemError> {
    RUNNERS
        .into_iter()
        .find(|&(s, _)| s == system)
        .map(|(_, label)| label)
        .ok_or(SystemError::NoRunner(system))
}

impl fmt::Display for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown(name) => {
                write!(f, "unknown system `{name}`; known: ")?;
                let names: Vec<&str> =
                    neutral::ALL.iter().map(|s| s.name()).collect();
                f.write_str(&names.join(", "))
            }
            Self::NoRunner(system) => write!(
                f,
                "system `{system}` has no GitHub-hosted runner; build and \
                 verify it by hand on an Intel Mac"
            ),
        }
    }
}

impl std::error::Error for SystemError {}
