//! A job and its matrix, lowered in a fixed key order (V5).

use std::collections::BTreeMap;

use super::node::{Entry, Node, Value};
use super::step::Step;
use super::text::{Comment, Expr, Text};
use super::workflow::{Env, Permissions, env, list, put, string};

/// A job. Keys are written as `name`, `needs`, `if`, `runs-on`,
/// `permissions`, `strategy`, `timeout-minutes`, `env`, `steps`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Job {
    /// Comments above the job id.
    pub(crate) comments: Vec<Comment>,
    pub(crate) name: Option<String>,
    /// Ids of the jobs this one waits for.
    pub(crate) needs: Vec<String>,
    /// The `if:` condition.
    pub(crate) condition: Option<Expr>,
    pub(crate) runs_on: Text,
    /// Permissions widened for this job only.
    pub(crate) permissions: Option<Permissions>,
    pub(crate) matrix: Option<Matrix>,
    pub(crate) timeout_minutes: u32,
    pub(crate) env: Env,
    pub(crate) steps: Vec<Step>,
}

/// A matrix variable name: never `include` or `exclude`, which the
/// matrix itself uses.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct AxisName(String);

impl AxisName {
    /// The name `name`; `None` for `include` and `exclude`.
    pub(crate) fn new(name: &str) -> Option<Self> {
        let taken = name == "include" || name == "exclude";
        (!taken).then(|| Self(name.to_owned()))
    }
}

/// One matrix variable: its values, and the comments above it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Axis {
    pub(crate) comments: Vec<Comment>,
    pub(crate) values: Vec<String>,
}

/// `strategy:` with `fail-fast` and a `matrix` of variables (in name
/// order) and `include` combinations (each in key order).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Matrix {
    pub(crate) fail_fast: bool,
    pub(crate) axes: BTreeMap<AxisName, Axis>,
    pub(crate) include: Vec<BTreeMap<String, String>>,
}

impl Job {
    /// The job's entry under `jobs:`.
    pub(crate) fn entry(&self, id: &str) -> Entry {
        let mut entry = Entry::user(id, self.node());
        entry.comments.clone_from(&self.comments);
        entry
    }

    fn node(&self) -> Node {
        let mut e = self.head();
        let permissions = self.permissions.as_ref().map(Permissions::node);
        put(&mut e, "permissions", permissions);
        put(&mut e, "strategy", self.matrix.as_ref().map(Matrix::node));
        let timeout = Value::Int(u64::from(self.timeout_minutes));
        put(&mut e, "timeout-minutes", Some(Node::Value(timeout)));
        put(&mut e, "env", env(&self.env));
        let steps = self.steps.iter().map(Step::entries).collect();
        put(&mut e, "steps", Some(Node::Seq(steps)));
        Node::Map(e)
    }

    /// `name`, `needs`, `if`, `runs-on`.
    fn head(&self) -> Vec<Entry> {
        let mut e = Vec::new();
        put(&mut e, "name", self.name.as_deref().map(string));
        let needs = (!self.needs.is_empty()).then(|| list(&self.needs));
        put(&mut e, "needs", needs);
        put(&mut e, "if", self.condition.clone().map(condition));
        put(&mut e, "runs-on", Some(text(self.runs_on.clone())));
        e
    }
}

impl Matrix {
    fn node(&self) -> Node {
        let mut m: Vec<Entry> = self.axes.iter().map(axis).collect();
        let include = self.include.iter().map(combination).collect::<Vec<_>>();
        put(
            &mut m,
            "include",
            (!include.is_empty()).then_some(Node::Seq(include)),
        );
        Node::Map(vec![
            Entry::fixed("fail-fast", Node::Value(Value::Bool(self.fail_fast))),
            Entry::fixed("matrix", Node::Map(m)),
        ])
    }
}

fn axis((name, axis): (&AxisName, &Axis)) -> Entry {
    let mut entry = Entry::user(&name.0, list(&axis.values));
    entry.comments.clone_from(&axis.comments);
    entry
}

fn combination(values: &BTreeMap<String, String>) -> Vec<Entry> {
    values
        .iter()
        .map(|(k, v)| Entry::user(k, string(v)))
        .collect()
}

/// An `if:` value.
pub(crate) fn condition(expr: Expr) -> Node {
    text(Text::expr(expr))
}

fn text(t: Text) -> Node {
    Node::Value(Value::Text(t))
}

#[cfg(test)]
mod tests;
