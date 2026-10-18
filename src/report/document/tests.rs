//! The JSON document (V33): `schema_version` first, stable field names,
//! `null` for no cache. The full document is the golden in `output`.

use super::{SCHEMA_VERSION, document};
use crate::pin::Uses;
use crate::report::json::write;
use crate::report::model::{Hosted, Need, Report, Workflow};

fn uses() -> Uses {
    let (action, sha, version) = ("a/b".into(), "c".into(), "v1".into());
    Uses {
        action,
        sha,
        version,
    }
}

fn workflow() -> Workflow {
    let (system, runner) = ("x86_64-linux".to_owned(), "ubuntu-latest");
    let (name, purpose) = ("S".to_owned(), "p");
    Workflow {
        id: "ci".to_owned(),
        kind: "gate",
        systems: vec![Hosted { system, runner }],
        cache: None,
        actions: vec![uses()],
        secrets: vec![Need { name, purpose }],
        xcode: None,
    }
}

#[test]
fn the_schema_version_is_one() {
    assert_eq!(SCHEMA_VERSION, 1);
}

#[test]
fn an_empty_report_is_its_version_and_empty_lists() {
    let want = "{\n  \"schema_version\": 1,\n  \"workflows\": [],\n  \
                \"hand_written\": [],\n  \"bots\": []\n}\n";
    assert_eq!(write(&document(&Report::default())), want);
}

#[test]
fn a_workflow_carries_every_field_and_null_for_no_cache() {
    let report = Report {
        workflows: vec![workflow()],
        ..Report::default()
    };
    let got = write(&document(&report));
    let want = "    {\n      \"id\": \"ci\",\n      \"kind\": \"gate\",\n      \
                \"systems\": [\n        {\n          \"system\": \
                \"x86_64-linux\",\n          \"runner\": \"ubuntu-latest\"\n\
                \x20       }\n      ],\n      \"cache\": null,\n      \
                \"actions\": [\n        {\n          \"action\": \"a/b\",\n\
                \x20         \"version\": \"v1\",\n          \"sha\": \"c\"\n\
                \x20       }\n      ],\n      \"secrets\": [\n        {\n\
                \x20         \"name\": \"S\",\n          \"purpose\": \"p\"\n\
                \x20       }\n      ]\n    }\n";
    assert!(got.contains(want), "{got}");
}

/// [`workflow`] on `aarch64-darwin`, Xcode `xcode` pinned.
fn darwin(xcode: Option<&str>) -> String {
    let (system, runner) = ("aarch64-darwin".to_owned(), "macos-latest");
    let workflows = vec![Workflow {
        systems: vec![Hosted { system, runner }],
        xcode: xcode.map(str::to_owned),
        ..workflow()
    }];
    let report = Report {
        workflows,
        ..Report::default()
    };
    write(&document(&report))
}

#[test]
fn a_pinned_xcode_is_a_field_beside_the_runner() {
    let got = darwin(Some("26.4"));
    let want = "          \"runner\": \"macos-latest\",\n          \
                \"xcode\": \"26.4\"\n        }";
    assert!(got.contains(want), "{got}");
    let bare = darwin(None);
    assert!(!bare.contains("xcode"), "{bare}");
}
