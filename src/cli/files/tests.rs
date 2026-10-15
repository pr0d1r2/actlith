//! `gen`, `gen --check` and `check` over temp roots (`.:I`, V34).

use std::fs;
use std::path::Path;

use super::{check, generate};
use crate::cli::exit::Outcome;
use crate::cli::scratch::{put, scratch};

const REPO: &str = include_str!("../../../tests/fixtures/gen/repo.actlith");

/// Either side of a verb's result.
fn ran(result: Result<Outcome, Outcome>) -> Outcome {
    result.unwrap_or_else(|outcome| outcome)
}

/// A root holding the fixture `.actlith`.
fn repo(name: &str) -> std::path::PathBuf {
    let root = scratch(name);
    put(&root, ".actlith", REPO);
    root
}

#[test]
fn gen_writes_and_names_every_file_then_check_passes() {
    let root = repo("files-gen");
    let wrote = ran(generate(&root, false));
    assert_eq!(wrote.code, 0);
    assert!(wrote.out.contains("wrote .github/workflows/ci.yml\n"));
    assert!(wrote.out.contains("wrote greptile.json\n"));
    assert_eq!(ran(generate(&root, true)), Outcome::ok(""));
    assert_eq!(ran(check(&root)), Outcome::ok(""));
    assert_eq!(ran(generate(&root, false)), Outcome::ok(""));
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn gen_check_names_each_stale_and_missing_file_with_the_fix() {
    let root = repo("files-stale");
    let _ = ran(generate(&root, false));
    put(&root, ".github/workflows/ci.yml", "edited\n");
    assert!(fs::remove_file(root.join("greptile.json")).is_ok());
    let stale = ran(generate(&root, true));
    assert_eq!(stale.code, 1);
    assert!(
        stale
            .err
            .contains("stale: .github/workflows/ci.yml; fix: run")
    );
    assert!(stale.err.contains("missing: greptile.json; fix: run"));
    let ci = fs::read_to_string(root.join(".github/workflows/ci.yml"));
    assert_eq!(ci.ok().as_deref(), Some("edited\n"), "--check writes none");
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn check_names_an_undeclared_workflow_with_the_fix() {
    let root = repo("files-undeclared");
    let _ = ran(generate(&root, false));
    put(&root, ".github/workflows/extra.yml", "name: extra\n");
    let found = ran(check(&root));
    assert_eq!(found.code, 1);
    assert!(found.err.starts_with(".github/workflows/extra.yml: "));
    assert!(found.err.contains("fix: "));
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn a_missing_actlith_is_exit_two_for_both_verbs() {
    let root = scratch("files-missing");
    assert_eq!(ran(generate(&root, false)).code, 2);
    assert_eq!(ran(generate(&root, true)).code, 2);
    assert_eq!(ran(check(&root)).code, 2);
    let _ = fs::remove_dir_all(&root);
}

/// `root` with a directory where `rel` would be a file.
fn blocked(name: &str, rel: &str) -> std::path::PathBuf {
    let root = repo(name);
    assert!(fs::create_dir_all(Path::new(&root).join(rel)).is_ok());
    root
}

#[test]
fn a_read_or_write_failure_is_exit_two() {
    let root = blocked("files-blocked", ".github/workflows/ci.yml");
    assert_eq!(ran(generate(&root, false)).code, 2);
    assert_eq!(ran(generate(&root, true)).code, 2);
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn an_unlistable_workflows_dir_is_exit_two() {
    let root = repo("files-unlistable");
    put(&root, ".github/workflows", "not a directory\n");
    let failed = ran(check(&root));
    assert_eq!(failed.code, 2);
    assert!(failed.err.contains("cannot list"));
    let _ = fs::remove_dir_all(&root);
}
