//! The `build` row checks (T20): a cache is required, `package` is an
//! attribute name and one shell word, `timeout` is bounded.

use super::{Plan, attribute, cache, package};
use crate::build::Build;
use crate::kind::{BUILD, EXTRAS, KindError, Row, parse_args};
use crate::pin::PinTable;
use crate::registry::parse;

const ROWS: &str = include_str!("../../../tests/fixtures/build/rows.actlith");

fn row(id: &str) -> Result<Row, String> {
    let registry = parse(ROWS).map_err(|e| e.to_string())?;
    let found = registry.workflows.iter().find(|w| w.id == id);
    crate::kind::check(&Build, found.ok_or("no such row")?)
        .map_err(|e| e.to_string())
}

/// The args of row `id`, schema-checked only: the row rules that
/// `check` runs first are skipped, so `Plan`'s own checks answer.
fn unchecked(id: &str) -> Result<crate::kind::Args, String> {
    let registry = parse(ROWS).map_err(|e| e.to_string())?;
    let found = registry.workflows.iter().find(|w| w.id == id);
    parse_args(&BUILD, found.ok_or("no such row")?, &EXTRAS)
        .map_err(|e| e.to_string())
}

fn refused(id: &str) -> String {
    let pins = PinTable::builtin();
    match row(id) {
        Ok(row) => Plan::new(&row, &pins).err().map(|e| e.to_string()),
        Err(e) => Some(e),
    }
    .unwrap_or_default()
}

#[test]
fn a_good_row_plans_its_cache_package_and_timeout() -> Result<(), String> {
    let (row, pins) = (row("fine")?, PinTable::builtin());
    let plan = Plan::new(&row, &pins).map_err(|e| e.to_string())?;
    assert_eq!(plan.cache.name(), "pr0d1r2");
    assert_eq!((plan.package, plan.minutes), ("nix-hk_2", 30));
    assert_eq!(plan.row.id, "fine");
    Ok(())
}

#[test]
fn a_row_without_a_cache_is_refused() {
    let error = refused("nocache");
    assert!(error.contains(".actlith:7"), "{error}");
    assert!(error.contains("needs a cache"), "{error}");
}

#[test]
fn a_row_without_a_package_is_refused() {
    let error = refused("nopackage");
    assert!(error.contains("needs arg `package`"), "{error}");
}

#[test]
fn a_package_that_is_not_one_word_is_refused() {
    for id in ["spliced", "digit"] {
        let error = refused(id);
        assert!(error.contains("not a flake attribute name"), "{error}");
    }
}

#[test]
fn a_timeout_past_the_runner_limit_is_refused() {
    let error = refused("slow");
    assert!(error.contains("timeout `999`"), "{error}");
}

#[test]
fn the_parts_answer_alone() -> Result<(), String> {
    let good = row("fine")?;
    assert_eq!(package(&good.args), Ok("nix-hk_2"));
    assert_eq!(
        cache(&good).map(|c| c.name().to_owned()),
        Ok("pr0d1r2".into())
    );
    let bare = Row {
        cache: None,
        ..good.clone()
    };
    assert!(matches!(cache(&bare), Err(KindError::Row(_))));
    Ok(())
}

#[test]
fn a_bad_package_alone_is_refused_in_actlith_words() -> Result<(), String> {
    let spliced = unchecked("spliced")?;
    let error = package(&spliced).err().map(|e| e.to_string());
    assert_eq!(
        error.as_deref(),
        Some(
            ".actlith:9: package `hk;rm` is not a flake attribute name\n  \
             fix: use letters, digits, `_` and `-`, starting with a letter \
             or `_`"
        )
    );
    Ok(())
}

#[test]
fn only_attribute_names_that_are_shell_words_pass() {
    for name in ["hk", "nix-hk", "_x1", "a-b_c9"] {
        assert!(attribute(name), "{name}");
    }
    for name in ["", "1x", "-x", "a b", "a;b", "$(x)", "a'b", "a.b", "a\"b"] {
        assert!(!attribute(name), "{name}");
    }
}
