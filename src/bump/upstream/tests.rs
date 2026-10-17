//! Kind `upstream` (T23, V26, `src/kind:V3`, `src/kind:V14`): golden
//! files, the declared secret, and every refused row.

use crate::bump::Upstream;
use crate::kind::{Kind, Row};
use crate::pin::PinTable;
use crate::registry::parse;
use crate::system::System;
use crate::yaml::render;

const ROWS: &str =
    include_str!("../../../tests/fixtures/bump/upstream.actlith");
const DEFAULT: &str = include_str!("../../../tests/fixtures/bump/upstream.yml");
const CUSTOM: &str = include_str!("../../../tests/fixtures/bump/up-custom.yml");

fn checked(id: &str) -> Result<Row, String> {
    let registry = parse(ROWS).map_err(|e| e.to_string())?;
    let row = registry.workflows.iter().find(|r| r.id == id);
    crate::kind::check(&Upstream, row.ok_or("no row")?)
        .map_err(|e| e.to_string())
}

fn built(id: &str) -> Result<String, String> {
    let workflow = Upstream.build(&checked(id)?, &PinTable::builtin());
    workflow.map(|w| render(&w)).map_err(|e| e.to_string())
}

fn refusal(id: &str) -> Result<String, String> {
    match built(id) {
        Ok(text) => Err(format!("built:\n{text}")),
        Err(error) => Ok(error),
    }
}

#[test]
fn the_default_row_renders_the_golden_file() -> Result<(), String> {
    assert_eq!(built("upstream")?, DEFAULT);
    Ok(())
}

#[test]
fn every_arg_and_the_cache_render_the_golden_file() -> Result<(), String> {
    assert_eq!(built("up-custom")?, CUSTOM);
    Ok(())
}

#[test]
fn runs_are_never_cancelled_and_the_cache_never_pushed() -> Result<(), String> {
    let text = built("up-custom")?;
    assert!(text.contains("cancel-in-progress: false"), "{text}");
    assert!(text.contains("skipPush: true"), "{text}");
    assert!(!text.contains("CACHIX_AUTH_TOKEN"), "{text}");
    Ok(())
}

#[test]
fn it_declares_the_token_and_no_cachix_secret() -> Result<(), String> {
    let names = |id| -> Result<Vec<String>, String> {
        let secrets = Upstream.secrets(&checked(id)?);
        Ok(secrets.iter().map(|s| s.name().to_owned()).collect())
    };
    assert_eq!(names("upstream")?, ["GITHUB_TOKEN"]);
    assert_eq!(names("up-custom")?, ["BUMP_TOKEN"]);
    Ok(())
}

#[test]
fn a_row_without_script_is_refused() {
    let error = checked("no-script").err().unwrap_or_default();
    assert!(error.contains("needs arg `script`"), "{error}");
}

#[test]
fn a_prefix_or_script_that_is_not_a_plain_word_is_refused() -> Result<(), String>
{
    assert!(refusal("bad-prefix")?.contains("arg `branch-prefix`"));
    assert!(refusal("bad-script")?.contains("arg `script`"));
    Ok(())
}

#[test]
fn a_cron_that_is_not_five_fields_is_refused() -> Result<(), String> {
    assert!(refusal("bad-cron")?.contains("cron `daily`"));
    Ok(())
}

#[test]
fn the_kind_names_its_schema_and_default_system() {
    let keys: Vec<&str> =
        Upstream.spec().schema.iter().map(|s| s.key).collect();
    assert_eq!(
        keys,
        ["script", "cron", "branch-prefix", "token", "timeout"]
    );
    assert_eq!(Upstream.name(), "upstream");
    assert_eq!(Upstream.spec().default_systems, [System::X86_64Linux]);
}

#[test]
fn the_open_step_passes_every_value_through_env() -> Result<(), String> {
    let text = built("upstream")?;
    let open = "run: 'bash scripts/actlith/open-pr.sh \"$BRANCH\" \"$TITLE\" \
                \"$BODY\"'";
    assert!(text.contains(open), "{text}");
    assert!(text.contains("BRANCH: upstream/${{ steps.bump.outputs"));
    Ok(())
}
