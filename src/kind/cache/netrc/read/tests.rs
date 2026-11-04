//! The read netrc of a private https cache (V66): a trusted-only step
//! writes it through the seeded script before install-nix, its secret
//! apart from the push one, and the trusted nix config points at it.

use crate::core::cache::{BinaryCache, builtin};
use crate::kind::TRUSTED;
use crate::kind::cache::netrc::{
    READ_SCRIPT, netrc_file, read_netrc, read_secret,
};
use crate::registry::parse;
use crate::yaml::{Action, Expr, GITHUB, Step, Text};

const CACHES: &str =
    include_str!("../../../../../tests/fixtures/kind/caches.actlith");

/// The `## caches` row `name` of the caches fixture.
fn row_cache(name: &str) -> Result<BinaryCache, String> {
    let registry = parse(CACHES).map_err(|e| e.to_string())?;
    registry
        .caches
        .find(name)
        .ok_or_else(|| format!("no cache {name}"))
}

fn step(name: &str) -> Result<Option<Step>, String> {
    read_netrc(3, &row_cache(name)?).map_err(|e| e.to_string())
}

fn secret(name: &str) -> Option<Text> {
    Expr::new(&GITHUB, &format!("secrets.{name}")).map(Text::expr)
}

#[test]
fn a_read_netrc_is_written_by_the_seeded_script() -> Result<(), String> {
    let step = step("private-cache")?.ok_or("no step")?;
    let command = format!("bash {READ_SCRIPT}");
    assert_eq!(step.action, Action::Run(command));
    assert_eq!(READ_SCRIPT, "scripts/actlith/read-netrc.sh");
    assert_eq!(step.condition, Expr::new(&GITHUB, TRUSTED));
    assert!(!step.comments.is_empty(), "the step says why");
    Ok(())
}

#[test]
fn the_step_reads_the_read_secret_not_the_push_one() -> Result<(), String> {
    let step = step("private-cache")?.ok_or("no step")?;
    assert_eq!(step.env.get("NETRC"), secret("PRIVATE_READ").as_ref());
    let name = Text::lit("PRIVATE_READ");
    assert_eq!(step.env.get("NETRC_SECRET"), Some(&name));
    assert_eq!(step.env.get("CACHE"), Some(&Text::lit("private-cache")));
    let url = Text::lit("https://private.example.org");
    assert_eq!(step.env.get("URL"), Some(&url));
    Ok(())
}

#[test]
fn a_secret_url_reaches_the_step_as_its_secret() -> Result<(), String> {
    let step = step("private-read")?.ok_or("no step")?;
    assert_eq!(step.env.get("URL"), secret("PRIVATE_URL").as_ref());
    Ok(())
}

#[test]
fn a_cache_without_a_read_netrc_gets_no_step() -> Result<(), String> {
    for name in ["example-cache", "hidden-cache", "read-cache", "own-cachix"] {
        assert!(step(name)?.is_none(), "{name}");
        assert!(read_secret(&row_cache(name)?).is_none(), "{name}");
    }
    let cache = builtin("pr0d1r2").ok_or("no built-in")?;
    assert!(read_netrc(3, &cache).map_err(|e| e.to_string())?.is_none());
    Ok(())
}

#[test]
fn the_read_secret_has_its_own_purpose() -> Result<(), String> {
    let read = read_secret(&row_cache("private-cache")?).ok_or("none")?;
    assert_eq!(read.name(), "PRIVATE_READ");
    assert_eq!(read.purpose(), "cache read (netrc)");
    Ok(())
}

#[test]
fn the_nix_config_points_at_the_file_the_script_writes() -> Result<(), String> {
    let text = netrc_file(Text::lit("a\n"), &row_cache("private-cache")?);
    let temp = Expr::new(&GITHUB, "runner.temp").ok_or("no expr")?;
    let want = Text::lit("a\nnetrc-file = ")
        .then_expr(temp)
        .then_lit("/read-netrc\n");
    assert_eq!(text, want);
    let plain = netrc_file(Text::lit("a\n"), &row_cache("hidden-cache")?);
    assert_eq!(plain, Text::lit("a\n"));
    Ok(())
}
