//! `detect`, `seed --init` and `import` over temp roots (`.:I`, V34).

use std::fs;
use std::os::unix::fs::PermissionsExt as _;
use std::path::{Path, PathBuf};

use super::{detect, import, seed};
use crate::cli::exit::Outcome;
use crate::cli::scratch::{git, put, scratch};

const LOCK: &str =
    include_str!("../../../tests/fixtures/detect/with-lock.lock");
const CI: &str =
    include_str!("../../../tests/fixtures/render/dot-github/workflows/ci.yml");
const FOREIGN: &str =
    include_str!("../../../tests/fixtures/import/foreign.yml");

/// Either side of a verb's result.
fn ran(result: Result<Outcome, Outcome>) -> Outcome {
    result.unwrap_or_else(|outcome| outcome)
}

/// A git repository tracking `flake.nix` and a lock with `nixpkgs-lock`.
fn flake(name: &str) -> PathBuf {
    let root = scratch(name);
    put(&root, "flake.nix", "{ }\n");
    put(&root, "flake.lock", LOCK);
    assert!(git(&root, &["init", "-q"]));
    assert!(git(&root, &["add", "flake.nix", "flake.lock"]));
    root
}

#[test]
fn detect_prints_one_row_per_line() {
    let root = flake("rows-detect");
    let rows = ran(detect(&root));
    let want = "ci|gate|-|-|-\n# off by default: uncomment to get daily \
                nixpkgs-lock bump PRs\n# update-pins|update-pins|-|-|-\n";
    assert_eq!(rows, Outcome::ok(want));
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn seed_leaves_update_pins_off() {
    let root = flake("rows-seed-off");
    assert_eq!(ran(seed(&root, true)).code, 0);
    let text = fs::read_to_string(root.join(".actlith")).unwrap_or_default();
    let on = crate::registry::parse(&text).map(|r| r.workflows.len());
    assert_eq!(on, Ok(1), "only `ci` is live; update-pins is commented");
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn detect_outside_a_repository_is_exit_two() {
    let root = scratch("rows-norepo");
    assert_eq!(ran(detect(&root)).code, 2);
    assert_eq!(ran(seed(&root, true)).code, 2);
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn seed_writes_once_then_refuses_naming_the_file() {
    let root = flake("rows-seed");
    let first = ran(seed(&root, true));
    assert_eq!(first.code, 0);
    assert!(first.out.starts_with("wrote "));
    let text = fs::read_to_string(root.join(".actlith")).unwrap_or_default();
    assert!(text.contains("ci|gate|-|-|-\n"));
    let again = ran(seed(&root, true));
    assert_eq!(again.code, 1);
    assert!(again.err.contains(".actlith: already exists"));
    assert!(again.err.contains("fix: "));
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn seed_without_init_is_a_usage_error() {
    let failed = ran(seed(Path::new("."), false));
    assert_eq!(failed.code, 2);
    assert!(failed.err.contains("fix: run `actlith seed --init`"));
}

#[test]
fn an_unwritable_seed_is_exit_two() {
    let root = flake("rows-unwritable");
    let mode = |m| fs::set_permissions(&root, fs::Permissions::from_mode(m));
    assert!(mode(0o555).is_ok());
    let failed = ran(seed(&root, true));
    assert!(mode(0o755).is_ok());
    assert_eq!(failed.code, 2);
    assert!(failed.err.contains("fix: make the root writable"));
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn import_prints_the_row_of_a_matching_file() {
    let root = scratch("rows-import");
    put(&root, "ci.yml", CI);
    let row = import(&[root.join("ci.yml")]);
    assert_eq!(row, Outcome::ok("ci|gate|-|pr0d1r2|-\n"));
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn import_refuses_a_file_named_other_than_its_id() {
    let root = scratch("rows-renamed");
    put(&root, "gate.yml", CI);
    let failed = import(&[root.join("gate.yml")]);
    assert_eq!((failed.code, &*failed.out), (1, ""));
    assert!(failed.err.contains("renders as row `ci`"));
    assert!(failed.err.contains("fix: rename the file to `ci.yml`"));
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn import_reports_every_file_and_exits_with_the_worst() {
    let root = scratch("rows-mixed");
    put(&root, "ci.yml", CI);
    put(&root, "foreign.yml", FOREIGN);
    let files = ["ci.yml", "foreign.yml", "absent.yml"].map(|f| root.join(f));
    let all = import(&files);
    assert_eq!((all.code, &*all.out), (2, "ci|gate|-|pr0d1r2|-\n"));
    assert!(all.err.contains("foreign.yml: line "));
    assert!(all.err.contains("absent.yml: cannot read"));
    let _ = fs::remove_dir_all(&root);
}
