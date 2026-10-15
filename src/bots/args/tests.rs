use super::{known, required};
use crate::registry::{Arg, BotRow};

fn row(args: &[(&str, &str)]) -> BotRow {
    BotRow {
        line: 4,
        bot: "coderabbit".to_owned(),
        args: args
            .iter()
            .map(|(k, v)| Arg {
                key: (*k).to_owned(),
                value: (*v).to_owned(),
            })
            .collect(),
    }
}

#[test]
fn known_keys_pass() {
    assert_eq!(known(&row(&[("central", "a/b")]), &["central"]), Ok(()));
}

#[test]
fn no_args_pass_any_schema() {
    assert_eq!(known(&row(&[]), &[]), Ok(()));
}

#[test]
fn an_unknown_key_names_the_line_and_the_known_ones() {
    let err = known(&row(&[("central", "a/b"), ("ref", "x")]), &["central"])
        .err()
        .unwrap_or_default();
    assert_eq!(err.line, 4);
    assert_eq!(err.what, "bot `coderabbit` has no arg `ref`");
    assert_eq!(err.fix, "its args: `central`");
}

#[test]
fn a_bot_without_args_says_so() {
    let err = known(&row(&[("x", "1")]), &[]).err().unwrap_or_default();
    assert_eq!(err.what, "bot `coderabbit` has no arg `x`");
    assert_eq!(err.fix, "it takes no args; write `-`");
}

#[test]
fn required_returns_the_value() {
    let r = row(&[("central", "a/b")]);
    assert_eq!(required(&r, "central", "<owner>/<repo>"), Ok("a/b"));
}

#[test]
fn a_missing_required_key_names_the_line_and_the_hint() {
    let err = required(&row(&[]), "central", "<owner>/<repo>")
        .err()
        .unwrap_or_default();
    assert_eq!(err.line, 4);
    assert_eq!(err.what, "bot `coderabbit` needs arg `central`");
    assert_eq!(err.fix, "add `central=<owner>/<repo>`");
}
