//! `render` end to end (T38, V31): the fixture `.actlith` renders the
//! fixture file set under `tests/fixtures/render/`, every rendered
//! workflow passes `actionlint` and `zizmor --offline` (the dev shell
//! carries both; a missing tool fails the test), and this repository's
//! own row renders its `ci.yml`. Then every refusal names its row.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::registry::{ActionPin, Registry, parse};
use crate::render::{File, RenderError, render};

const ALL: &str = include_str!("../../../tests/fixtures/render/all.actlith");
const OWN: &str = include_str!("../../../tests/fixtures/render/ci.actlith");
const CI: &str = include_str!("../../../.github/workflows/ci.yml");

/// Every path the fixture renders, in the order `render` returns them.
const PATHS: [&str; 10] = [
    ".coderabbit.yaml",
    ".github/workflows/build.yml",
    ".github/workflows/ci.yml",
    ".github/workflows/update-pins.yml",
    ".github/workflows/upstream.yml",
    "greptile.json",
    "scripts/actlith/open-pr.sh",
    "scripts/actlith/update-pins.sh",
    "scripts/actlith/verify-cache.sh",
    "socket.yml",
];

const HEAD: &str = "format 1\n\n## workflows\nid|kind|systems|cache|args\n";

/// A hook's git environment, dropped from every tool the tests run.
const GIT_ENV: [&str; 6] = [
    "GIT_DIR",
    "GIT_WORK_TREE",
    "GIT_INDEX_FILE",
    "GIT_OBJECT_DIRECTORY",
    "GIT_ALTERNATE_OBJECT_DIRECTORIES",
    "GIT_COMMON_DIR",
];

fn registry(text: &str) -> Result<Registry, String> {
    parse(text).map_err(|e| e.to_string())
}

fn rendered(text: &str) -> Result<Vec<File>, String> {
    render(&registry(text)?).map_err(|e| e.to_string())
}

fn refused(text: &str) -> Result<RenderError, String> {
    match render(&registry(text)?) {
        Ok(files) => Err(format!("rendered {} files", files.len())),
        Err(error) => Ok(error),
    }
}

fn rows(cells: &str) -> String {
    format!("{HEAD}{cells}\n")
}

/// The fixture file of `path`: a leading `.` is stored as `dot-`, so no
/// fixture sits where a tool takes it for this repository's own file.
fn fixture(path: &str) -> PathBuf {
    let stored = path
        .strip_prefix('.')
        .map_or_else(|| path.to_owned(), |rest| format!("dot-{rest}"));
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/render")
        .join(stored)
}

fn read(path: &Path) -> Result<String, String> {
    std::fs::read_to_string(path)
        .map_err(|e| format!("{}: {e}", path.display()))
}

/// `tool args... file`, without the hook's git environment.
fn lint(tool: &str, args: &[&str], file: &Path) -> Result<(), String> {
    let mut command = Command::new(tool);
    for var in GIT_ENV {
        command.env_remove(var);
    }
    let out = command
        .args(args)
        .arg(file)
        .output()
        .map_err(|e| format!("{tool}: {e} (run inside `nix develop`)"))?;
    if out.status.success() {
        return Ok(());
    }
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    Err(format!("{tool} {}:\n{stdout}{stderr}", file.display()))
}

fn workflows() -> impl Iterator<Item = PathBuf> {
    PATHS
        .into_iter()
        .filter(|p| p.starts_with(".github/workflows/"))
        .map(fixture)
}

#[test]
fn the_fixture_renders_the_fixture_file_set() -> Result<(), String> {
    let files = rendered(ALL)?;
    let paths: Vec<&str> = files.iter().map(|f| f.path.as_str()).collect();
    assert_eq!(paths, PATHS);
    for file in &files {
        assert_eq!(file.bytes, read(&fixture(&file.path))?, "{}", file.path);
    }
    Ok(())
}

#[test]
fn every_rendered_workflow_passes_actionlint() -> Result<(), String> {
    for path in workflows() {
        lint("actionlint", &["-no-color"], &path)?;
    }
    Ok(())
}

