use std::collections::BTreeMap;

use super::{Axis, AxisName, Job, Matrix, RunsOn};
use crate::core::yaml::document;
use crate::yaml::Env;
use crate::yaml::GITHUB;

fn job() -> Job {
    Job {
        comments: Vec::new(),
        name: None,
        needs: vec!["a".to_owned(), "b".to_owned()],
        condition: None,
        runs_on: RunsOn::One("ubuntu-latest".into()),
        permissions: None,
        matrix: None,
        timeout_minutes: 5,
        env: Env::new(),
        steps: Vec::new(),
    }
}

fn out(job: &Job) -> String {
    document(&GITHUB, &[], &[job.entry("j")])
}

#[test]
fn axis_name_refuses_matrix_keywords() {
    assert_eq!(AxisName::new("include"), None);
    assert_eq!(AxisName::new("exclude"), None);
    assert!(AxisName::new("os").is_some());
}

#[test]
fn minimal_job_keys_in_order() {
    let want = "---\nj:\n  needs: [a, b]\n  runs-on: ubuntu-latest\n  \
                timeout-minutes: 5\n  steps: []\n";
    assert_eq!(out(&job()), want);
}

#[test]
fn own_labels_are_a_flow_list() {
    let mut j = job();
    j.runs_on = RunsOn::Labels(
        ["self-hosted", "macOS", "X64"].map(str::to_owned).to_vec(),
    );
    let text = out(&j);
    assert!(
        text.contains("  runs-on: [self-hosted, macOS, X64]\n"),
        "{text}"
    );
}

fn axis(value: &str) -> Axis {
    Axis {
        comments: Vec::new(),
        values: vec![value.to_owned()],
    }
}

#[test]
fn matrix_axes_in_name_order_then_include() -> Result<(), &'static str> {
    let mut j = job();
    j.matrix = Some(Matrix {
        fail_fast: true,
        axes: BTreeMap::from([
            (AxisName::new("z").ok_or("axis")?, axis("1")),
            (AxisName::new("a").ok_or("axis")?, axis("x")),
        ]),
        include: vec![BTreeMap::from([("a".to_owned(), "w".to_owned())])],
    });
    let want = "  strategy:\n    fail-fast: true\n    matrix:\n      \
                a: [x]\n      z: ['1']\n      include:\n        - a: w\n";
    assert!(out(&j).contains(want));
    Ok(())
}
