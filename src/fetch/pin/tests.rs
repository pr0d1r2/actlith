//! `pin` with a fake resolver: rows, dry run, one-section rewrite and each
//! refusal (V16, T29). No network: the fake answers from tables.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::fetch::FetchError;
use crate::fetch::Resolver;
use crate::fetch::pin;
use crate::fetch::pin::BUILTIN_ACTIONS;
use crate::fetch::section::replace;
use crate::pin::PinTable;

const FULL: &str = include_str!("../../../tests/fixtures/fetch/full.actlith");
const NONE: &str =
    include_str!("../../../tests/fixtures/fetch/no-actions.actlith");
const BAD_SHA: &str =
    include_str!("../../../tests/fixtures/fetch/bad-sha.actlith");

/// A resolver answering from tables; anything else is an API error.
#[derive(Default)]
struct Fake {
    latest: BTreeMap<&'static str, &'static str>,
    shas: BTreeMap<(&'static str, &'static str), &'static str>,
}

impl Resolver for Fake {
    fn latest(&self, action: &str) -> Result<Option<String>, FetchError> {
        Ok(self.latest.get(action).map(|t| (*t).to_owned()))
    }

    fn sha(&self, action: &str, tag: &str) -> Result<String, FetchError> {
        let found = self.shas.get(&(action, tag));
        found
            .map(|s| (*s).to_owned())
            .ok_or_else(|| FetchError::Api {
                action: action.to_owned(),
                why: format!("no tag `{tag}`"),
            })
    }
}

const A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const B: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
const C: &str = "cccccccccccccccccccccccccccccccccccccccc";
const D: &str = "dddddddddddddddddddddddddddddddddddddddd";

/// checkout and install-nix have newer releases; cachix-action and
/// example/action have none, so their recorded versions stay.
fn fake() -> Fake {
    let latest = [
        ("actions/checkout", "v7.0.2"),
        ("cachix/install-nix-action", "v31.12.0"),
    ];
    let shas = [
        (("actions/checkout", "v7.0.2"), A),
        (("cachix/cachix-action", "v17"), B),
        (("cachix/install-nix-action", "v31.12.0"), C),
        (("example/action", "v1.0.0"), D),
    ];
    Fake {
        latest: latest.into_iter().collect(),
        shas: shas.into_iter().collect(),
    }
}

fn full_rows() -> String {
    format!(
        "action|version|sha\nactions/checkout|v7.0.2|{A}\n\
         cachix/cachix-action|v17|{B}\n\
         cachix/install-nix-action|v31.12.0|{C}\nexample/action|v1.0.0|{D}\n"
    )
}

/// A fresh directory holding `.actlith` = `text`.
fn scratch(name: &str, text: &str) -> PathBuf {
    let dir = std::env::temp_dir()
        .join(format!("actlith-fetch-{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    let _ = fs::create_dir_all(&dir);
    let _ = fs::write(dir.join(".actlith"), text);
    dir
}

fn actlith(root: &Path) -> String {
    fs::read_to_string(root.join(".actlith")).unwrap_or_default()
}

#[test]
fn every_builtin_action_has_a_builtin_pin() {
    let table = PinTable::builtin();
    for action in BUILTIN_ACTIONS {
        assert!(table.uses(action).is_ok(), "{action}");
    }
}

#[test]
fn dry_run_returns_the_rows_and_writes_nothing() {
    let root = scratch("dry", FULL);
    assert_eq!(pin(&root, true, &fake()), Ok(full_rows()));
    assert_eq!(actlith(&root), FULL);
}

#[test]
fn pin_rewrites_only_the_actions_section() {
    let root = scratch("write", FULL);
    assert_eq!(pin(&root, false, &fake()), Ok(full_rows()));
    assert_eq!(actlith(&root), replace(FULL, &full_rows()));
}

#[test]
fn a_file_without_actions_gets_the_builtin_actions() {
    let root = scratch("none", NONE);
    let rows = format!(
        "action|version|sha\nactions/checkout|v7.0.2|{A}\n\
         cachix/cachix-action|v17|{B}\n\
         cachix/install-nix-action|v31.12.0|{C}\n"
    );
    assert_eq!(pin(&root, false, &fake()), Ok(rows.clone()));
    assert_eq!(actlith(&root), format!("{NONE}\n## actions\n{rows}"));
}

#[test]
fn a_missing_file_is_a_read_error() {
    let root = scratch("missing", "");
    let _ = fs::remove_file(root.join(".actlith"));
    let got = pin(&root, true, &fake());
    let named = matches!(
        &got,
        Err(FetchError::Read { path, .. }) if *path == root.join(".actlith")
    );
    assert!(named, "{got:?}");
}

#[test]
fn an_unparsable_file_is_a_registry_error() {
    let root = scratch("unparsable", "nope\n");
    let got = pin(&root, true, &fake());
    assert!(matches!(got, Err(FetchError::Registry(_))), "{got:?}");
}

#[test]
fn a_refused_override_is_a_pin_error() {
    let root = scratch("bad-sha", BAD_SHA);
    let got = pin(&root, false, &fake());
    assert!(matches!(got, Err(FetchError::Pin(_))), "{got:?}");
    assert_eq!(actlith(&root), BAD_SHA);
}

#[test]
fn a_resolver_failure_writes_nothing() {
    let root = scratch("api", FULL);
    let mut resolver = fake();
    resolver.shas.remove(&("example/action", "v1.0.0"));
    let want = FetchError::Api {
        action: "example/action".to_owned(),
        why: "no tag `v1.0.0`".to_owned(),
    };
    assert_eq!(pin(&root, false, &resolver), Err(want));
    assert_eq!(actlith(&root), FULL);
}

#[test]
fn a_bad_sha_answer_is_refused() {
    let root = scratch("answer", FULL);
    let mut resolver = fake();
    resolver.shas.insert(("example/action", "v1.0.0"), "v1.0.0");
    let got = pin(&root, false, &resolver);
    assert!(matches!(got, Err(FetchError::Sha { .. })), "{got:?}");
    assert_eq!(actlith(&root), FULL);
}

#[test]
fn a_bad_latest_tag_is_refused() {
    let root = scratch("tag", FULL);
    let mut resolver = fake();
    resolver.latest.insert("example/action", "../v2");
    let want = FetchError::Tag {
        action: "example/action".to_owned(),
        tag: "../v2".to_owned(),
    };
    assert_eq!(pin(&root, true, &resolver), Err(want));
}

#[test]
fn an_unwritable_file_is_a_write_error() {
    let root = scratch("readonly", FULL);
    let file = root.join(".actlith");
    let mut perms = fs::metadata(&file).map(|m| m.permissions()).ok();
    if let Some(p) = perms.as_mut() {
        p.set_readonly(true);
        let _ = fs::set_permissions(&file, p.clone());
    }
    let got = pin(&root, false, &fake());
    let named =
        matches!(&got, Err(FetchError::Write { path, .. }) if *path == file);
    assert!(named, "{got:?}");
    assert_eq!(pin(&root, true, &fake()), Ok(full_rows()));
}
