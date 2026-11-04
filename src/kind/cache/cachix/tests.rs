//! cachix push modes: fork-PR skip, main only, read-only; `pushFilter`
//! (T17, `src/kind:V14`); the token a `## caches` row names, and no step
//! for a read-only or https cache (V61).

use crate::core::cache::{BinaryCache, builtin};
use crate::kind::{
    CachixPush, KindError, PUSH_FILTER, SKIP_FORK_PR, SKIP_UNLESS_MAIN, cachix,
    job, workflow,
};
use crate::pin::PinTable;
use crate::registry::parse;
use crate::yaml::GITHUB;
use crate::yaml::{
    Action, Comment, Expr, HEADER, Input, Step, Text, Trigger, Value, With,
    render,
};

const CACHES: &str =
    include_str!("../../../../tests/fixtures/kind/caches.actlith");

fn pr0d1r2() -> Result<BinaryCache, KindError> {
    builtin("pr0d1r2").ok_or_else(|| KindError::row(1, "no built-in", "-"))
}

fn step(push: CachixPush) -> Result<Step, KindError> {
    cachix(&PinTable::builtin(), &pr0d1r2()?, push)?
        .ok_or_else(|| KindError::row(1, "no cachix step", "-"))
}

/// The `## caches` row `name` of the caches fixture.
fn row_cache(name: &str) -> Result<BinaryCache, String> {
    let registry = parse(CACHES).map_err(|e| e.to_string())?;
    registry
        .caches
        .find(name)
        .ok_or_else(|| format!("no cache {name}"))
}

fn with(step: &Step) -> With {
    match &step.action {
        Action::Uses(u) => u.with.clone(),
        Action::Run(_) => With::default(),
    }
}

/// The step's `with:` lines as the writer prints them.
fn printed(step: Step) -> String {
    let job = job(Text::lit("r"), 1, vec![step]);
    let jobs = [("j".to_owned(), job)].into();
    render(&workflow("w", Trigger::default(), jobs)).replace(HEADER, "")
}

#[test]
fn fork_prs_skip_the_push_as_ci_yml_does() -> Result<(), KindError> {
    let text = printed(step(CachixPush::SkipForkPr)?);
    let want = "\
          name: pr0d1r2
          authToken: ${{ secrets.CACHIX_AUTH_TOKEN }}
          skipPush: >-
            ${{ github.event_name == 'pull_request' &&
            github.event.pull_request.head.repo.full_name != github.repository }}
          # Only what was built here; sources and tarballs stay out of the cache.
          pushFilter: '(-source$|nixpkgs\\.tar\\.gz$)'
";
    assert!(text.contains(want), "{text}");
    assert!(text.contains(
        "# them from source. Fork PRs get no secret and still read the cache.\n"
    ));
    Ok(())
}

#[test]
fn main_only_skips_every_other_run() -> Result<(), KindError> {
    let text = printed(step(CachixPush::MainOnly)?);
    let want = format!(
        "          authToken: ${{{{ secrets.CACHIX_AUTH_TOKEN }}}}\n          \
         skipPush: ${{{{ {SKIP_UNLESS_MAIN} }}}}\n"
    );
    assert!(text.contains(&want), "{text}");
    assert!(text.contains("# Push only from main"), "{text}");
    assert!(text.contains("pushFilter:"), "{text}");
    Ok(())
}

fn input(value: Value) -> Input {
    Input::new(value)
}

#[test]
fn read_only_never_pushes_nor_reads_a_secret() -> Result<(), KindError> {
    let step = step(CachixPush::ReadOnly)?;
    let want = With::default()
        .set("name", input(Value::Text(Text::lit("pr0d1r2"))))
        .set("skipPush", input(Value::Bool(true)));
    assert_eq!(with(&step), want);
    assert_eq!(step.comments, Vec::<Comment>::new());
    assert!(!CachixPush::ReadOnly.pushes());
    Ok(())
}

#[test]
fn every_other_mode_pushes() {
    for push in [
        CachixPush::SkipForkPr,
        CachixPush::MainOnly,
        CachixPush::ExplicitMain,
    ] {
        assert!(push.pushes(), "{push:?}");
    }
}

#[test]
fn a_row_token_secret_is_the_auth_token() -> Result<(), String> {
    let cache = row_cache("own-cachix")?;
    let step = cachix(&PinTable::builtin(), &cache, CachixPush::SkipForkPr)
        .map_err(|e| e.to_string())?
        .ok_or("no step")?;
    let text = printed(step);
    assert!(text.contains("          name: own-cachix\n"), "{text}");
    let token = "          authToken: ${{ secrets.OWN_CACHIX_TOKEN }}\n";
    assert!(text.contains(token), "{text}");
    assert!(!text.contains("CACHIX_AUTH_TOKEN"), "{text}");
    Ok(())
}

#[test]
fn a_read_only_or_https_cache_gets_no_cachix_step() -> Result<(), String> {
    for name in ["read-cachix", "example-cache", "read-cache"] {
        let cache = row_cache(name)?;
        for push in [CachixPush::SkipForkPr, CachixPush::ReadOnly] {
            let step = cachix(&PinTable::builtin(), &cache, push);
            assert_eq!(step.map_err(|e| e.to_string())?, None);
        }
    }
    Ok(())
}

#[test]
fn the_filter_and_skip_are_the_documented_ones() {
    assert_eq!(PUSH_FILTER, "(-source$|nixpkgs\\.tar\\.gz$)");
    assert!(SKIP_FORK_PR.starts_with("github.event_name == 'pull_request'"));
    assert!(Expr::new(&GITHUB, SKIP_FORK_PR).is_some());
}

#[test]
fn cachix_without_a_pin_is_refused() -> Result<(), KindError> {
    let cache = pr0d1r2()?;
    for push in [CachixPush::SkipForkPr, CachixPush::ReadOnly] {
        assert!(cachix(&PinTable::default(), &cache, push).is_err());
    }
    Ok(())
}
