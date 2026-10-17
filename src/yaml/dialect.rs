//! GitHub's expression syntax for pipelith's writer (V5): `${{ inner }}`,
//! and a literal `${{` written so that GitHub reads it back as text.

use crate::core::yaml::Dialect;

/// GitHub Actions expressions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Github;

/// The dialect every workflow actlith writes is in.
pub(crate) const GITHUB: Github = Github;

impl Dialect for Github {
    /// RED stub: refuses every expression.
    fn expr(&self, _inner: &str) -> Option<String> {
        None
    }

    /// RED stub: leaves literals as they are.
    fn escape(&self, lit: &str) -> String {
        lit.to_owned()
    }
}

#[cfg(test)]
mod tests;
