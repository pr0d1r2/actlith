//! `gen` over a temp root: writes rendered paths only, only on change
//! (T24, gen C).

use std::fs;
use std::path::{Path, PathBuf};

use super::r#gen;
use crate::render::File;

/// A fresh, empty directory for one test.
fn scratch(name: &str) -> PathBuf {
    let id = std::process::id();
    let dir =
        std::env::temp_dir().join(format!("actlith-gen-write-{id}-{name}"));
    let _ = fs::remove_dir_all(&dir);
    assert!(fs::create_dir_all(&dir).is_ok());
    dir
}

fn file(path: &str, bytes: &str) -> File {
    File {
        path: path.to_owned(),
        bytes: bytes.to_owned(),
    }
}

fn two() -> Vec<File> {
    vec![
        file(".github/workflows/ci.yml", "name: ci\n"),
        file("scripts/actlith/open-pr.sh", "echo pr\n"),
    ]
}

fn read(root: &Path, rel: &str) -> String {
    fs::read_to_string(root.join(rel)).unwrap_or_default()
}

#[test]
fn writes_every_file_creating_its_parents() {
    let root = scratch("fresh");
    let written = r#gen(&root, &two()).ok();
    let want = [".github/workflows/ci.yml", "scripts/actlith/open-pr.sh"];
    assert_eq!(written, Some(want.map(str::to_owned).to_vec()));
    assert_eq!(read(&root, ".github/workflows/ci.yml"), "name: ci\n");
    assert_eq!(read(&root, "scripts/actlith/open-pr.sh"), "echo pr\n");
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn writes_nothing_when_every_file_is_current() {
    let root = scratch("current");
    assert!(r#gen(&root, &two()).is_ok());
    assert_eq!(r#gen(&root, &two()).ok(), Some(Vec::new()));
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn rewrites_only_the_file_whose_bytes_differ() {
    let root = scratch("changed");
    assert!(r#gen(&root, &two()).is_ok());
    let ci = root.join(".github/workflows/ci.yml");
    assert!(fs::write(&ci, "hand edit\n").is_ok());
    let written = r#gen(&root, &two()).ok();
    assert_eq!(written, Some(vec![".github/workflows/ci.yml".to_owned()]));
    assert_eq!(read(&root, ".github/workflows/ci.yml"), "name: ci\n");
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn refuses_an_escaping_path_before_writing_anything() {
    let root = scratch("escape");
    let mut files = two();
    files.push(file("../outside.yml", "x\n"));
    assert!(r#gen(&root.join("in"), &files).is_err());
    assert!(!root.join("outside.yml").exists());
    assert!(!root.join("in").exists());
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn reports_a_failed_write() {
    let root = scratch("blocked");
    assert!(fs::write(root.join(".github"), "a file, not a dir\n").is_ok());
    assert!(r#gen(&root, &two()).is_err());
    let _ = fs::remove_dir_all(&root);
}
