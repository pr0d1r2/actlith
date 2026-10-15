//! The test helpers themselves.

use std::fs;

use super::{git, put, scratch};

#[test]
fn scratch_is_fresh_and_put_creates_parents() {
    let root = scratch("helpers");
    put(&root, "a/b.txt", "x");
    assert_eq!(
        fs::read_to_string(root.join("a/b.txt")).ok(),
        Some("x".into())
    );
    let again = scratch("helpers");
    assert!(!again.join("a").exists());
    let _ = fs::remove_dir_all(&again);
}

#[test]
fn git_reports_success_and_failure() {
    let root = scratch("helpers-git");
    assert!(git(&root, &["init", "-q"]));
    assert!(!git(&root, &["no-such-verb"]));
    let _ = fs::remove_dir_all(&root);
}
