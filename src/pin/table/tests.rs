//! Built-in pins, override merge, SHA check, missing pin (T15, V2).

use std::collections::BTreeMap;

use crate::pin::{PinError, PinTable, Uses};
use crate::registry::{ActionPin, parse};
use crate::yaml::{
    Action, Job, Permissions, Step, Text, Trigger, With, Workflow, render,
};

/// This repository's own CI: the built-in pins must match its SHAs.
const CI: &str = include_str!("../../../.github/workflows/ci.yml");
const OVERRIDES: &str =
    include_str!("../../../tests/fixtures/pin/overrides.actlith");

const BUILTIN_ACTIONS: [&str; 3] = [
    "actions/checkout",
    "cachix/install-nix-action",
    "cachix/cachix-action",
];

fn pin(table: &PinTable, action: &str) -> Option<Uses> {
    table.uses(action).ok()
}

fn uses_lines(yaml: &str) -> Vec<String> {
    yaml.lines()
        .map(str::trim_start)
        .filter(|l| l.starts_with("- uses: "))
        .map(str::to_owned)
        .collect()
}

fn step(uses: &Uses) -> Step {
    Step {
        comments: Vec::new(),
        name: None,
        id: None,
        condition: None,
        action: Action::Uses(uses.step(With::default())),
        env: BTreeMap::new(),
    }
}

fn job(steps: Vec<Step>) -> Job {
    Job {
        comments: Vec::new(),
        name: None,
        needs: Vec::new(),
        condition: None,
        runs_on: Text::lit("ubuntu-latest"),
        permissions: None,
        matrix: None,
        timeout_minutes: 5,
        env: BTreeMap::new(),
        steps,
    }
}

fn workflow(steps: Vec<Step>) -> Workflow {
    Workflow {
        comments: Vec::new(),
        name: "w".to_owned(),
        on: Trigger::default(),
        concurrency: None,
        permissions: Permissions(BTreeMap::new()),
        env: BTreeMap::new(),
        jobs: BTreeMap::from([("j".to_owned(), job(steps))]),
    }
}

fn overrides() -> Vec<ActionPin> {
    parse(OVERRIDES).map(|r| r.actions).unwrap_or_default()
}

fn row(sha: &str) -> ActionPin {
    ActionPin {
        line: 9,
        action: "actions/checkout".to_owned(),
        version: "v1".to_owned(),
        sha: sha.to_owned(),
    }
}

#[test]
fn builtin_renders_the_uses_lines_of_this_repos_ci() {
    let table = PinTable::builtin();
    let steps: Vec<Step> = BUILTIN_ACTIONS
        .iter()
        .filter_map(|a| pin(&table, a))
        .map(|u| step(&u))
        .collect();
    let got = uses_lines(&render(&workflow(steps)));
    assert_eq!(got, uses_lines(CI));
    assert_eq!(got.len(), 3);
}

#[test]
fn builtin_checkout_is_v7_0_1() {
    let got = pin(&PinTable::builtin(), "actions/checkout");
    let want = Uses {
        action: "actions/checkout".to_owned(),
        sha: "3d3c42e5aac5ba805825da76410c181273ba90b1".to_owned(),
        version: "v7.0.1".to_owned(),
    };
    assert_eq!(got, Some(want));
}

#[test]
fn override_replaces_builtin_and_adds_new_action() {
    let table = PinTable::builtin().with_overrides(&overrides());
    let table = table.unwrap_or_default();
    let checkout = pin(&table, "actions/checkout").map(|u| u.version);
    assert_eq!(checkout.as_deref(), Some("v4.2.2"));
    let new = pin(&table, "example/action");
    assert_eq!(new.map(|u| u.version).as_deref(), Some("v1.0.0"));
    let nix = pin(&table, "cachix/install-nix-action").map(|u| u.version);
    assert_eq!(nix.as_deref(), Some("v31.11.0"));
}

#[test]
fn override_with_tag_for_sha_is_refused() {
    let got = PinTable::builtin().with_overrides(&[row("v4")]);
    let want = PinError::NotSha {
        line: 9,
        action: "actions/checkout".to_owned(),
        sha: "v4".to_owned(),
    };
    assert_eq!(got, Err(want));
}

#[test]
fn override_with_upper_or_short_hex_is_refused() {
    let upper = "3D3C42E5AAC5BA805825DA76410C181273BA90B1";
    let short = "3d3c42e5aac5ba805825da76410c181273ba90b";
    for sha in [upper, short] {
        let got = PinTable::builtin().with_overrides(&[row(sha)]);
        assert!(matches!(got, Err(PinError::NotSha { .. })), "{sha}");
    }
}

#[test]
fn every_builtin_sha_is_40_hex() {
    let table = PinTable::builtin();
    let rows: Vec<ActionPin> = BUILTIN_ACTIONS
        .iter()
        .filter_map(|a| pin(&table, a))
        .map(|u| ActionPin {
            line: 0,
            action: u.action,
            version: u.version,
            sha: u.sha,
        })
        .collect();
    assert_eq!(rows.len(), 3);
    assert_eq!(PinTable::default().with_overrides(&rows), Ok(table));
}

#[test]
fn missing_pin_names_the_action() {
    let got = PinTable::builtin().uses("o/unknown");
    let want = PinError::Missing {
        action: "o/unknown".to_owned(),
    };
    assert_eq!(got, Err(want));
}
