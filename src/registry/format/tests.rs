use super::check;

#[test]
fn format_1_is_read() {
    assert_eq!(check(Some((1, "format 1"))), Ok(()));
    assert_eq!(check(Some((3, "format  1 "))), Ok(()));
}

#[test]
fn no_format_line_is_refused() {
    for first in [None, Some((2, "## workflows")), Some((1, "formats 1"))] {
        let e = check(first).err().unwrap_or_default();
        assert_eq!(e.what, "no `format` line", "{first:?}");
        assert_eq!(e.fix, "start the file with `format 1`");
    }
    let e = check(Some((2, "## workflows"))).err().unwrap_or_default();
    assert_eq!(e.line, 2);
    assert_eq!(check(None).err().unwrap_or_default().line, 1);
}

#[test]
fn a_newer_format_names_both_versions() {
    let e = check(Some((1, "format 2"))).err().unwrap_or_default();
    assert_eq!(e.line, 1);
    assert_eq!(
        e.what,
        "`.actlith` is format 2; this actlith reads format 1"
    );
    assert_eq!(e.fix, "upgrade actlith to one that reads format 2");
}

#[test]
fn a_malformed_version_is_refused() {
    for bad in ["format x", "format 0", "format", "format -1"] {
        let e = check(Some((1, bad))).err().unwrap_or_default();
        assert!(e.what.starts_with("unknown format"), "{bad}: {}", e.what);
        assert_eq!(e.fix, "write `format 1`");
    }
}
