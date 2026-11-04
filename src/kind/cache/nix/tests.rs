//! install-nix trusts the row's cache (T17): literal values as they are,
//! a secret one only where the run gets it, a fork PR without the cache
//! (V60).

use crate::core::cache::{BinaryCache, builtin};
use crate::kind::{
    KindError, SKIP_FORK_PR, TRUSTED, install_nix, job, workflow,
};
use crate::pin::PinTable;
use crate::registry::parse;
use crate::yaml::{
    Action, Expr, GITHUB, HEADER, Input, Step, Text, Trigger, Value, With,
    render,
};

const CACHES: &str =
    include_str!("../../../../tests/fixtures/kind/caches.actlith");

fn with(step: &Step) -> With {
    match &step.action {
        Action::Uses(u) => u.with.clone(),
        Action::Run(_) => With::default(),
    }
}

/// The `## caches` row `name` of the caches fixture.
fn row_cache(name: &str) -> Result<BinaryCache, String> {
    let registry = parse(CACHES).map_err(|e| e.to_string())?;
    registry
        .caches
        .find(name)
        .ok_or_else(|| format!("no cache {name}"))
}

fn steps(cache: Option<&BinaryCache>) -> Result<Vec<Step>, String> {
    install_nix(1, &PinTable::builtin(), cache).map_err(|e| e.to_string())
}

/// The steps as the writer prints them.
fn printed(steps: Vec<Step>) -> String {
    let jobs = [("j".to_owned(), job(Text::lit("r"), 1, steps))].into();
    render(&workflow("w", Trigger::default(), jobs)).replace(HEADER, "")
}

#[test]
fn with_a_cache_the_substituter_is_trusted() -> Result<(), String> {
    let cache = builtin("pr0d1r2").ok_or("no built-in")?;
    let steps = steps(Some(&cache))?;
    let config = cache.nix_config().ok_or("not literal")?;
    let config = Value::Text(Text::lit(&config));
    let want = With::default().set("extra_nix_config", Input::new(config));
    let [step] = steps.as_slice() else {
        return Err(format!("{} steps", steps.len()));
    };
    assert_eq!(with(step), want);
    assert_eq!(step.condition, None);
    Ok(())
}

#[test]
fn without_a_cache_nothing_extra_is_trusted() -> Result<(), String> {
    let steps = steps(None)?;
    let [step] = steps.as_slice() else {
        return Err(format!("{} steps", steps.len()));
    };
    assert_eq!(with(step), With::default());
    Ok(())
}

#[test]
fn a_literal_https_cache_is_written_as_is() -> Result<(), String> {
    let text = printed(steps(Some(&row_cache("example-cache")?))?);
    let want = "\
          extra_nix_config: |
            extra-substituters = https://cache.example.org
            extra-trusted-public-keys = example-cache-1:";
    assert!(text.contains(want), "{text}");
    assert!(!text.contains("if:"), "{text}");
    assert!(!text.contains("secrets."), "{text}");
    Ok(())
}

#[test]
fn a_secret_value_is_read_only_off_forks() -> Result<(), String> {
    let steps = steps(Some(&row_cache("hidden-cache")?))?;
    let [trusted, fork] = steps.as_slice() else {
        return Err(format!("{} steps", steps.len()));
    };
    assert_eq!(trusted.condition, Expr::new(&GITHUB, TRUSTED));
    assert_eq!(fork.condition, Expr::new(&GITHUB, SKIP_FORK_PR));
    assert_eq!(with(fork), With::default(), "no empty substituter");
    Ok(())
}

#[test]
fn a_secret_value_is_its_expression_in_the_config() -> Result<(), String> {
    let text = printed(steps(Some(&row_cache("hidden-cache")?))?);
    let want = "\
          extra_nix_config: |
            extra-substituters = ${{ secrets.HIDDEN_URL }}
            extra-trusted-public-keys = ${{ secrets.HIDDEN_KEY }}
";
    assert!(text.contains(want), "{text}");
    assert!(text.contains("# A fork PR: no cache secret"), "{text}");
    Ok(())
}

#[test]
fn a_literal_url_beside_a_secret_key_keeps_both() -> Result<(), String> {
    let steps = steps(Some(&row_cache("half-cache")?))?;
    assert_eq!(steps.len(), 2, "a secret key is read off forks only");
    let want = "\
            extra-substituters = https://half.example.org
            extra-trusted-public-keys = ${{ secrets.HALF_KEY }}
";
    let text = printed(steps);
    assert!(text.contains(want), "{text}");
    Ok(())
}

#[test]
fn install_nix_without_a_pin_is_refused() -> Result<(), String> {
    let cache = row_cache("hidden-cache")?;
    let refused = install_nix(1, &PinTable::default(), Some(&cache));
    assert!(matches!(refused, Err(KindError::Pin(_))));
    Ok(())
}

#[test]
fn a_read_netrc_is_written_before_install_nix() -> Result<(), String> {
    let steps = steps(Some(&row_cache("private-cache")?))?;
    let [read, trusted, fork] = steps.as_slice() else {
        return Err(format!("{} steps", steps.len()));
    };
    let script = Action::Run("bash scripts/actlith/read-netrc.sh".to_owned());
    assert_eq!(read.action, script);
    assert_eq!(read.condition, Expr::new(&GITHUB, TRUSTED));
    assert_eq!(trusted.condition, Expr::new(&GITHUB, TRUSTED));
    assert_eq!(fork.condition, Expr::new(&GITHUB, SKIP_FORK_PR));
    assert_eq!(with(fork), With::default(), "a fork PR has no netrc");
    Ok(())
}

#[test]
fn the_trusted_config_points_nix_at_the_read_netrc() -> Result<(), String> {
    let text = printed(steps(Some(&row_cache("private-cache")?))?);
    let want = "\
          extra_nix_config: |
            extra-substituters = https://private.example.org
            extra-trusted-public-keys = private-cache-1:\
            79mE5RnWzUnV7Xlzq0cdEHe4GW4t0qBGsmoGOUr64p0=
            netrc-file = ${{ runner.temp }}/read-netrc
";
    assert!(text.contains(want), "{text}");
    assert!(text.contains("# The cache is read with a netrc"), "{text}");
    assert!(!text.contains("PRIVATE_PUSH"), "{text}");
    Ok(())
}
