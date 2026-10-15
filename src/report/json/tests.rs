//! The JSON writer: scalars, escapes, empty and nested containers, two
//! spaces per level, one trailing newline.

use super::{Json, write};

fn s(text: &str) -> Json {
    Json::String(text.to_owned())
}

#[test]
fn scalars_print_bare() {
    assert_eq!(write(&Json::Null), "null\n");
    assert_eq!(write(&Json::Number(1)), "1\n");
    assert_eq!(write(&s("a")), "\"a\"\n");
}

#[test]
fn a_string_is_escaped() {
    let raw = "q\" b\\ n\n t\t r\r c\u{1}";
    let want = "\"q\\\" b\\\\ n\\n t\\t r\\r c\\u0001\"\n";
    assert_eq!(write(&s(raw)), want);
}

#[test]
fn empty_containers_print_on_one_line() {
    assert_eq!(write(&Json::Array(Vec::new())), "[]\n");
    assert_eq!(write(&Json::Object(Vec::new())), "{}\n");
}

#[test]
fn nested_containers_indent_two_spaces_a_level() {
    let value = Json::Object(vec![
        ("a", Json::Array(vec![Json::Number(1), s("x")])),
        ("b", Json::Object(vec![("c", Json::Null)])),
    ]);
    let want = "{\n  \"a\": [\n    1,\n    \"x\"\n  ],\n  \"b\": {\n    \
                \"c\": null\n  }\n}\n";
    assert_eq!(write(&value), want);
}
