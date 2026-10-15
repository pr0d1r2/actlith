//! A step and its `with:` inputs, lowered in a fixed key order (V5).

use super::job::condition;
use super::node::{Entry, Node, Value};
use super::text::{Comment, Expr};
use super::workflow::{Env, env, put, string};

/// A step. Keys are written as `name`, `id`, `if`, `uses` or `run`,
/// `with`, `env`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Step {
    /// Comments above the step.
    pub(crate) comments: Vec<Comment>,
    pub(crate) name: Option<String>,
    pub(crate) id: Option<String>,
    /// The `if:` condition.
    pub(crate) condition: Option<Expr>,
    pub(crate) action: Action,
    pub(crate) env: Env,
}

/// What a step does.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Action {
    /// An action, with its inputs.
    Uses(Uses),
    /// One shell command (`docs/KINDS.md`: every `run:` is one line).
    Run(String),
}

/// `uses: <reference>  # <version>` and its `with:` inputs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Uses {
    /// `owner/repo@sha`.
    pub(crate) reference: String,
    /// The comment after the reference, the version the SHA pins.
    pub(crate) version: Option<Comment>,
    pub(crate) with: With,
}

/// One `with:` input: its value and the comments above it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Input {
    pub(crate) comments: Vec<Comment>,
    pub(crate) value: Value,
}

/// `with:` inputs in the order they were set; a key is never repeated.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct With(Vec<(String, Input)>);

impl With {
    /// These inputs with `key` set to `input`. RED: signature only.
    #[must_use]
    pub(crate) fn set(self, _key: &str, _input: Input) -> Self {
        self
    }
}

impl Input {
    /// An input of `value` with no comments.
    pub(crate) fn new(value: Value) -> Self {
        Self {
            comments: Vec::new(),
            value,
        }
    }
}

impl Step {
    /// The step as the entries of its sequence item. RED: signature only.
    pub(crate) fn entries(&self) -> Vec<Entry> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests;
