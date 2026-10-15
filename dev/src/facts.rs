//! The facts the README badges and the LLM disclaimer quote, each read from
//! the file that owns it. Pure functions over text. RED: signatures only.

/// A `key = "value"` line's value, from a TOML file read as text.
#[must_use]
pub(crate) fn quoted(_toml: &str, _key: &str) -> Option<String> {
    None
}

/// How many entries the `[dependencies]` table of a Cargo.toml holds;
/// `None` when it has no such table.
#[must_use]
pub(crate) fn dependencies(_toml: &str) -> Option<usize> {
    None
}

/// The coverage floor `.coverage` records, trailing zeros dropped.
#[must_use]
pub(crate) fn floor(_coverage: &str) -> Option<String> {
    None
}

/// Rows of a spec table across `specs`: `prefix`, digits, then `sep`.
#[must_use]
pub(crate) fn rows(_specs: &[String], _prefix: char, _sep: char) -> usize {
    0
}

/// The nixpkgs the flake builds with, from `flake.lock`.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Nixpkgs {
    /// The release series, `26.05` from `nixos-26.05`.
    pub(crate) series: String,
    /// The locked commit, 7 characters.
    pub(crate) rev: String,
    /// The commit date, `YYYY-MM-DD`.
    pub(crate) day: String,
}

/// The `nixpkgs` node of a flake lock.
#[must_use]
pub(crate) fn nixpkgs(_lock: &str) -> Option<Nixpkgs> {
    None
}

/// `YYYY-MM-DD` for a Unix time, in UTC; `None` before 1970.
#[must_use]
pub(crate) fn day(_secs: i64) -> Option<String> {
    None
}

/// A platform badge: architecture and operating system.
pub(crate) type Platform = (&'static str, &'static str);

/// The platforms CI's matrix builds on, in badge order.
///
/// # Errors
///
/// A runner this table does not know, named.
pub(crate) fn platforms(_ci: &str) -> Result<Vec<Platform>, String> {
    Err(String::new())
}

#[cfg(test)]
mod tests;
