//! Every `pin` refusal names what failed and a fix (V16, T29).

use std::path::PathBuf;

use crate::fetch::FetchError;
use crate::pin::PinError;
use crate::registry::RegistryError;

fn shown(error: &FetchError) -> String {
    error.to_string()
}

#[test]
fn io_errors_name_the_file() {
    let path = PathBuf::from("/r/.actlith");
    let why = "denied".to_owned();
    let read = FetchError::Read {
        path: path.clone(),
        why: why.clone(),
    };
    let write = FetchError::Write { path, why };
    for (text, verb) in [(shown(&read), "read"), (shown(&write), "write")] {
        assert!(
            text.contains(&format!("cannot {verb} /r/.actlith")),
            "{text}"
        );
        assert!(text.contains("denied") && text.contains("fix:"), "{text}");
    }
}

#[test]
fn registry_and_pin_errors_show_through() {
    let reg = RegistryError::new(3, "bad row", "fix it");
    let pin = PinError::Missing {
        action: "a/b".to_owned(),
    };
    assert_eq!(shown(&FetchError::Registry(reg.clone())), reg.to_string());
    assert_eq!(shown(&FetchError::Pin(pin.clone())), pin.to_string());
}

fn s(text: &str) -> String {
    text.to_owned()
}

/// The text names `a/b`, `name` and a fix.
fn names(error: &FetchError, name: &str) {
    let text = shown(error);
    assert!(text.contains("`a/b") && text.contains(name), "{text}");
    assert!(text.contains("\n  fix: "), "{text}");
}

#[test]
fn a_bad_action_is_named() {
    names(&FetchError::Action { action: s("a/b") }, "owner/repo");
}

#[test]
fn a_bad_tag_is_named() {
    let tag = s("v 1");
    names(
        &FetchError::Tag {
            action: s("a/b"),
            tag,
        },
        "`v 1`",
    );
}

#[test]
fn a_bad_answer_is_named() {
    let (tag, got) = (s("v1"), s("null"));
    names(
        &FetchError::Sha {
            action: s("a/b"),
            tag,
            got,
        },
        "`null`",
    );
}

#[test]
fn an_api_failure_is_named() {
    let why = s("HTTP 401");
    names(
        &FetchError::Api {
            action: s("a/b"),
            why,
        },
        "HTTP 401",
    );
}

#[test]
fn it_is_an_error() {
    let error: &dyn std::error::Error = &FetchError::Action { action: s("x") };
    assert!(error.source().is_none());
}
