//! The https push step (V61): only for an https cache with a push secret
//! in a mode that pushes; its netrc and url reach the script through
//! `env:`, never `run:`.

use crate::core::cache::{BinaryCache, builtin};
use crate::kind::{
    CachixPush, PUSH_SCRIPT, TRUSTED, https_push, push_https, url,
};
use crate::registry::parse;
use crate::yaml::{Action, Expr, GITHUB, Step, Text};

const CACHES: &str =
    include_str!("../../../../tests/fixtures/kind/caches.actlith");

const SHELL: &str = "devShells.default.inputDerivation";

/// The `## caches` row `name` of the caches fixture.
fn row_cache(name: &str) -> Result<BinaryCache, String> {
    let registry = parse(CACHES).map_err(|e| e.to_string())?;
    registry
        .caches
        .find(name)
        .ok_or_else(|| format!("no cache {name}"))
}

fn step(name: &str, mode: CachixPush) -> Result<Option<Step>, String> {
    let cache = row_cache(name)?;
    push_https(3, &cache, mode, SHELL).map_err(|e| e.to_string())
}

fn secret(name: &str) -> Option<Text> {
    Expr::new(&GITHUB, &format!("secrets.{name}")).map(Text::expr)
}

#[test]
fn a_netrc_secret_pushes_through_the_seeded_script() -> Result<(), String> {
    let step = step("example-cache", CachixPush::SkipForkPr)?.ok_or("none")?;
    let command = format!("bash {PUSH_SCRIPT} {SHELL}");
    assert_eq!(step.action, Action::Run(command));
    assert_eq!(step.env.get("NETRC"), secret("EXAMPLE_NETRC").as_ref());
    let literal = Text::lit("https://cache.example.org");
    assert_eq!(step.env.get("URL"), Some(&literal));
    assert_eq!(step.condition, Expr::new(&GITHUB, TRUSTED));
    assert_eq!(PUSH_SCRIPT, "scripts/actlith/push-https.sh");
    Ok(())
}

#[test]
fn the_step_names_the_cache_and_netrc_secret() -> Result<(), String> {
    let step = step("example-cache", CachixPush::SkipForkPr)?.ok_or("none")?;
    // The names the run-time https check reports (V65), never the url.
    let cache = Text::lit("example-cache");
    assert_eq!(step.env.get("CACHE"), Some(&cache));
    let secret_name = Text::lit("EXAMPLE_NETRC");
    assert_eq!(step.env.get("NETRC_SECRET"), Some(&secret_name));
    Ok(())
}

#[test]
fn main_only_pushes_from_a_push_to_main() -> Result<(), String> {
    for mode in [CachixPush::MainOnly, CachixPush::ExplicitMain] {
        let step = step("hidden-cache", mode)?.ok_or("none")?;
        let main = "github.event_name == 'push' && \
                    github.ref == 'refs/heads/main'";
        assert_eq!(step.condition, Expr::new(&GITHUB, main));
        assert_eq!(step.env.get("URL"), secret("HIDDEN_URL").as_ref());
        assert!(step.comments.iter().any(|c| c.inline().contains("main")));
    }
    Ok(())
}

#[test]
fn nothing_pushes_without_a_secret_an_https_cache_or_a_mode()
-> Result<(), String> {
    for name in ["read-cache", "own-cachix", "read-cachix"] {
        assert_eq!(step(name, CachixPush::SkipForkPr)?, None, "{name}");
    }
    assert_eq!(step("example-cache", CachixPush::ReadOnly)?, None);
    let built = builtin("pr0d1r2").ok_or("no built-in")?;
    let none = push_https(1, &built, CachixPush::SkipForkPr, SHELL);
    assert_eq!(none.map_err(|e| e.to_string())?, None);
    Ok(())
}

#[test]
fn https_push_says_when_the_script_is_needed() -> Result<(), String> {
    let https = row_cache("example-cache")?;
    assert!(https_push(Some(&https), CachixPush::MainOnly));
    assert!(!https_push(Some(&https), CachixPush::ReadOnly));
    let read = row_cache("read-cache")?;
    assert!(!https_push(Some(&read), CachixPush::MainOnly));
    assert!(!https_push(None, CachixPush::MainOnly));
    Ok(())
}

#[test]
fn the_url_is_the_literal_or_its_secret() -> Result<(), String> {
    let literal = url(&row_cache("example-cache")?);
    assert_eq!(literal, Text::lit("https://cache.example.org"));
    assert_eq!(Some(url(&row_cache("hidden-cache")?)), secret("HIDDEN_URL"));
    Ok(())
}

#[test]
fn an_attribute_with_an_expression_is_refused() -> Result<(), String> {
    let cache = row_cache("example-cache")?;
    let bad = push_https(9, &cache, CachixPush::MainOnly, "a${{ x }}");
    assert!(bad.is_err());
    Ok(())
}

#[test]
fn a_private_cache_pushes_with_its_push_secret_only() -> Result<(), String> {
    let push = step("private-cache", CachixPush::MainOnly)?.ok_or("none")?;
    assert_eq!(push.env.get("NETRC"), secret("PRIVATE_PUSH").as_ref());
    let name = Text::lit("PRIVATE_PUSH");
    assert_eq!(push.env.get("NETRC_SECRET"), Some(&name));
    assert!(step("private-read", CachixPush::MainOnly)?.is_none());
    Ok(())
}
