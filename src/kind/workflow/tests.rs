//! Workflow hardening (T17, V3), and the proof the common builders are
//! enough: a kind made of nothing but them renders this repository's own
//! `ci.yml` byte for byte.

use crate::yaml::GITHUB;
use std::collections::BTreeMap;

use crate::kind::{
    CachixPush, Checkout, GATE, Kind, KindError, KindSpec, Row, Secret, cachix,
    checkout, concurrency, install_nix, job, matrix_runner, on_push_and_pr,
    on_schedule, read_only, run, runner_matrix, workflow,
};
use crate::pin::PinTable;
use crate::registry::parse;
use crate::yaml::{Comment, Level, Scope, Step, Workflow, write};

/// The hand-written ci.yml this repository had before actlith generated
/// it (nix T10): the golden shape the model and builders must reproduce.
const CI: &str = include_str!("../../../tests/fixtures/yaml/ci-hand.yml");
const ROW: &str = include_str!("../../../tests/fixtures/kind/ci.actlith");

const CI_WHY: &str = "Hand-written until `actlith gen` can render the gate \
    kind, then generated\n(T10, V21). Orchestration only: the gate is \
    hk.pkl, run through the same\ndev shell a laptop uses, so CI and a local \
    `hk check` cannot disagree\nabout a tool version. One job per CI system \
    (SPEC.md C).";

const DEPTH_WHY: &str = "Full history: tdd-order reads the push range, and \
    a later semver\nstep compares with the last release tag.";

/// The gate shape, built from the common builders only.
struct Gate;

impl Kind for Gate {
    fn spec(&self) -> &'static KindSpec {
        &GATE
    }

    fn secrets(&self, row: &Row) -> Vec<Secret> {
        let push = row.cache.as_ref().and(CachixPush::SkipForkPr.secret());
        push.into_iter().collect()
    }

    fn build(&self, row: &Row, pins: &PinTable) -> Result<Workflow, KindError> {
        let mut gate =
            job(matrix_runner(), row.args.timeout()?, steps(row, pins)?);
        gate.name = Some("gate".to_owned());
        gate.matrix = Some(runner_matrix(row)?);
        let jobs = BTreeMap::from([("gate".to_owned(), gate)]);
        let mut built = workflow(&row.id, on_push_and_pr(), jobs);
        built.comments = vec![Comment::new(CI_WHY)];
        Ok(built)
    }
}

fn steps(row: &Row, pins: &PinTable) -> Result<Vec<Step>, KindError> {
    let how = Checkout {
        pushes: false,
        fetch_depth: Some(0),
        why_depth: Some(Comment::new(DEPTH_WHY)),
    };
    let mut steps = vec![
        checkout(pins, &how)?,
        install_nix(pins, row.cache.as_ref())?,
    ];
    if let Some(cache) = &row.cache {
        steps.push(cachix(pins, cache, CachixPush::SkipForkPr)?);
    }
    let command = row.args.get("command").unwrap_or_default();
    steps.push(run(row.line, command, &[])?);
    Ok(steps)
}

fn ci_row() -> Result<Row, String> {
    let registry = parse(ROW).map_err(|e| e.to_string())?;
    let row = registry.workflows.first().ok_or("no row")?;
    crate::kind::check(&Gate, row).map_err(|e| e.to_string())
}

#[test]
fn the_builders_reproduce_ci_yml_byte_for_byte() -> Result<(), String> {
    let row = ci_row()?;
    let built = Gate.build(&row, &PinTable::builtin());
    assert_eq!(built.map(|w| write(&w)), Ok(CI.to_owned()));
    Ok(())
}

#[test]
fn the_ci_row_declares_the_cachix_secret() -> Result<(), String> {
    let row = ci_row()?;
    let names: Vec<String> = Gate
        .secrets(&row)
        .iter()
        .map(|s| s.name().to_owned())
        .collect();
    assert_eq!(names, ["CACHIX_AUTH_TOKEN"]);
    assert_eq!(Gate.name(), "gate");
    Ok(())
}

#[test]
fn every_workflow_reads_contents_only_at_the_top() {
    for on in [on_push_and_pr(), on_schedule("50 6 * * *")] {
        let built = workflow("w", on, BTreeMap::new());
        assert_eq!(built.permissions, read_only());
        let scopes: Vec<_> = built.permissions.0.into_iter().collect();
        assert_eq!(scopes, [(Scope::Contents, Level::Read)]);
        assert_eq!(built.name, "w");
    }
}

#[test]
fn push_and_pr_runs_cancel_scheduled_runs_do_not() {
    let push = concurrency("ci", &on_push_and_pr());
    assert_eq!(push.group.render(&GITHUB), "ci-${{ github.ref }}");
    assert!(push.cancel_in_progress);
    let scheduled = concurrency("update-pins", &on_schedule("50 6 * * *"));
    assert_eq!(
        scheduled.group.render(&GITHUB),
        "update-pins-${{ github.ref }}"
    );
    assert!(!scheduled.cancel_in_progress);
}

#[test]
fn the_workflow_carries_its_concurrency() {
    let on = on_schedule("20 6 * * *");
    let built = workflow("up", on.clone(), BTreeMap::new());
    assert_eq!(built.concurrency, Some(concurrency("up", &on)));
    assert_eq!(built.on.schedule, ["20 6 * * *"]);
    assert!(built.on.workflow_dispatch && !built.on.pull_request);
    assert_eq!(built.on.push, None);
}

#[test]
fn no_trigger_writes_pull_request_target() {
    for on in [on_push_and_pr(), on_schedule("0 0 * * *")] {
        let text = write(&workflow("w", on, BTreeMap::new()));
        assert!(!text.contains("pull_request_target"), "{text}");
    }
}
