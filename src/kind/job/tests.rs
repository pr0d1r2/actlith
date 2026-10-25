//! Job hardening: timeout always set, permissions widened
//! only by the declared need, runner matrix (T17, V3); systems split by
//! host, own-runner jobs on their labels behind the fork-PR guard (V54).

use crate::yaml::GITHUB;
use std::collections::BTreeMap;

use crate::kind::{
    Args, Row, TRUSTED, hosts, job, matrix_runner, own, place, runner_matrix,
    widen,
};
use crate::registry::parse;
use crate::system::{Host, Runners, System};
use crate::yaml::{Expr, Level, RunsOn, Scope, Text};

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
    assert_eq!(built.runs_on, RunsOn::One(Text::lit("ubuntu-latest")));
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

/// `## runners` rows, checked by pipelith.
fn runners(rows: &str) -> Result<Runners, String> {
    let text = format!("format 1\n\n## runners\nsystem|labels\n{rows}");
    parse(&text)
        .map(|registry| registry.runners)
        .map_err(|e| e.to_string())
}

const INTEL: &str = "x86_64-darwin|self-hosted, macOS, X64\n";

fn own_labels() -> Vec<String> {
    ["self-hosted", "macOS", "X64"].map(str::to_owned).to_vec()
}

#[test]
fn the_matrix_runs_on_each_system_runner() {
    let both = row(vec![System::Aarch64Darwin, System::X86_64Linux]);
    let matrix = runner_matrix(&hosts(&both, &Runners::default()).hosted);
    assert!(!matrix.fail_fast);
    let axis = matrix.axes.values().next();
    let values = axis.map(|a| a.values.clone()).unwrap_or_default();
    assert_eq!(values, ["macos-latest", "ubuntu-latest"]);
    let comment = axis
        .and_then(|a| a.comments.first())
        .map(crate::yaml::Comment::lines);
    assert_eq!(comment, Some(vec!["# aarch64-darwin, x86_64-linux".into()]));
    assert_eq!(matrix_runner().render(&GITHUB), "${{ matrix.os }}");
}

#[test]
fn every_system_is_hosted_without_runners() {
    let all = row(crate::core::system::ALL.to_vec());
    let none = Runners::default();
    let split = hosts(&all, &none);
    assert!(split.own.is_empty());
    let labels: Vec<&str> = split.hosted.iter().map(|&(_, l)| l).collect();
    assert_eq!(
        labels,
        [
            "ubuntu-latest",
            "ubuntu-24.04-arm",
            "macos-latest",
            "macos-15-intel"
        ]
    );
}

#[test]
fn a_runners_row_takes_its_system_out_of_the_hosted_ones() -> Result<(), String>
{
    let own = runners(INTEL)?;
    let both = row(vec![System::X86_64Linux, System::X86_64Darwin]);
    let split = hosts(&both, &own);
    assert_eq!(split.hosted, [(System::X86_64Linux, "ubuntu-latest")]);
    let labels = own_labels();
    assert_eq!(split.own, [(System::X86_64Darwin, labels.as_slice())]);
    Ok(())
}

#[test]
fn an_own_runner_job_runs_on_every_label() {
    let placed = own(job(Text::default(), 30, Vec::new()), &own_labels());
    assert_eq!(placed.runs_on, RunsOn::Labels(own_labels()));
}

#[test]
fn an_own_runner_job_skips_fork_pull_requests() {
    let placed = own(job(Text::default(), 30, Vec::new()), &own_labels());
    let guard = placed.condition.map(|c| c.inner().to_owned());
    assert_eq!(
        guard.as_deref(),
        Some(
            "github.event_name != 'pull_request' || \
             github.event.pull_request.head.repo.full_name == \
             github.repository"
        )
    );
    assert_eq!(guard.as_deref(), Some(TRUSTED));
}

#[test]
fn the_guard_joins_a_condition_the_job_has() {
    let mut main = job(Text::default(), 30, Vec::new());
    main.condition = Expr::new(&GITHUB, "github.ref == 'refs/heads/main'");
    let placed = own(main, &own_labels());
    let guard = placed.condition.map(|c| c.inner().to_owned());
    let want = format!("({TRUSTED}) && (github.ref == 'refs/heads/main')");
    assert_eq!(guard, Some(want));
}

#[test]
fn a_hosted_job_has_no_guard() {
    let labels = own_labels();
    let hosted = place(job(Text::default(), 30, Vec::new()), Host::Hosted("x"));
    assert_eq!(hosted.runs_on, RunsOn::One(Text::lit("x")));
    assert_eq!(hosted.condition, None);
    let mine = place(job(Text::default(), 30, Vec::new()), Host::Own(&labels));
    assert_eq!(mine.runs_on, RunsOn::Labels(own_labels()));
    assert!(mine.condition.is_some());
}
