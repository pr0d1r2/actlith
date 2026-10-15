//! `gen --check` over a temp root: names each stale or missing file and
//! writes nothing (V6, T24).

use std::fs;
use std::path::{Path, PathBuf};

use crate::r#gen::{Stale, check, r#gen};
use crate::registry::parse;
use crate::render::{File, render};

const REPO: &str = include_str!("../../../tests/fixtures/gen/repo.actlith");

/// A fresh, empty directory for one test.
fn scratch(name: &str) -> PathBuf {
    let id = std::process::id();
    let dir =
        std::env::temp_dir().join(format!("actlith-gen-stale-{id}-{name}"));
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

/// Every entry under `dir`, sorted; to prove a check left it untouched.
fn tree(dir: &Path) -> Vec<(PathBuf, Vec<u8>)> {
    let mut out = Vec::new();
    let entries = fs::read_dir(dir).map(Iterator::collect::<Vec<_>>);
    for entry in entries.unwrap_or_default().into_iter().flatten() {
        let path = entry.path();
        if path.is_dir() {
            out.push((path.clone(), Vec::new()));
            out.extend(tree(&path));
        } else {
            out.push((path.clone(), fs::read(&path).unwrap_or_default()));
        }
    }
    out.sort();
    out
}

#[test]
fn names_every_missing_file_and_creates_nothing() {
    let root = scratch("missing");
    let want = vec![
        Stale::Missing(".github/workflows/ci.yml".to_owned()),
        Stale::Missing("scripts/actlith/open-pr.sh".to_owned()),
    ];
    assert_eq!(check(&root, &two()).ok(), Some(want));
    assert_eq!(tree(&root), Vec::new());
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn names_a_changed_file_and_leaves_it_as_it_was() {
    let root = scratch("changed");
    assert!(r#gen(&root, &two()).is_ok());
    let ci = root.join(".github/workflows/ci.yml");
    assert!(fs::write(&ci, "hand edit\n").is_ok());
    let before = tree(&root);
    let want = vec![Stale::Changed(".github/workflows/ci.yml".to_owned())];
    assert_eq!(check(&root, &two()).ok(), Some(want));
    assert_eq!(tree(&root), before);
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn finds_nothing_after_gen_of_a_rendered_fixture() {
    let root = scratch("fixture");
    let files = parse(REPO).ok().and_then(|r| render(&r).ok());
    let files = files.unwrap_or_default();
    assert!(files.len() > 2, "{files:?}");
    let written = r#gen(&root, &files).ok().unwrap_or_default();
    assert_eq!(written.len(), files.len());
    assert_eq!(check(&root, &files).ok(), Some(Vec::new()));
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn refuses_an_escaping_path() {
    let root = scratch("escape");
    assert!(check(&root, &[file("/etc/passwd", "x\n")]).is_err());
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn reports_a_path_it_cannot_read() {
    let root = scratch("unreadable");
    assert!(fs::create_dir_all(root.join(".github/workflows/ci.yml")).is_ok());
    assert!(check(&root, &two()).is_err());
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn names_each_kind_and_path_when_shown() {
    let missing = Stale::Missing("a.yml".to_owned()).to_string();
    let changed = Stale::Changed("b.yml".to_owned()).to_string();
    assert_eq!(missing, "missing: a.yml");
    assert_eq!(changed, "stale: b.yml");
}
