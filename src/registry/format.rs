//! The `format N` line every `.actlith` starts with (registry V11).

use super::error::RegistryError;

/// The newest format this actlith reads.
pub(crate) const SUPPORTED: u32 = 1;

/// The first content line, `(line, text)`, is `format 1`.
///
/// # Errors
///
/// No format line, a malformed one, or a version newer than
/// [`SUPPORTED`], named next to it (registry V11).
pub(crate) fn check(first: Option<(usize, &str)>) -> Result<(), RegistryError> {
    let _ = (first, SUPPORTED);
    Err(RegistryError::default())
}

#[cfg(test)]
mod tests;
