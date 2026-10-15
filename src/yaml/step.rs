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
    /// These inputs with `key` set to `input`: replaced in place when the
    /// key is already set, else appended.
    #[must_use]
    pub(crate) fn set(mut self, key: &str, input: Input) -> Self {
        match self.0.iter_mut().find(|(k, _)| k == key) {
            Some(slot) => slot.1 = input,
            None => self.0.push((key.to_owned(), input)),
        }
        self
    }

    fn node(&self) -> Option<Node> {
        let entries: Vec<Entry> = self.0.iter().map(input).collect();
        (!entries.is_empty()).then_some(Node::Map(entries))
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

fn input((key, input): &(String, Input)) -> Entry {
    let mut entry = Entry::user(key, Node::Value(input.value.clone()));
    entry.comments.clone_from(&input.comments);
    entry
}

impl Step {
    /// The step as the entries of its sequence item.
    pub(crate) fn entries(&self) -> Vec<Entry> {
        let mut e = Vec::new();
        put(&mut e, "name", self.name.as_deref().map(string));
        put(&mut e, "id", self.id.as_deref().map(string));
        put(&mut e, "if", self.condition.clone().map(condition));
        self.action.lower(&mut e);
        put(&mut e, "env", env(&self.env));
        if let Some(first) = e.first_mut() {
            first.comments.clone_from(&self.comments);
        }
        e
    }
}

impl Action {
    fn lower(&self, e: &mut Vec<Entry>) {
        match self {
            Self::Run(command) => put(e, "run", Some(string(command))),
            Self::Uses(uses) => {
                let mut entry = Entry::fixed("uses", string(&uses.reference));
                entry.trailing.clone_from(&uses.version);
                e.push(entry);
                put(e, "with", uses.with.node());
            }
        }
    }
}

#[cfg(test)]
mod tests;
