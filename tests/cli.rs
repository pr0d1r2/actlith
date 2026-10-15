//! The built `actlith` binary over temp repositories made from the
//! fixtures (`.:I` cmd, `src/cli:V34`). Hermetic: no network; `pin`
//! reaches a fake `gh` on PATH; every git run drops the repository
//! variables and turns hooks off.

use std::fs;
use std::os::unix::fs::PermissionsExt as _;
use std::path::{Path, PathBuf};
use std::process::Command;

const BIN: &str = env!("CARGO_BIN_EXE_actlith");
const REPO: &str = include_str!("fixtures/gen/repo.actlith");
const ALL: &str = include_str!("fixtures/render/all.actlith");
const JSON: &str = include_str!("fixtures/report/all.json");
const FAKE_GH: &str = include_str!("fixtures/cli/fake-gh.sh");

/// Repository variables no child of a test may inherit.
const GIT_REPO_VARS: [&str; 6] = [
    "GIT_DIR",
    "GIT_WORK_TREE",
    "GIT_INDEX_FILE",
    "GIT_OBJECT_DIRECTORY",
    "GIT_ALTERNATE_OBJECT_DIRECTORIES",
    "GIT_COMMON_DIR",
];

/// What one run printed and returned.
struct Ran {
    code: Option<i32>,
    out: String,
    err: String,
}

/// A fresh, empty directory outside this work tree.
fn scratch(name: &str) -> PathBuf {
    let id = std::process::id();
    let dir = std::env::temp_dir().join(format!("actlith-it-{id}-{name}"));
    let _ = fs::remove_dir_all(&dir);
    assert!(fs::create_dir_all(&dir).is_ok());
    dir
}

/// Write `text` at `rel` under `root`, creating parents.
fn put(root: &Path, rel: &str, text: &str) {
    let path = root.join(rel);
    assert!(path.parent().is_some_and(|p| fs::create_dir_all(p).is_ok()));
    assert!(fs::write(path, text).is_ok());
}

/// `command` with the repository variables dropped.
fn hermetic(mut command: Command) -> Command {
    for var in GIT_REPO_VARS {
        command.env_remove(var);
    }
    command
}

/// Run the binary with `args` in `dir`.
fn actlith(dir: &Path, args: &[&str]) -> Ran {
    let mut command = hermetic(Command::new(BIN));
    run(command.args(args).current_dir(dir))
}

/// Run `command`, capturing both streams.
fn run(command: &mut Command) -> Ran {
    let out = command.output();
    let text = |b: &[u8]| String::from_utf8_lossy(b).into_owned();
    out.map_or_else(
        |e| Ran {
            code: None,
            out: String::new(),
            err: e.to_string(),
        },
        |o| Ran {
            code: o.status.code(),
            out: text(&o.stdout),
            err: text(&o.stderr),
        },
    )
}

/// Real `git args...` in `root`, hooks off.
fn git(root: &Path, args: &[&str]) -> bool {
    let mut command = hermetic(Command::new("git"));
    command.args(["-c", "core.hooksPath=/dev/null"]).args(args);
    run(command.current_dir(root)).code == Some(0)
}

/// A root holding the gen fixture `.actlith`, generated.
fn generated(name: &str) -> PathBuf {
    let root = scratch(name);
    put(&root, ".actlith", REPO);
    assert_eq!(actlith(&root, &["gen"]).code, Some(0));
    root
}

