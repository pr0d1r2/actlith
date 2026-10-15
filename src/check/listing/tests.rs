//! `listing` in temp dirs: workflow files and bot files, sorted (check C).

use std::fs;
use std::path::{Path, PathBuf};

use crate::check::listing;

/// A fresh, empty directory for one test.
fn scratch(name: &str) -> PathBuf {
    let id = std::process::id();
    let dir = std::env::temp_dir().join(format!("actlith-check-{id}-{name}"));
    let _ = fs::remove_dir_all(&dir);
    assert!(fs::create_dir_all(&dir).is_ok());
    dir
}

/// Write `path` (and its parents) under `root` with some text.
fn touch(root: &Path, path: &str) {
    let file = root.join(path);
    if let Some(parent) = file.parent() {
        assert!(fs::create_dir_all(parent).is_ok());
    }
    assert!(fs::write(file, "x\n").is_ok());
}

/// The files `lists_workflow_and_bot_files_sorted` writes, unsorted.
const WRITTEN: [&str; 6] = [
    "socket.yml",
    ".github/workflows/z.yml",
    ".github/workflows/a.yaml",
    "greptile.json",
    ".coderabbit.yaml",
    "README.md",
];

/// What it lists: all but `README.md`, sorted.
const LISTED: [&str; 5] = [
    ".coderabbit.yaml",
    ".github/workflows/a.yaml",
    ".github/workflows/z.yml",
    "greptile.json",
    "socket.yml",
];

#[test]
fn lists_workflow_and_bot_files_sorted() {
    let dir = scratch("both");
    for path in WRITTEN {
        touch(&dir, path);
    }
    let got = listing(&dir).unwrap_or_default();
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(got, LISTED);
}

#[test]
fn lists_every_file_in_workflows_but_not_subdirectories() {
    let dir = scratch("flat");
    touch(&dir, ".github/workflows/notes.txt");
    touch(&dir, ".github/workflows/deep/inner.yml");
    let got = listing(&dir).unwrap_or_default();
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(got, [".github/workflows/notes.txt"]);
}

#[test]
fn a_bot_path_that_is_a_directory_is_not_listed() {
    let dir = scratch("botdir");
    assert!(fs::create_dir_all(dir.join("socket.yml")).is_ok());
    let got = listing(&dir).unwrap_or_default();
    let _ = fs::remove_dir_all(&dir);
    assert!(got.is_empty(), "{got:?}");
}

#[test]
fn no_workflows_directory_lists_nothing() {
    let dir = scratch("empty");
    let got = listing(&dir).ok();
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(got, Some(Vec::new()));
}

#[test]
fn an_unreadable_workflows_path_is_an_error() {
    let dir = scratch("notdir");
    touch(&dir, ".github/workflows");
    let got = listing(&dir);
    let _ = fs::remove_dir_all(&dir);
    assert!(got.is_err(), "{got:?}");
}
