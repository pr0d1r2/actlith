//! `target`: rendered paths stay under the root (gen C).

use std::io::ErrorKind;
use std::path::Path;

use super::target;

#[test]
fn joins_a_nested_relative_path_under_the_root() {
    let root = Path::new("root");
    let got = target(root, ".github/workflows/ci.yml").ok();
    assert_eq!(got, Some(root.join(".github/workflows/ci.yml")));
}

#[test]
fn refuses_every_path_that_is_not_plain_and_relative() {
    for rel in ["", "/etc/passwd", "..", "../x", "a/../../x", "./a", "a/./b"] {
        let err = target(Path::new("root"), rel).err();
        assert_eq!(
            err.as_ref().map(std::io::Error::kind),
            Some(ErrorKind::InvalidInput)
        );
        let text = err.map(|e| e.to_string()).unwrap_or_default();
        assert!(text.contains(&format!("`{rel}`")), "{rel}: {text}");
    }
}
