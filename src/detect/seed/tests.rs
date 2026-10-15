//! `seed_init` in a temp dir: writes once, never over (detect V32, T27).

use std::fs;
use std::path::PathBuf;

use super::FILE;
use crate::detect::text::text;
use crate::detect::{SeedError, detect, seed_init};

/// A fresh, empty directory for one test.
fn scratch(name: &str) -> PathBuf {
    let id = std::process::id();
    let dir = std::env::temp_dir().join(format!("actlith-detect-{id}-{name}"));
    let _ = fs::remove_dir_all(&dir);
    assert!(fs::create_dir_all(&dir).is_ok());
    dir
}

#[test]
fn writes_the_detected_rows_when_absent() {
    let dir = scratch("absent");
    let rows = detect(&["flake.nix"], None);
    let path = seed_init(&dir, &rows).ok();
    assert_eq!(path, Some(dir.join(FILE)));
    let written = fs::read_to_string(dir.join(FILE)).unwrap_or_default();
    assert_eq!(written, text(&rows));
    let _ = fs::remove_dir_all(&dir);
}

/// Seed over a hand-written `.actlith`; returns the error and the file
/// as it is afterwards.
fn seed_over_existing(name: &str) -> (Option<SeedError>, PathBuf, String) {
    let dir = scratch(name);
    let path = dir.join(FILE);
    assert!(fs::write(&path, "format 1\n# mine\n").is_ok());
    let err = seed_init(&dir, &detect(&["flake.nix"], None)).err();
    let kept = fs::read_to_string(&path).unwrap_or_default();
    let _ = fs::remove_dir_all(&dir);
    (err, path, kept)
}

#[test]
fn an_existing_file_is_refused_and_kept() {
    let (err, path, kept) = seed_over_existing("exists");
    assert!(matches!(&err, Some(SeedError::Exists(p)) if *p == path));
    assert_eq!(err.as_ref().map(SeedError::code), Some(1));
    assert_eq!(kept, "format 1\n# mine\n");
}

#[test]
fn the_refusal_names_the_file() {
    let (err, path, _) = seed_over_existing("named");
    let shown = err.map(|e| e.to_string()).unwrap_or_default();
    assert!(shown.contains(&path.display().to_string()), "{shown}");
    assert!(shown.contains("already exists"), "{shown}");
}

#[test]
fn an_unwritable_root_is_a_write_error() {
    let dir = scratch("missing").join("no-such-dir");
    let err = seed_init(&dir, &[]).err();
    assert!(matches!(&err, Some(SeedError::Write(p, _)) if p.ends_with(FILE)));
    assert_eq!(err.as_ref().map(SeedError::code), Some(2));
    let shown = err.map(|e| e.to_string()).unwrap_or_default();
    assert!(shown.contains("cannot write"), "{shown}");
    let _ = fs::remove_dir_all(dir.parent().unwrap_or(&dir));
}
