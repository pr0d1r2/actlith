//! `target`: rendered paths stay under the root (gen C).

use std::io::ErrorKind;
use std::path::Path;

use super::target;

/// Paths render never emits: each one could leave or blur the root.
const REFUSED: [&str; 9] = [
    "",
    "/etc/passwd",
    "..",
    "../x",
    "a/../../x",
    "./a",
    "a/./b",
    "a//b",
    "a\\..\\b",
];

#[test]
fn joins_a_nested_relative_path_under_the_root() {
    let root = Path::new("root");
    let got = target(root, ".github/workflows/ci.yml").ok();
    assert_eq!(got, Some(root.join(".github/workflows/ci.yml")));
}

#[test]
fn refuses_every_path_that_is_not_plain_and_relative() {
    for rel in REFUSED {
        let err = target(Path::new("root"), rel).err();
        let kind = err.as_ref().map(std::io::Error::kind);
        assert_eq!(kind, Some(ErrorKind::InvalidInput), "{rel}");
        let text = err.map(|e| e.to_string()).unwrap_or_default();
        assert!(text.contains(&format!("`{rel}`")), "{rel}: {text}");
    }
}
