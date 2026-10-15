//! The workflow, its triggers, concurrency and permissions, and their
//! lowering to the YAML tree in a fixed key order (V5).

use std::collections::BTreeMap;

use super::job::Job;
use super::node::{Entry, Node, Value};
use super::text::{Comment, Text};

/// Environment variables, written in name order.
pub(crate) type Env = BTreeMap<String, Text>;

/// A workflow file. Keys are written as `name`, `on`, `concurrency`,
/// `permissions`, `env`, `jobs`; jobs in id order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Workflow {
    /// Comments under `---`, above `name`.
    pub(crate) comments: Vec<Comment>,
    pub(crate) name: String,
    pub(crate) on: Trigger,
    pub(crate) concurrency: Option<Concurrency>,
    pub(crate) permissions: Permissions,
    pub(crate) env: Env,
    pub(crate) jobs: BTreeMap<String, Job>,
}

/// The `on:` events, written as `push`, `pull_request`, `schedule`,
/// `workflow_dispatch`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Trigger {
    pub(crate) push: Option<Push>,
    pub(crate) pull_request: bool,
    /// Cron lines; none means no `schedule`.
    pub(crate) schedule: Vec<String>,
    pub(crate) workflow_dispatch: bool,
}

/// The `push` event; no branches means every branch.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Push {
    pub(crate) branches: Vec<String>,
}

/// The `concurrency:` block.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Concurrency {
    pub(crate) group: Text,
    pub(crate) cancel_in_progress: bool,
}

/// A `GITHUB_TOKEN` permission scope, in key order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Scope {
    Actions,
    Attestations,
    Checks,
    Contents,
    Deployments,
    IdToken,
    Issues,
    Packages,
    Pages,
    PullRequests,
    SecurityEvents,
    Statuses,
}

/// A permission level.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Level {
    Read,
    Write,
    None,
}

/// Permissions, in scope order; none at all is written `{}`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Permissions(pub(crate) BTreeMap<Scope, Level>);

impl Workflow {
    /// The top-level entries. RED: signature only.
    pub(crate) fn entries(&self) -> Vec<Entry> {
        Vec::new()
    }
}

/// Appends `key: node` when there is a node. RED: signature only.
pub(crate) fn put(
    _e: &mut Vec<Entry>,
    _key: &'static str,
    _node: Option<Node>,
) {
}

/// A literal string node. RED: signature only.
pub(crate) fn string(_s: &str) -> Node {
    Node::Null
}

/// A `[a, b]` list of literal strings. RED: signature only.
pub(crate) fn list(_items: &[String]) -> Node {
    Node::Null
}

/// The `env:` value. RED: signature only.
pub(crate) fn env(_env: &Env) -> Option<Node> {
    None
}

#[cfg(test)]
mod tests;
