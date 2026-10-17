//! GitHub's expression syntax for pipelith's writer (V5): `${{ inner }}`,
//! and a literal `${{` written so that GitHub reads it back as text.

use crate::core::yaml::Dialect;

/// GitHub Actions expressions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Github;

/// The dialect every workflow actlith writes is in.
pub(crate) const GITHUB: Github = Github;

impl Dialect for Github {
    /// `${{ inner }}`; `None` when `inner` holds `${{` or `}}`, which would
    /// nest or end the expression early: an injected expression (V5).
    fn expr(&self, inner: &str) -> Option<String> {
        let broken = inner.contains("${{") || inner.contains("}}");
        (!broken).then(|| format!("${{{{ {inner} }}}}"))
    }

    /// A literal `${{` becomes an expression that yields `${{`, so literal
    /// text never opens one.
    fn escape(&self, lit: &str) -> String {
        lit.replace("${{", OPEN_ESCAPED)
    }
}

/// The literal `${{`, written as an expression that yields it.
const OPEN_ESCAPED: &str = "${{ '${{' }}";

#[cfg(test)]
mod tests;