#[test]
fn gen_then_gen_check_exits_zero() {
    let root = generated("gen");
    let check = actlith(&root, &["gen", "--check"]);
    assert_eq!((check.code, &*check.out, &*check.err), (Some(0), "", ""));
    assert_eq!(actlith(&root, &["check"]).code, Some(0));
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn an_edited_file_makes_gen_check_exit_one_naming_it() {
    let root = generated("edited");
    put(&root, ".github/workflows/ci.yml", "edited\n");
    let check = actlith(&root, &["gen", "--check"]);
    assert_eq!(check.code, Some(1));
    assert!(check.err.contains("stale: .github/workflows/ci.yml; fix: "));
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn an_undeclared_workflow_makes_check_exit_one() {
    let root = generated("undeclared");
    put(&root, ".github/workflows/extra.yml", "name: extra\n");
    let dir = root.to_string_lossy().into_owned();
    let check = actlith(Path::new("/"), &["check", "--root", &dir]);
    assert_eq!(check.code, Some(1));
    assert!(check.err.contains(".github/workflows/extra.yml: "));
    assert!(check.err.contains("fix: "));
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn a_missing_actlith_exits_two_naming_the_fix() {
    let root = scratch("missing");
    for verb in ["gen", "check", "report", "pin"] {
        let ran = actlith(&root, &[verb]);
        assert_eq!(ran.code, Some(2), "{verb}");
        assert!(ran.err.contains("fix: "), "{verb}: {}", ran.err);
    }
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn report_json_is_the_golden_document() {
    let root = scratch("report");
    put(&root, ".actlith", ALL);
    let json = actlith(&root, &["report", "--format", "json"]);
    assert_eq!((json.code, &*json.out), (Some(0), JSON));
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn help_lists_every_verb_with_an_example() {
    let help = actlith(Path::new("/"), &["--help"]);
    assert_eq!(help.code, Some(0));
    let verbs = ["gen", "check", "detect", "seed", "import", "report"];
    let more = ["pin", "completions", "man"];
    for verb in verbs.iter().chain(&more) {
        assert!(help.out.contains(&format!("  {verb} ")), "{verb}");
        assert!(help.out.contains(&format!("actlith {verb}")), "{verb}");
    }
}

#[test]
fn a_usage_error_exits_two() {
    let bad = actlith(Path::new("/"), &["gen", "--nope"]);
    assert_eq!(bad.code, Some(2));
    assert!(bad.err.contains("--help"));
}

/// A git repository tracking a `flake.nix`.
fn flake(name: &str) -> PathBuf {
    let root = scratch(name);
    put(&root, "flake.nix", "{ }\n");
    assert!(git(&root, &["init", "-q"]));
    assert!(git(&root, &["add", "flake.nix"]));
    root
}

#[test]
fn detect_then_seed_once() {
    let root = flake("seed");
    let rows = actlith(&root, &["detect"]);
    assert_eq!((rows.code, &*rows.out), (Some(0), "ci|gate|-|-|-\n"));
    assert_eq!(actlith(&root, &["seed", "--init"]).code, Some(0));
    let again = actlith(&root, &["seed", "--init"]);
    assert_eq!(again.code, Some(1));
    assert!(again.err.contains("already exists"));
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn import_checks_the_file_is_named_for_its_row() {
    let root = generated("import");
    let ci = actlith(&root, &["import", ".github/workflows/ci.yml"]);
    assert_eq!((ci.code, &*ci.out), (Some(0), "ci|gate|-|pr0d1r2|-\n"));
    let text = fs::read_to_string(root.join(".github/workflows/ci.yml"));
    put(&root, "gate.yml", &text.unwrap_or_default());
    let renamed = actlith(&root, &["import", "gate.yml"]);
    assert_eq!(renamed.code, Some(1));
    assert!(renamed.err.contains("fix: rename the file to `ci.yml`"));
    let _ = fs::remove_dir_all(&root);
}

/// The fake as `root/bin/gh`, executable; `PATH` with it first.
fn fake_gh(root: &Path) -> String {
    let bin = root.join("bin");
    put(&bin, "gh", &format!("#!/bin/sh\n{FAKE_GH}"));
    let mode = fs::Permissions::from_mode(0o755);
    assert!(fs::set_permissions(bin.join("gh"), mode).is_ok());
    let path = std::env::var("PATH").unwrap_or_default();
    format!("{}:{path}", bin.display())
}

#[test]
fn pin_dry_run_asks_gh_on_path_and_writes_nothing() {
    let root = scratch("pin");
    put(&root, ".actlith", REPO);
    let mut command = hermetic(Command::new(BIN));
    command.args(["pin", "--dry-run"]).current_dir(&root);
    let rows = run(command.env("PATH", fake_gh(&root)));
    assert_eq!(rows.code, Some(0), "{}", rows.err);
    let sha = "0123456789abcdef0123456789abcdef01234567";
    let row = format!("actions/checkout|v7.0.1|{sha}\n");
    assert!(rows.out.contains(&row), "{}", rows.out);
    let text = fs::read_to_string(root.join(".actlith")).unwrap_or_default();
    assert_eq!(text, REPO);
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn completions_and_man_print() {
    let zsh = actlith(Path::new("/"), &["completions", "zsh"]);
    assert_eq!(zsh.code, Some(0));
    assert!(zsh.out.contains("#compdef actlith"));
    let man = actlith(Path::new("/"), &["man"]);
    assert!(man.out.contains(".TH actlith"));
}
