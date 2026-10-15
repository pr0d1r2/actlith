//! `git ls-files` for `detect` and `seed --init`.

use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use super::{GIT_REPO_VARS, command, tracked};
use crate::cli::scratch::{git, put, scratch};

/// The variables `command` removes from the child's environment.
fn removed(root: &Path) -> Vec<String> {
    let cmd = command(root);
    let gone = cmd.get_envs().filter(|(_, value)| value.is_none());
    gone.map(|(key, _)| key.to_string_lossy().into_owned())
        .collect()
}

#[test]
fn another_root_drops_every_repository_variable() {
    let mut gone = removed(Path::new("/elsewhere"));
    gone.sort_unstable();
    let mut want = GIT_REPO_VARS.map(str::to_owned).to_vec();
    want.sort_unstable();
    assert_eq!(gone, want);
}

#[test]
fn the_current_directory_keeps_them() {
    assert!(removed(Path::new(".")).is_empty());
    let cmd = command(Path::new("."));
    let args: Vec<&OsStr> = cmd.get_args().collect();
    assert_eq!(args, ["ls-files", "-z"]);
}

#[test]
fn lists_the_tracked_files_of_a_repository() {
    let root = scratch("git-tracked");
    put(&root, "flake.nix", "{ }\n");
    put(&root, "dir/a b.txt", "x\n");
    put(&root, "untracked.txt", "x\n");
    assert!(git(&root, &["init", "-q"]));
    assert!(git(&root, &["add", "flake.nix", "dir"]));
    let files = tracked(&root).unwrap_or_default();
    assert_eq!(files, ["dir/a b.txt", "flake.nix"]);
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn outside_a_work_tree_is_exit_two_naming_root() {
    let root = scratch("git-none");
    let failed = tracked(&root).err().unwrap_or_default();
    assert_eq!(failed.code, 2);
    assert!(failed.err.contains("`git ls-files` failed"));
    assert!(failed.err.contains("fix: run in a git work tree"));
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn a_missing_root_is_exit_two() {
    let failed = tracked(Path::new("/no/such/actlith/root"));
    let failed = failed.err().unwrap_or_default();
    assert_eq!(failed.code, 2);
    assert!(failed.err.contains("fix: install git, or pass"));
}
