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

impl AxisName {
    /// The name `name`; `None` for `include` and `exclude`. RED: stub.
    pub(crate) fn new(name: &str) -> Option<Self> {
        Some(Self(name.to_owned()))
    }
}

impl Job {
    /// The job's entry under `jobs:`. RED: signature only.
    pub(crate) fn entry(&self, id: &str) -> Entry {
        Entry::user(id, Node::Null)
    }
}

/// An `if:` value. RED: signature only.
pub(crate) fn condition(_expr: Expr) -> Node {
    Node::Null
}

#[cfg(test)]
mod tests;
