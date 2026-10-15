//! The `gh api` resolver against a stand-in script: no network (src V4,
//! V16, T29).

use std::ffi::{OsStr, OsString};

use crate::fetch::FetchError;
use crate::fetch::Gh;
use crate::fetch::Resolver;
use crate::fetch::gh::{GIT_REPO_VARS, repo};

const SHA: &str = "0123456789abcdef0123456789abcdef01234567";

/// `sh tests/fixtures/fetch/fake-gh.sh` in place of `gh`.
fn fake() -> Gh {
    let script = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/fetch/fake-gh.sh"
    );
    Gh::with("sh", vec![OsString::from(script)])
}

fn args(gh: &Gh) -> Vec<OsString> {
    let command = gh.command("repos/a/b/commits/v1", ".sha");
    command.get_args().map(OsStr::to_owned).collect()
}

#[test]
fn the_command_is_gh_api_with_a_jq_filter() {
    let want = ["api", "repos/a/b/commits/v1", "--jq", ".sha"];
    let command = Gh::new().command("repos/a/b/commits/v1", ".sha");
    assert_eq!(command.get_program(), "gh");
    assert_eq!(args(&Gh::new()), want.map(OsString::from));
}

#[test]
fn the_prefix_goes_before_api() {
    let gh = Gh::with("sh", vec![OsString::from("x.sh")]);
    let got = args(&gh);
    assert_eq!(
        got.first().map(OsString::as_os_str),
        Some(OsStr::new("x.sh"))
    );
    assert_eq!(got.get(1).map(OsString::as_os_str), Some(OsStr::new("api")));
}

#[test]
fn the_command_drops_the_git_repository_variables() {
    let command = Gh::new().command("repos/a/b/commits/v1", ".sha");
    let removed: Vec<&OsStr> = command
        .get_envs()
        .filter(|(_, value)| value.is_none())
        .map(|(key, _)| key)
        .collect();
    for var in GIT_REPO_VARS {
        assert!(removed.contains(&OsStr::new(var)), "{var}");
    }
}

#[test]
fn repo_is_owner_and_name_of_the_action() {
    assert_eq!(repo("actions/checkout"), Ok("actions/checkout".to_owned()));
    assert_eq!(
        repo("github/codeql-action/init"),
        Ok("github/codeql-action".to_owned())
    );
}

#[test]
fn a_malformed_action_is_refused() {
    for action in ["", "checkout", "/x", "a/", "a/b c", "a/b?x", "../b"] {
        let want = FetchError::Action {
            action: action.to_owned(),
        };
        assert_eq!(repo(action), Err(want), "{action}");
    }
}

#[test]
fn latest_is_the_release_tag() {
    let got = fake().latest("actions/checkout");
    assert_eq!(got, Ok(Some("v7.0.2".to_owned())));
}

#[test]
fn no_release_is_none() {
    assert_eq!(fake().latest("example/norelease"), Ok(None));
}

#[test]
fn an_api_failure_names_the_action_and_the_reason() {
    let want = FetchError::Api {
        action: "example/denied".to_owned(),
        why: "gh: Bad credentials (HTTP 401)".to_owned(),
    };
    assert_eq!(fake().latest("example/denied"), Err(want));
}

#[test]
fn sha_is_the_commit_of_the_tag() {
    let got = fake().sha("actions/checkout", "v7.0.2");
    assert_eq!(got, Ok(SHA.to_owned()));
}

#[test]
fn a_missing_tag_is_an_api_error() {
    let got = fake().sha("example/norelease", "v9");
    let want = FetchError::Api {
        action: "example/norelease".to_owned(),
        why: "no tag `v9`".to_owned(),
    };
    assert_eq!(got, Err(want));
}

#[test]
fn a_malformed_action_never_reaches_the_api() {
    let gh = Gh::with("/nonexistent/gh", Vec::new());
    let want = FetchError::Action {
        action: "nope".to_owned(),
    };
    assert_eq!(gh.latest("nope"), Err(want.clone()));
    assert_eq!(gh.sha("nope", "v1"), Err(want));
}

#[test]
fn a_missing_program_is_an_api_error() {
    let gh = Gh::with("/nonexistent/gh", Vec::new());
    let got = gh.latest("a/b");
    let named = matches!(
        &got,
        Err(FetchError::Api { action, why })
            if action == "a/b" && why.starts_with("cannot run `/nonexistent/gh`")
    );
    assert!(named, "{got:?}");
}
