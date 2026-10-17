use std::collections::BTreeMap;

use super::{Level, Permissions, Push, Scope, Trigger, Workflow};
use crate::core::yaml::document;
use crate::yaml::GITHUB;
use crate::yaml::{Env, Text};

fn bare(on: Trigger) -> Workflow {
    Workflow {
        comments: Vec::new(),
        name: "w".to_owned(),
        on,
        concurrency: None,
        permissions: Permissions::default(),
        env: Env::new(),
        jobs: BTreeMap::new(),
    }
}

fn out(w: &Workflow) -> String {
    document(&GITHUB, &w.comments, &w.entries())
}

#[test]
fn no_triggers_no_jobs_are_empty_maps() {
    let want = "---\nname: w\n\non: {}\n\npermissions: {}\n\njobs: {}\n";
    assert_eq!(out(&bare(Trigger::default())), want);
}

#[test]
fn push_without_branches_is_every_branch() {
    let on = Trigger {
        push: Some(Push::default()),
        ..Trigger::default()
    };
    assert!(out(&bare(on)).contains("on:\n  push:\n\n"));
}

#[test]
fn triggers_keep_fixed_order() {
    let on = Trigger {
        push: Some(Push::default()),
        pull_request: true,
        schedule: vec!["0 1 * * *".to_owned(), "0 2 * * *".to_owned()],
        workflow_dispatch: true,
    };
    let want = "on:\n  push:\n  pull_request:\n  schedule:\n    \
                - cron: '0 1 * * *'\n    - cron: '0 2 * * *'\n  \
                workflow_dispatch:\n";
    assert!(out(&bare(on)).contains(want));
}

const SCOPES: [Scope; 12] = [
    Scope::Statuses,
    Scope::SecurityEvents,
    Scope::PullRequests,
    Scope::Pages,
    Scope::Packages,
    Scope::Issues,
    Scope::IdToken,
    Scope::Deployments,
    Scope::Contents,
    Scope::Checks,
    Scope::Attestations,
    Scope::Actions,
];

#[test]
fn permission_scopes_in_key_order() {
    let mut w = bare(Trigger::default());
    w.permissions = Permissions(SCOPES.map(|s| (s, Level::None)).into());
    let keys: Vec<String> = out(&w)
        .lines()
        .filter_map(|l| l.strip_suffix(": none"))
        .map(|l| l.trim().to_owned())
        .collect();
    let mut sorted = keys.clone();
    sorted.sort();
    assert_eq!(keys.len(), 12);
    assert_eq!(keys, sorted);
}

#[test]
fn levels_and_env_render() {
    let mut w = bare(Trigger::default());
    w.permissions = Permissions(BTreeMap::from([
        (Scope::IdToken, Level::Write),
        (Scope::Contents, Level::Read),
    ]));
    w.env = Env::from([
        ("B".to_owned(), Text::lit("2")),
        ("A".to_owned(), Text::lit("x")),
    ]);
    let o = out(&w);
    assert!(o.contains("permissions:\n  contents: read\n  id-token: write\n"));
    assert!(o.contains("env:\n  A: x\n  B: '2'\n"));
}
