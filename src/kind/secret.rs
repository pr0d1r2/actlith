//! A secret a workflow reads (V14): every kind declares each one, so
//! `report` can list them per workflow.

use crate::yaml::Expr;

/// A repository secret and what the workflow reads it for.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Secret {
    name: String,
    purpose: &'static str,
    expr: Expr,
}

impl Secret {
    /// The secret `name`; `None` unless the name is letters, digits and
    /// `_`, not starting with a digit (GitHub's secret name rule).
    pub(crate) fn new(name: &str, purpose: &'static str) -> Option<Self> {
        let _ = (name, purpose);
        None
    }

    /// `CACHIX_AUTH_TOKEN`, read by a cachix step that pushes (`.:I`).
    pub(crate) fn cachix() -> Option<Self> {
        Self::new("CACHIX_AUTH_TOKEN", "cachix push")
    }

    /// The secret's name.
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    /// What the workflow reads it for.
    pub(crate) fn purpose(&self) -> &'static str {
        self.purpose
    }

    /// `secrets.<name>`, for a `with:` or `env:` value.
    pub(crate) fn expr(&self) -> Expr {
        self.expr.clone()
    }
}

#[cfg(test)]
mod tests;
