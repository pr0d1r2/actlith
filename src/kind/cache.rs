//! The row's cachix cache: its substituter URL and public key, so
//! install-nix trusts it (`docs/KINDS.md` common parts).

use super::error::KindError;

/// Public keys of the caches actlith knows, `<name>.cachix.org-1:<key>`.
/// A row naming any other cache is refused: a key is never guessed.
const KEYS: [&str; 1] =
    ["pr0d1r2.cachix.org-1:NfWjbhgAj41byXhCKiaE+av3Vnphm1fTezHXEGsiQIM="];

/// A cachix cache with a known public key.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Cache {
    name: String,
    key: &'static str,
}

impl Cache {
    /// The cache `name` of the row at `line`, or an error naming it.
    pub(crate) fn parse(line: usize, name: &str) -> Result<Self, KindError> {
        let prefix = format!("{name}.cachix.org-1:");
        KEYS.into_iter()
            .find(|k| k.starts_with(&prefix))
            .map(|key| Self {
                name: name.to_owned(),
                key,
            })
            .ok_or_else(|| unknown(line, name))
    }

    /// The cache name, as cachix-action's `name:` input.
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    /// The `extra_nix_config` lines that trust the cache.
    pub(crate) fn nix_config(&self) -> String {
        format!(
            "extra-substituters = https://{}.cachix.org\n\
             extra-trusted-public-keys = {}\n",
            self.name, self.key
        )
    }
}

fn unknown(line: usize, name: &str) -> KindError {
    let names: Vec<&str> =
        KEYS.iter().filter_map(|k| k.split('.').next()).collect();
    KindError::row(
        line,
        format!("cache `{name}` has no known public key"),
        format!("use one of: {}, or `-` for no cache", names.join(", ")),
    )
}

#[cfg(test)]
mod tests;
