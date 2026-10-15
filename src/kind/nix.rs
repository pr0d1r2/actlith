//! The install-nix step, trusting the row's cache as a substituter.

use super::cache::Cache;
use super::error::KindError;
use super::step::uses;
use crate::pin::PinTable;
use crate::yaml::{Input, Step, Text, Value, With};

/// The `cachix/install-nix-action` step, pinned; with a cache, its
/// `extra-substituters` and `extra-trusted-public-keys`.
pub(crate) fn install_nix(
    pins: &PinTable,
    cache: Option<&Cache>,
) -> Result<Step, KindError> {
    let with = cache.map_or_else(With::default, |c| {
        let config = Value::Text(Text::lit(&c.nix_config()));
        With::default().set("extra_nix_config", Input::new(config))
    });
    uses(pins, "cachix/install-nix-action", with)
}

#[cfg(test)]
mod tests;
