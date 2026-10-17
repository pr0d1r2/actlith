//! cachix push modes: fork-PR skip, main only, read-only; `pushFilter`
//! (T17, V14).

use crate::kind::{
    Cache, CachixPush, KindError, PUSH_FILTER, SKIP_FORK_PR, SKIP_UNLESS_MAIN,
    cachix, job, workflow,
};
use crate::pin::PinTable;
use crate::yaml::GITHUB;
use crate::yaml::{
    Action, Comment, Expr, HEADER, Input, Step, Text, Trigger, Value, With,
    render,
};

fn step(push: CachixPush) -> Result<Step, KindError> {
    cachix(&PinTable::builtin(), &Cache::parse(1, "pr0d1r2")?, push)
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
    assert_eq!(CachixPush::ReadOnly.secret(), None);
    Ok(())
}

#[test]
fn pushing_modes_declare_the_cachix_secret() {
    for push in [CachixPush::SkipForkPr, CachixPush::MainOnly] {
        let name = push.secret().map(|s| s.name().to_owned());
        assert_eq!(name.as_deref(), Some("CACHIX_AUTH_TOKEN"));
    }
}

#[test]
fn the_filter_and_skip_are_the_documented_ones() {
    assert_eq!(PUSH_FILTER, "(-source$|nixpkgs\\.tar\\.gz$)");
    assert!(SKIP_FORK_PR.starts_with("github.event_name == 'pull_request'"));
    assert!(Expr::new(&GITHUB, SKIP_FORK_PR).is_some());
}

#[test]
fn cachix_without_a_pin_is_refused() -> Result<(), KindError> {
    let cache = Cache::parse(1, "pr0d1r2")?;
    for push in [CachixPush::SkipForkPr, CachixPush::ReadOnly] {
        assert!(cachix(&PinTable::default(), &cache, push).is_err());
    }
    Ok(())
}
