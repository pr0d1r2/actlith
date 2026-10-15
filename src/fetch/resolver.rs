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
pub(crate) fn checked_tag(action: &str, tag: &str) -> Result<(), FetchError> {
    let ok = |c: char| c.is_ascii_alphanumeric() || ".-_+/".contains(c);
    let lead = tag
        .chars()
        .next()
        .is_some_and(|c| c.is_ascii_alphanumeric());
    if lead && tag.chars().all(ok) && !tag.contains("..") {
        return Ok(());
    }
    Err(FetchError::Tag {
        action: action.to_owned(),
        tag: tag.to_owned(),
    })
}

/// `got` when it is a full commit SHA: 40 lowercase hex digits (pin V2).
///
/// # Errors
///
/// [`FetchError::Sha`] naming the action, tag and answer.
pub(crate) fn checked_sha(
    action: &str,
    tag: &str,
    got: &str,
) -> Result<String, FetchError> {
    let got = got.trim();
    let hex = |c: char| c.is_ascii_digit() || ('a'..='f').contains(&c);
    if got.len() == 40 && got.chars().all(hex) {
        return Ok(got.to_owned());
    }
    Err(FetchError::Sha {
        action: action.to_owned(),
        tag: tag.to_owned(),
        got: got.to_owned(),
    })
}

#[cfg(test)]
mod tests;
