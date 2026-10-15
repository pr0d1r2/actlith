//! Tag -> commit SHA, behind a trait so tests use a fake (C, R14).

use super::error::FetchError;

/// Resolves an action's tags against GitHub. The only network seam:
/// `Gh` talks to the API, tests pass a fake (src V4).
pub(crate) trait Resolver {
    /// The newest release tag of `action`; `None` when it has no release,
    /// so the recorded version stays.
    ///
    /// # Errors
    ///
    /// The API call failed.
    fn latest(&self, action: &str) -> Result<Option<String>, FetchError>;

    /// The full commit SHA `tag` of `action` points at (annotated tags
    /// dereferenced).
    ///
    /// # Errors
    ///
    /// The API call failed or the tag does not exist.
    fn sha(&self, action: &str, tag: &str) -> Result<String, FetchError>;
}

/// `tag` when it is safe in an API path: ASCII letters, digits and
/// `.-_+/`, starting with a letter or digit, no `..`.
///
/// # Errors
///
/// [`FetchError::Tag`] naming the action and tag.
#[expect(clippy::unnecessary_wraps, reason = "RED stub")]
pub(crate) fn checked_tag(action: &str, tag: &str) -> Result<(), FetchError> {
    let _ = (action, tag);
    Ok(())
}

/// `got` when it is a full commit SHA: 40 lowercase hex digits (pin V2).
///
/// # Errors
///
/// [`FetchError::Sha`] naming the action, tag and answer.
#[expect(clippy::unnecessary_wraps, reason = "RED stub")]
pub(crate) fn checked_sha(
    action: &str,
    tag: &str,
    got: &str,
) -> Result<String, FetchError> {
    let _ = (action, tag);
    Ok(got.to_owned())
}

#[cfg(test)]
mod tests;
