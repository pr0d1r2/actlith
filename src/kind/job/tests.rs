//! Job hardening: timeout always set, permissions widened
//! only by the declared need, runner matrix (T17, V3).

use crate::yaml::GITHUB;
use std::collections::BTreeMap;

use crate::kind::{
    Args, KindError, Row, job, matrix_runner, runner_matrix, widen,
};
use crate::system::System;
use crate::yaml::{Level, Scope, Text};

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
    assert_eq!(matrix_runner().render(&GITHUB), "${{ matrix.os }}");
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