#[test]
fn every_rendered_workflow_passes_zizmor_offline() -> Result<(), String> {
    for path in workflows() {
        lint("zizmor", &["--offline", "--no-progress"], &path)?;
    }
    Ok(())
}

#[test]
fn the_own_row_renders_the_committed_ci_yml_exactly() -> Result<(), String> {
    let files = rendered(OWN)?;
    let [file] = files.as_slice() else {
        return Err(format!("{} files", files.len()));
    };
    assert_eq!(file.path, ".github/workflows/ci.yml");
    // Dogfood (nix T31): ci.yml is generated, header included.
    assert_eq!(file.bytes, CI);
    Ok(())
}

#[test]
fn a_gate_row_needs_no_script() -> Result<(), String> {
    let files = rendered(&rows("ci|gate|-|-|-"))?;
    let paths: Vec<&str> = files.iter().map(|f| f.path.as_str()).collect();
    assert_eq!(paths, [".github/workflows/ci.yml"]);
    Ok(())
}

#[test]
fn a_script_two_kinds_need_is_written_once() -> Result<(), String> {
    let text = rows(
        "a|update-pins|-|-|-\nb|upstream|-|-|script=s.sh\n\
         c|upstream|-|-|script=t.sh",
    );
    let files = rendered(&text)?;
    let open = "scripts/actlith/open-pr.sh";
    assert_eq!(files.iter().filter(|f| f.path == open).count(), 1);
    assert_eq!(files.len(), 5);
    Ok(())
}

#[test]
fn an_unknown_kind_names_its_row_and_the_kinds() -> Result<(), String> {
    let error = refused(&rows("ci|gate|-|-|-\nlint|lint|-|-|-"))?;
    assert_eq!(
        error.to_string(),
        ".actlith:6: unknown kind `lint`\n  fix: use one of `gate`, \
         `build`, `update-pins`, `upstream`"
    );
    Ok(())
}

#[test]
fn an_unknown_or_unhosted_system_names_its_row() -> Result<(), String> {
    for systems in ["riscv64-linux", "x86_64-darwin"] {
        let error = refused(&rows(&format!("ci|gate|{systems}|-|-")))?;
        assert!(matches!(error, RenderError::Kind(_)), "{error}");
        assert!(error.to_string().starts_with(".actlith:5: "), "{error}");
    }
    Ok(())
}

#[test]
fn a_row_its_kind_refuses_is_a_kind_error() -> Result<(), String> {
    let error = refused(&rows("b|build|-|-|-"))?;
    assert!(matches!(error, RenderError::Kind(_)), "{error}");
    Ok(())
}

#[test]
fn a_bad_pin_override_is_refused() -> Result<(), String> {
    let mut pinned = registry(&rows("ci|gate|-|-|-"))?;
    pinned.actions.push(ActionPin {
        line: 9,
        action: "actions/checkout".to_owned(),
        version: "v1".to_owned(),
        sha: "main".to_owned(),
    });
    let error = render(&pinned).err().ok_or("rendered")?;
    assert!(matches!(error, RenderError::Pin(_)), "{error}");
    Ok(())
}

#[test]
fn a_bad_bot_row_is_refused() -> Result<(), String> {
    let error = refused("format 1\n\n## bots\nbot|args\nclippy|-\n")?;
    assert!(matches!(error, RenderError::Bot(_)), "{error}");
    Ok(())
}

#[test]
fn two_rows_at_one_path_are_refused() -> Result<(), String> {
    let mut twice = registry(&rows("ci|gate|-|-|-"))?;
    twice.workflows.extend(twice.workflows.clone());
    let error = render(&twice).err().ok_or("rendered")?;
    let path = ".github/workflows/ci.yml".to_owned();
    assert_eq!(error, RenderError::Duplicate(path));
    Ok(())
}

#[test]
fn file_is_pipelith_file() {
    use std::any::TypeId;
    let ours = TypeId::of::<crate::render::File>();
    assert_eq!(ours, TypeId::of::<crate::core::files::File>());
}
