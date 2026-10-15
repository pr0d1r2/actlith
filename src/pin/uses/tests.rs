use super::Uses;
use crate::yaml::{Comment, Input, Value, With};

const SHA: &str = "3d3c42e5aac5ba805825da76410c181273ba90b1";

fn checkout() -> Uses {
    Uses {
        action: "actions/checkout".to_owned(),
        sha: SHA.to_owned(),
        version: "v7.0.1".to_owned(),
    }
}

#[test]
fn reference_is_action_at_sha() {
    let want = format!("actions/checkout@{SHA}");
    assert_eq!(checkout().reference(), want);
}

#[test]
fn comment_is_the_version() {
    assert_eq!(checkout().comment().inline(), "# v7.0.1");
}

#[test]
fn step_carries_reference_version_and_inputs() {
    let with = With::default().set("k", Input::new(Value::Bool(true)));
    let got = checkout().step(with.clone());
    assert_eq!(got.reference, checkout().reference());
    assert_eq!(got.version, Some(Comment::new("v7.0.1")));
    assert_eq!(got.with, with);
}
