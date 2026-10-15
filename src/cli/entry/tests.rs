//! In-process runs of `actlith`: argv in, streams and exit code out
//! (V34). Every verb is dispatched once; `pin` uses the fake resolver.

use std::ffi::OsString;
use std::fs;
use std::path::Path;

use super::run;
use crate::cli::scratch::{Fake, put, scratch};

const REPO: &str = include_str!("../../../tests/fixtures/gen/repo.actlith");
const CI: &str =
    include_str!("../../../tests/fixtures/render/dot-github/workflows/ci.yml");

/// What one run printed and returned.
struct Ran {
    code: u8,
    out: String,
    err: String,
}

/// Run `actlith args...` in-process.
fn ran(args: &[&str]) -> Ran {
    let full = ["actlith"].iter().chain(args).map(OsString::from);
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let code = run(full, [&mut out, &mut err], &Fake);
    let text = |bytes: Vec<u8>| String::from_utf8_lossy(&bytes).into_owned();
    Ran {
        code,
        out: text(out),
        err: text(err),
    }
}

/// `path` as an argument.
fn arg(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

#[test]
fn help_is_stdout_and_exit_zero() {
    let help = ran(&["--help"]);
    assert_eq!((help.code, &*help.err), (0, ""));
    assert!(help.out.contains("Examples:"));
    assert_eq!(ran(&["--version"]).code, 0);
}

#[test]
fn a_usage_error_is_stderr_and_exit_two_naming_help() {
    let bad = ran(&["nope"]);
    assert_eq!((bad.code, &*bad.out), (2, ""));
    assert!(bad.err.contains("--help"));
}

#[test]
fn the_file_verbs_dispatch() {
    let root = scratch("entry-files");
    put(&root, ".actlith", REPO);
    let dir = arg(&root);
    assert_eq!(ran(&["gen", "--root", &dir]).code, 0);
    assert_eq!(ran(&["gen", "--check", "--root", &dir]).code, 0);
    assert_eq!(ran(&["check", "--root", &dir]).code, 0);
    let report = ran(&["report", "--root", &dir, "--format", "md"]);
    assert_eq!(report.code, 0);
    let pin = ran(&["pin", "--dry-run", "--root", &dir]);
    assert!(pin.out.starts_with("action|version|sha\n"));
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn the_row_verbs_dispatch() {
    let root = scratch("entry-rows");
    put(&root, "ci.yml", CI);
    let file = arg(&root.join("ci.yml"));
    assert_eq!(ran(&["import", &file]).out, "ci|gate|-|pr0d1r2|-\n");
    let dir = arg(&root);
    assert_eq!(ran(&["detect", "--root", &dir]).code, 2);
    assert_eq!(ran(&["seed", "--root", &dir]).code, 2);
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn completions_and_man_dispatch() {
    assert!(ran(&["completions", "fish"]).out.contains("actlith"));
    assert!(ran(&["man"]).out.contains(".TH"));
}
