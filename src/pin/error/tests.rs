use super::PinError;

#[test]
fn missing_names_the_action_and_the_fix() {
    let e = PinError::Missing {
        action: "o/r".to_owned(),
    };
    let want = "action `o/r` has no pin\n  fix: add a `## actions` row \
        `o/r|<version>|<sha>` (`actlith pin` resolves it)";
    assert_eq!(e.to_string(), want);
}

#[test]
fn not_sha_names_line_action_and_sha() {
    let e = PinError::NotSha {
        line: 7,
        action: "o/r".to_owned(),
        sha: "v1".to_owned(),
    };
    let want = ".actlith:7: pin `v1` of `o/r` is not a full commit SHA\n  \
        fix: write the 40-hex commit SHA (`actlith pin` resolves it)";
    assert_eq!(e.to_string(), want);
}
