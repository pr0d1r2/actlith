//! The read netrc of a private https cache (V66). RED stub: nothing yet.

use crate::core::cache::BinaryCache;
use crate::kind::{KindError, Secret};
use crate::yaml::{Step, Text};

/// The seeded script that writes the read netrc (V66).
pub(crate) const READ_SCRIPT: &str = "scripts/actlith/read-netrc.sh";

/// The secret holding `cache`'s read netrc; `None` without one.
pub(crate) fn read_secret(_cache: &BinaryCache) -> Option<Secret> {
    None
}

/// The step writing `cache`'s read netrc; `None` without one.
#[expect(clippy::unnecessary_wraps, reason = "RED stub: T85 builds the step")]
pub(crate) fn read_netrc(
    _line: usize,
    _cache: &BinaryCache,
) -> Result<Option<Step>, KindError> {
    Ok(None)
}

/// `text` with the `netrc-file` line for `cache`'s read netrc, if any.
pub(crate) fn netrc_file(text: Text, _cache: &BinaryCache) -> Text {
    text
}

#[cfg(test)]
mod tests;
