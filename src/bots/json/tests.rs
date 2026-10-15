use super::{array, string};

#[test]
fn plain_text_is_quoted() {
    assert_eq!(string("chore: release"), "\"chore: release\"");
}

#[test]
fn quotes_and_backslashes_are_escaped() {
    assert_eq!(string(r#"a"b\c"#), r#""a\"b\\c""#);
}

#[test]
fn newlines_tabs_and_returns_are_escaped() {
    assert_eq!(string("a\nb\tc\rd"), r#""a\nb\tc\rd""#);
}

#[test]
fn other_control_characters_are_unicode_escapes() {
    assert_eq!(string("\u{1}\u{7f}"), r#""\u0001\u007f""#);
}

#[test]
fn an_array_is_one_line() {
    assert_eq!(array(&["a", "b c"]), r#"["a", "b c"]"#);
}

#[test]
fn an_empty_array_is_brackets() {
    assert_eq!(array(&[]), "[]");
}
