//! The one system table (V15): nix system name and GitHub-hosted runner
//! label, side by side. No runner label is spelled anywhere else.
#![cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "used by src/render and the kind nodes in a later wave"
    )
)]

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

/// One row: the system, its nix name, its hosted runner label if any.
type Row = (System, &'static str, Option<&'static str>);

/// The table (V15). `x86_64-darwin` has no hosted runner: it is built
/// and verified by hand on an Intel Mac (`src/registry` V10).
const TABLE: [Row; 4] = [
    (System::X86_64Linux, "x86_64-linux", Some("ubuntu-latest")),
    (
        System::Aarch64Linux,
        "aarch64-linux",
        Some("ubuntu-24.04-arm"),
    ),
    (
        System::Aarch64Darwin,
        "aarch64-darwin",
        Some("macos-latest"),
    ),
    (System::X86_64Darwin, "x86_64-darwin", None),
];

/// Systems a kind runs on when its row says `-`.
pub(crate) const DEFAULT_SYSTEMS: [System; 3] = [
    System::X86_64Linux,
    System::Aarch64Linux,
    System::Aarch64Darwin,
];

/// The table row of a system.
fn row(system: System) -> Row {
    TABLE
        .into_iter()
        .find(|&(s, _, _)| s == system)
        .unwrap_or((system, "", None))
}

/// Parse a nix system name, exactly as nix spells it.
pub(crate) fn parse_system(name: &str) -> Result<System, SystemError> {
    TABLE
        .into_iter()
        .find(|&(_, n, _)| n == name)
        .map(|(s, _, _)| s)
        .ok_or_else(|| SystemError::Unknown(name.to_owned()))
}

/// The GitHub-hosted runner label for a system; refused when GitHub
/// hosts none.
pub(crate) fn runner(system: System) -> Result<&'static str, SystemError> {
    row(system).2.ok_or(SystemError::NoRunner(system))
}

impl fmt::Display for System {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(row(*self).1)
    }
}

impl fmt::Display for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown(name) => {
                write!(f, "unknown system `{name}`; known: ")?;
                let names: Vec<&str> = TABLE.iter().map(|r| r.1).collect();
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
