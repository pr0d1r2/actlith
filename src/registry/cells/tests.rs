use super::{args, fixed, header, list, optional, required, split};
use crate::registry::model::Arg;
use crate::registry::table::Table;

fn arg(key: &str, value: &str) -> Arg {
    Arg {
        key: key.into(),
        value: value.into(),
    }
}

#[test]
fn a_row_splits_on_pipes_trimmed() {
    assert_eq!(split(" a | b|c "), ["a", "b", "c"]);
    assert_eq!(split("a"), ["a"]);
    assert_eq!(split("a||"), ["a", "", ""]);
}

#[test]
fn a_dash_is_an_empty_cell() {
    assert_eq!(split("-|a-b| - "), ["", "a-b", ""]);
}

#[test]
fn an_escaped_pipe_is_literal() {
    assert_eq!(split(r"a \| b|c\d"), ["a | b", r"c\d"]);
    assert_eq!(split(r"x\"), [r"x\"]);
}

#[test]
fn the_cell_count_must_match_the_header() {
    let ok = fixed::<2>(4, Table::HandWritten, vec!["p".into(), "r".into()]);
    assert_eq!(ok, Ok(["p".to_owned(), "r".to_owned()]));
    let e = fixed::<2>(4, Table::HandWritten, vec!["p".into()])
        .err()
        .unwrap_or_default();
    assert_eq!(e.line, 4);
    assert_eq!(e.what, "1 cell where `## hand-written` rows have 2");
    assert!(e.fix.contains("path|reason"), "{}", e.fix);
}

#[test]
fn the_header_row_is_the_spec_one() {
    assert_eq!(header(2, Table::Bots, "bot | args"), Ok(()));
    let e = header(2, Table::Bots, "bot|arg").err();
    let e = e.unwrap_or_default();
    assert_eq!(e.line, 2);
    assert_eq!(e.what, "`## bots` header is `bot|arg`");
    assert_eq!(e.fix, "start the section with `bot|args`");
}

#[test]
fn a_required_cell_is_not_empty() {
    assert_eq!(required(1, "kind", "gate".into()), Ok("gate".into()));
    let e = required(5, "kind", String::new()).err();
    let e = e.unwrap_or_default();
    assert_eq!((e.line, e.what.as_str()), (5, "`kind` is empty"));
}

#[test]
fn an_optional_cell_is_none_when_empty() {
    assert_eq!(optional("c"), Some("c".into()));
    assert_eq!(optional(""), None);
}

#[test]
fn a_list_splits_on_commas() {
    let got = list(1, "systems", "x86_64-linux, aarch64-linux");
    assert_eq!(got, Ok(vec!["x86_64-linux".into(), "aarch64-linux".into()]));
    assert_eq!(list(1, "systems", ""), Ok(vec![]));
}

#[test]
fn a_list_has_no_empty_item() {
    let e = list(7, "systems", "a,,b").err();
    let e = e.unwrap_or_default();
    assert_eq!(
        (e.line, e.what.as_str()),
        (7, "`systems` has an empty item")
    );
}

#[test]
fn args_are_key_value_pairs() {
    let got = args(1, "package=actlith, cron=0 6 * * *, x=a=b");
    let want = vec![
        arg("package", "actlith"),
        arg("cron", "0 6 * * *"),
        arg("x", "a=b"),
    ];
    assert_eq!(got, Ok(want));
    assert_eq!(args(1, ""), Ok(vec![]));
}

#[test]
fn an_arg_without_key_or_value_is_refused() {
    for bad in ["package", "=x", "package=", " = "] {
        let e = args(3, bad).err().unwrap_or_default();
        assert_eq!(e.line, 3, "{bad}");
        assert!(e.what.contains("is not `key=value`"), "{}", e.what);
    }
}

#[test]
fn an_arg_key_is_given_once() {
    let e = args(2, "a=1, a=2").err().unwrap_or_default();
    assert_eq!((e.line, e.what.as_str()), (2, "arg `a` given twice"));
}
