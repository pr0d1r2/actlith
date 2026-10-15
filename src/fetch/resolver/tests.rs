//! Tag and SHA checks every resolver answer passes through (V16, T29).

use crate::fetch::FetchError;
use crate::fetch::resolver::{checked_sha, checked_tag};

const SHA: &str = "0123456789abcdef0123456789abcdef01234567";

#[test]
fn release_tags_pass() {
    for tag in ["v7.0.1", "v17", "1.2.3-rc.1+build", "release/v2", "v1_0"] {
        assert_eq!(checked_tag("a/b", tag), Ok(()), "{tag}");
    }
}

#[test]
fn tags_unsafe_in_an_api_path_are_refused() {
    for tag in ["", "-v1", "/v1", "v1/../x", "v1?x", "v1#x", "v 1", "v1%2f"] {
        let want = FetchError::Tag {
            action: "a/b".to_owned(),
            tag: tag.to_owned(),
        };
        assert_eq!(checked_tag("a/b", tag), Err(want), "{tag}");
    }
}

#[test]
fn a_full_sha_passes_trimmed() {
    let got = checked_sha("a/b", "v1", &format!("{SHA}\n"));
    assert_eq!(got, Ok(SHA.to_owned()));
}

#[test]
fn anything_else_is_refused_naming_the_answer() {
    let upper = SHA.to_uppercase();
    let short = SHA.get(..39).unwrap_or_default();
    for got in ["", "v1", short, upper.as_str(), "null"] {
        let want = FetchError::Sha {
            action: "a/b".to_owned(),
            tag: "v1".to_owned(),
            got: got.to_owned(),
        };
        assert_eq!(checked_sha("a/b", "v1", got), Err(want), "{got}");
    }
}
