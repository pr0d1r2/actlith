//! `CodeRabbit`'s docs (getting-started/yaml-configuration), quoted for the
//! keys the stub writes:
//!
//! ```yaml
//! remote_config:
//!   repository: "shared-configs"
//!   ref: "main"
//!   path: ".shared-coderabbit.yaml"
//! ```
//!
//! "A `repository:` source can be public or private, but the `CodeRabbit`
//! installation must be able to access it." Inheritance
//! (configuration/configuration-inheritance): `inheritance: true` makes
//! a level merge with its parent; schema.v2.json: "Use parent settings
//! for any values not defined here."

use super::{PATH, file};
use crate::registry::{Arg, BotRow};

const GOLDEN: &str =
    include_str!("../../../tests/fixtures/bots/coderabbit.yaml");

fn row(args: &[(&str, &str)]) -> BotRow {
    BotRow {
        line: 9,
        bot: "coderabbit".to_owned(),
        args: args
            .iter()
            .map(|(k, v)| Arg {
                key: (*k).to_owned(),
                value: (*v).to_owned(),
            })
            .collect(),
    }
}

fn refused(args: &[(&str, &str)]) -> (usize, String) {
    let err = file(&row(args)).err().unwrap_or_default();
    (err.line, err.what)
}

#[test]
fn central_renders_the_golden_stub() {
    let got = file(&row(&[("central", "pr0d1r2/coderabbit")]));
    assert_eq!(got.as_ref().map(|f| f.path.as_str()), Ok(PATH));
    assert_eq!(got.map(|f| f.bytes), Ok(GOLDEN.to_owned()));
}

#[test]
fn the_stub_starts_with_the_generated_header() {
    let got = file(&row(&[("central", "a/b")])).map(|f| f.bytes);
    let first = got.unwrap_or_default();
    assert!(first.starts_with(crate::yaml::HEADER));
}

#[test]
fn central_is_required() {
    let (line, what) = refused(&[]);
    assert_eq!(line, 9);
    assert_eq!(what, "bot `coderabbit` needs arg `central`");
}

#[test]
fn an_unknown_arg_is_refused() {
    let (line, what) = refused(&[("central", "a/b"), ("ref", "dev")]);
    assert_eq!(line, 9);
    assert_eq!(what, "bot `coderabbit` has no arg `ref`");
}

#[test]
fn central_without_owner_is_refused() {
    let (line, what) = refused(&[("central", "coderabbit")]);
    assert_eq!(line, 9);
    assert_eq!(what, "`central` `coderabbit` is not `owner/repo`");
}

#[test]
fn central_with_quote_or_extra_slash_is_refused() {
    for bad in ["a/b/c", "a/\"b", "/b", "a/", "a b/c"] {
        let (_, what) = refused(&[("central", bad)]);
        assert_eq!(what, format!("`central` `{bad}` is not `owner/repo`"));
    }
}
