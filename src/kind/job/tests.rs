//! Job hardening: timeout always set and bounded, permissions widened
//! only by the declared need, runner matrix (T17, V3).

use std::collections::BTreeMap;

use crate::kind::{
    ArgSpec, Args, DEFAULT_TIMEOUT, KindError, MAX_TIMEOUT, Row, TIMEOUT, job,
    matrix_runner, parse_args, runner_matrix, timeout, widen,
};
use crate::registry::{Arg, WorkflowRow};
use crate::system::System;
use crate::yaml::{Level, Scope, Text};

fn args(schema: &[ArgSpec], pairs: &[(&str, &str)]) -> Args {
    let row = WorkflowRow {
        line: 4,
        id: "w".to_owned(),
        kind: "k".to_owned(),
        systems: Vec::new(),
        cache: None,
        args: pairs
            .iter()
            .map(|&(k, v)| Arg {
                key: k.to_owned(),
                value: v.to_owned(),
            })
            .collect(),
    };
    parse_args(schema, &row).unwrap_or_default()
}

fn row(systems: Vec<System>) -> Row {
    Row {
        id: "w".to_owned(),
        line: 4,
        systems,
        cache: None,
        args: Args::default(),
    }
}

#[test]
fn a_job_always_has_its_timeout() {
    let built = job(Text::lit("ubuntu-latest"), 12, Vec::new());
    assert_eq!(built.timeout_minutes, 12);
    assert_eq!(built.permissions, None);
    assert_eq!(built.runs_on, Text::lit("ubuntu-latest"));
}

#[test]
fn timeout_defaults_to_thirty() -> Result<(), KindError> {
    assert_eq!(DEFAULT_TIMEOUT, 30);
    assert_eq!(timeout(&args(&[TIMEOUT], &[]))?, 30);
    assert_eq!(timeout(&args(&[], &[]))?, 30);
    assert_eq!(timeout(&args(&[TIMEOUT], &[("timeout", "360")]))?, 360);
    Ok(())
}

#[test]
fn timeout_out_of_range_is_refused() {
    assert_eq!(MAX_TIMEOUT, 360);
    for minutes in ["0", "361"] {
        let given = args(&[TIMEOUT], &[("timeout", minutes)]);
        let error = timeout(&given).err().map(|e| e.to_string());
        let want = format!(
            ".actlith:4: timeout `{minutes}` is not 1 to 360 minutes\n  fix: \
             write `timeout=<1..360>`"
        );
        assert_eq!(error, Some(want));
    }
}

#[test]
fn a_non_number_timeout_is_refused() {
    let given = args(&[TIMEOUT], &[("timeout", "x")]);
    assert!(timeout(&given).is_err());
}

#[test]
fn widening_keeps_contents_read_and_adds_only_the_need() {
    let bump = widen(&[(Scope::PullRequests, Level::Write)]);
    assert_eq!(
        bump.0,
        BTreeMap::from([
            (Scope::Contents, Level::Read),
            (Scope::PullRequests, Level::Write),
        ])
    );
    let push = widen(&[(Scope::Contents, Level::Write)]);
    assert_eq!(push.0, BTreeMap::from([(Scope::Contents, Level::Write)]));
}

#[test]
fn the_matrix_runs_on_each_system_runner() -> Result<(), KindError> {
    let matrix =
        runner_matrix(&row(vec![System::Aarch64Darwin, System::X86_64Linux]))?;
    assert!(!matrix.fail_fast);
    let axis = matrix.axes.values().next();
    let values = axis.map(|a| a.values.clone()).unwrap_or_default();
    assert_eq!(values, ["macos-latest", "ubuntu-latest"]);
    let comment = axis
        .and_then(|a| a.comments.first())
        .map(crate::yaml::Comment::lines);
    assert_eq!(comment, Some(vec!["# aarch64-darwin, x86_64-linux".into()]));
    assert_eq!(matrix_runner().render(), "${{ matrix.os }}");
    Ok(())
}

#[test]
fn a_system_without_a_runner_is_refused_in_the_matrix() {
    let error = runner_matrix(&row(vec![System::X86_64Darwin]));
    let error = error.err().map(|e| e.to_string()).unwrap_or_default();
    assert!(
        error.starts_with(".actlith:4: system `x86_64-darwin`"),
        "{error}"
    );
}
