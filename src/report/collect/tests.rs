//! `collect` (T28): each row checked and built by its kind, its systems,
//! cache, pinned actions and secrets (`src/kind:V14`) listed, every list
//! sorted (C); an unknown kind, a refused row or an unknown bot names its
//! line.

use super::collect;
use crate::pin::PinTable;
use crate::registry::parse;
use crate::report::model::{App, Hosted, Manual, Need, Report};

const HEAD: &str = "format 1\n\n## workflows\nid|kind|systems|cache|args\n";

fn report(text: &str) -> Result<Report, String> {
    let registry = parse(text).map_err(|e| e.to_string())?;
    let pins = PinTable::builtin()
        .with_overrides(&registry.actions)
        .map_err(|e| e.to_string())?;
    collect(&registry, &pins).map_err(|e| e.to_string())
}

fn rows(cells: &str) -> String {
    format!("{HEAD}{cells}\n")
}

fn one(cells: &str) -> Result<crate::report::model::Workflow, String> {
    let mut all = report(&rows(cells))?.workflows;
    all.pop().ok_or_else(|| "no workflow".to_owned())
}

#[test]
fn workflows_sort_by_id() -> Result<(), String> {
    let got = report(&rows("zz|gate|-|-|-\naa|gate|-|-|-"))?;
    let ids: Vec<&str> = got.workflows.iter().map(|w| w.id.as_str()).collect();
    assert_eq!(ids, ["aa", "zz"]);
    Ok(())
}

#[test]
fn a_row_lists_kind_systems_runners_cache() -> Result<(), String> {
    let got = one("ci|gate|aarch64-darwin, x86_64-linux|pr0d1r2|-")?;
    assert_eq!(got.kind, "gate");
    let hosted = |system: &str, runner| Hosted {
        system: system.to_owned(),
        runner,
    };
    assert_eq!(
        got.systems,
        [
            hosted("aarch64-darwin", "macos-latest"),
            hosted("x86_64-linux", "ubuntu-latest")
        ]
    );
    assert_eq!(got.cache.as_deref(), Some("pr0d1r2"));
    Ok(())
}

#[test]
fn a_dash_lists_default_systems_no_cache() -> Result<(), String> {
    let got = one("ci|gate|-|-|-")?;
    let names: Vec<&str> =
        got.systems.iter().map(|h| h.system.as_str()).collect();
    assert_eq!(names, ["x86_64-linux", "aarch64-linux", "aarch64-darwin"]);
    assert_eq!(got.cache, None);
    Ok(())
}

const SHA: &str = "11bd71901bbe5b1630ceea73d27597364c9af683";

/// A gate row with a cache, `actions/checkout` pinned by `## actions`.
fn overridden() -> String {
    format!(
        "{}\n## actions\naction|version|sha\nactions/checkout|v4.2.2|{SHA}\n",
        rows("ci|gate|-|pr0d1r2|-")
    )
}

#[test]
fn actions_sort_by_name_pinned_from_the_table() -> Result<(), String> {
    let got = report(&overridden())?.workflows;
    let actions = got.first().map(|w| w.actions.clone()).unwrap_or_default();
    let names: Vec<&str> = actions.iter().map(|u| u.action.as_str()).collect();
    assert_eq!(
        names,
        [
            "actions/checkout",
            "cachix/cachix-action",
            "cachix/install-nix-action"
        ]
    );
    let checkout = actions.first().map(|u| (u.version.as_str(), &*u.sha));
    assert_eq!(checkout, Some(("v4.2.2", SHA)));
    Ok(())
}

#[test]
fn a_pushing_cache_reads_the_cachix_token() -> Result<(), String> {
    let need = Need {
        name: "CACHIX_AUTH_TOKEN".to_owned(),
        purpose: "cachix push",
    };
    assert_eq!(one("ci|gate|-|pr0d1r2|-")?.secrets, [need]);
    assert_eq!(one("ci|gate|-|-|-")?.secrets, []);
    Ok(())
}

#[test]
fn a_bump_reads_the_token_its_row_names() -> Result<(), String> {
    let got = one("up|update-pins|-|-|token=PIN_TOKEN")?.secrets;
    let names: Vec<&str> = got.iter().map(|n| n.name.as_str()).collect();
    assert_eq!(names, ["PIN_TOKEN"]);
    Ok(())
}

#[test]
fn hand_written_rows_sort_by_path() -> Result<(), String> {
    let text = format!(
        "{HEAD}\n## hand-written\npath|reason\nz.yml|later\na.yml|first\n"
    );
    let manual = |path: &str, reason: &str| Manual {
        path: path.to_owned(),
        reason: reason.to_owned(),
    };
    assert_eq!(
        report(&text)?.hand_written,
        [manual("a.yml", "first"), manual("z.yml", "later")]
    );
    Ok(())
}

#[test]
fn bots_sort_by_name_with_their_app() -> Result<(), String> {
    let text = format!("{HEAD}\n## bots\nbot|args\nsocket|-\ngreptile|-\n");
    let app = |bot: &str, name| App {
        bot: bot.to_owned(),
        name,
        note: "install the app; free tier per docs/SERVICES.md",
    };
    assert_eq!(
        report(&text)?.bots,
        [app("greptile", "Greptile"), app("socket", "Socket")]
    );
    Ok(())
}

#[test]
fn an_unknown_kind_names_its_line() {
    assert_eq!(
        report(&rows("ci|deploy|-|-|-")),
        Err(
            ".actlith:5: unknown kind `deploy`\n  fix: use one of `gate`, \
             `build`, `update-pins`, `upstream`"
                .to_owned()
        )
    );
}

#[test]
fn a_row_its_kind_refuses_is_the_kind_error() {
    let got = report(&rows("ci|gate|x86_64-darwin|-|-"));
    assert!(got.is_err_and(|e| e.starts_with(".actlith:5: system")));
}

#[test]
fn an_unknown_bot_names_its_line() {
    let text = format!("{HEAD}\n## bots\nbot|args\nbugbot|-\n");
    assert_eq!(
        report(&text),
        Err(".actlith:8: unknown bot `bugbot`\n  fix: use one of \
             `coderabbit`, `greptile`, `socket`"
            .to_owned())
    );
}

#[test]
fn a_gate_row_lists_its_pinned_xcode() -> Result<(), String> {
    let pinned = one("ci|gate|aarch64-darwin|-|xcode=26.4")?;
    assert_eq!(pinned.xcode.as_deref(), Some("26.4"));
    assert_eq!(one("ci|gate|aarch64-darwin|-|-")?.xcode, None);
    Ok(())
}
