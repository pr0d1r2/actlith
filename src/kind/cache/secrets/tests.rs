//! The secrets a cache's steps read (`src/kind:V14`): read secrets
//! always, the push secret only where the mode pushes (V60, V61).

use crate::core::cache::{BinaryCache, builtin};
use crate::kind::{CachixPush, push_secret, secrets};
use crate::registry::parse;

const CACHES: &str =
    include_str!("../../../../tests/fixtures/kind/caches.actlith");

/// The `## caches` row `name` of the caches fixture.
fn row_cache(name: &str) -> Result<BinaryCache, String> {
    let registry = parse(CACHES).map_err(|e| e.to_string())?;
    registry
        .caches
        .find(name)
        .ok_or_else(|| format!("no cache {name}"))
}

/// `(name, purpose)` of each secret, in order.
fn named(cache: &BinaryCache, mode: CachixPush) -> Vec<(String, String)> {
    let all = secrets(Some(cache), mode).into_iter();
    all.map(|s| (s.name().to_owned(), s.purpose().to_owned()))
        .collect()
}

fn pairs(want: &[(&str, &str)]) -> Vec<(String, String)> {
    want.iter()
        .map(|(n, p)| ((*n).to_owned(), (*p).to_owned()))
        .collect()
}

#[test]
fn the_built_in_cache_reads_cachix_auth_token_when_pushing()
-> Result<(), String> {
    let cache = builtin("pr0d1r2").ok_or("no built-in")?;
    let want = pairs(&[("CACHIX_AUTH_TOKEN", "cachix push")]);
    assert_eq!(named(&cache, CachixPush::SkipForkPr), want);
    assert!(named(&cache, CachixPush::ReadOnly).is_empty());
    Ok(())
}

#[test]
fn read_secrets_come_first_and_stay_when_read_only() -> Result<(), String> {
    let cache = row_cache("hidden-cache")?;
    let read = [("HIDDEN_URL", "cache read"), ("HIDDEN_KEY", "cache read")];
    let mut all = pairs(&read);
    all.extend(pairs(&[("HIDDEN_NETRC", "cache push (netrc)")]));
    assert_eq!(named(&cache, CachixPush::MainOnly), all);
    assert_eq!(named(&cache, CachixPush::ReadOnly), pairs(&read));
    Ok(())
}

#[test]
fn a_cachix_row_names_its_token() -> Result<(), String> {
    let cache = row_cache("own-cachix")?;
    let want = pairs(&[("OWN_CACHIX_TOKEN", "cachix push")]);
    assert_eq!(named(&cache, CachixPush::SkipForkPr), want);
    Ok(())
}

#[test]
fn no_cache_or_a_read_only_one_reads_nothing() -> Result<(), String> {
    assert!(secrets(None, CachixPush::SkipForkPr).is_empty());
    for name in ["read-cache", "read-cachix", "example-cache"] {
        let cache = row_cache(name)?;
        let reads = named(&cache, CachixPush::ReadOnly);
        assert!(reads.is_empty(), "{name}: {reads:?}");
    }
    assert!(push_secret(&row_cache("read-cache")?).is_none());
    Ok(())
}

#[test]
fn a_read_netrc_is_its_own_secret_beside_the_push_one() -> Result<(), String> {
    let cache = row_cache("private-cache")?;
    let read = pairs(&[("PRIVATE_READ", "cache read (netrc)")]);
    let mut all = read.clone();
    all.extend(pairs(&[("PRIVATE_PUSH", "cache push (netrc)")]));
    assert_eq!(named(&cache, CachixPush::SkipForkPr), all);
    assert_eq!(named(&cache, CachixPush::ReadOnly), read);
    Ok(())
}

#[test]
fn a_read_netrc_follows_the_url_and_key_secrets() -> Result<(), String> {
    let cache = row_cache("private-read")?;
    let want = pairs(&[
        ("PRIVATE_URL", "cache read"),
        ("PRIVATE_READ", "cache read (netrc)"),
    ]);
    assert_eq!(named(&cache, CachixPush::SkipForkPr), want);
    assert!(push_secret(&cache).is_none(), "read-only");
    Ok(())
}
