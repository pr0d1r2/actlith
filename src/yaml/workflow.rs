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

impl Scope {
    fn key(self) -> &'static str {
        match self {
            Self::Actions => "actions",
            Self::Attestations => "attestations",
            Self::Checks => "checks",
            Self::Contents => "contents",
            Self::Deployments => "deployments",
            Self::IdToken => "id-token",
            Self::Issues => "issues",
            Self::Packages => "packages",
            Self::Pages => "pages",
            Self::PullRequests => "pull-requests",
            Self::SecurityEvents => "security-events",
            Self::Statuses => "statuses",
        }
    }
}

impl Level {
    fn word(self) -> &'static str {
        match self {
            Self::Read => "read",
            Self::Write => "write",
            Self::None => "none",
        }
    }
}

impl Permissions {
    /// The `permissions:` value.
    pub(crate) fn node(&self) -> Node {
        let entries = self.0.iter().map(|(scope, level)| {
            Entry::fixed(scope.key(), Node::Value(word(level.word())))
        });
        Node::Map(entries.collect())
    }
}

impl Workflow {
    /// The top-level entries.
    pub(crate) fn entries(&self) -> Vec<Entry> {
        let mut e = vec![
            Entry::fixed("name", string(&self.name)),
            Entry::fixed("on", self.on.node()),
        ];
        let concurrency = self.concurrency.as_ref().map(Concurrency::node);
        put(&mut e, "concurrency", concurrency);
        put(&mut e, "permissions", Some(self.permissions.node()));
        put(&mut e, "env", env(&self.env));
        let jobs = self.jobs.iter().map(|(id, job)| job.entry(id));
        put(&mut e, "jobs", Some(Node::Map(jobs.collect())));
        e
    }
}

impl Trigger {
    fn node(&self) -> Node {
        let mut e = Vec::new();
        put(&mut e, "push", self.push.as_ref().map(Push::node));
        put(
            &mut e,
            "pull_request",
            self.pull_request.then_some(Node::Null),
        );
        put(&mut e, "schedule", crons(&self.schedule));
        let dispatch = self.workflow_dispatch.then_some(Node::Null);
        put(&mut e, "workflow_dispatch", dispatch);
        Node::Map(e)
    }
}

impl Push {
    fn node(&self) -> Node {
        if self.branches.is_empty() {
            return Node::Null;
        }
        Node::Map(vec![Entry::fixed("branches", list(&self.branches))])
    }
}

impl Concurrency {
    fn node(&self) -> Node {
        Node::Map(vec![
            Entry::fixed("group", Node::Value(Value::Text(self.group.clone()))),
            Entry::fixed(
                "cancel-in-progress",
                Node::Value(Value::Bool(self.cancel_in_progress)),
            ),
        ])
    }
}

fn crons(crons: &[String]) -> Option<Node> {
    let items = crons
        .iter()
        .map(|c| vec![Entry::fixed("cron", string(c))])
        .collect::<Vec<_>>();
    (!items.is_empty()).then_some(Node::Seq(items))
}

fn word(w: &'static str) -> Value {
    Value::Text(Text::lit(w))
}

/// Appends `key: node` when there is a node.
pub(crate) fn put(e: &mut Vec<Entry>, key: &'static str, node: Option<Node>) {
    if let Some(node) = node {
        e.push(Entry::fixed(key, node));
    }
}

/// A literal string node.
pub(crate) fn string(s: &str) -> Node {
    Node::Value(Value::Text(Text::lit(s)))
}

/// A `[a, b]` list of literal strings.
pub(crate) fn list(items: &[String]) -> Node {
    Node::List(items.iter().map(|s| Text::lit(s)).collect())
}

/// The `env:` value, or none for no variables.
pub(crate) fn env(env: &Env) -> Option<Node> {
    let entries = env
        .iter()
        .map(|(k, v)| Entry::user(k, Node::Value(Value::Text(v.clone()))));
    let entries: Vec<Entry> = entries.collect();
    (!entries.is_empty()).then_some(Node::Map(entries))
}

#[cfg(test)]
mod tests;
