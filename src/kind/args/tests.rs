//! Arg schema check: unknown, repeated, missing, defaults (T17,
//! `src/registry` V10).

use crate::kind::{ArgSchema, ArgSpec, Args, KindError, TIMEOUT, parse_args};
use crate::registry::{Arg, WorkflowRow};

const SCHEMA: [ArgSpec; 3] = [
    ArgSpec {
        key: "package",
        required: true,
        default: None,
    },
    ArgSpec {
        key: "note",
        required: false,
        default: None,
    },
    TIMEOUT,
];

fn row(args: &[(&str, &str)]) -> WorkflowRow {
    WorkflowRow {
        line: 12,
        id: "build".to_owned(),
        kind: "build".to_owned(),
        systems: Vec::new(),
        cache: None,
        args: args
            .iter()
            .map(|&(k, v)| Arg {
                key: k.to_owned(),
                value: v.to_owned(),
            })
            .collect(),
    }
}

fn refused(schema: &ArgSchema, args: &[(&str, &str)]) -> String {
    parse_args(schema, &row(args))
        .err()
        .map(|e| e.to_string())
        .unwrap_or_default()
}

fn parsed(args: &[(&str, &str)]) -> Result<Args, KindError> {
    parse_args(&SCHEMA, &row(args))
}

#[test]
fn given_values_are_kept_and_defaults_filled() -> Result<(), KindError> {
    let args = parsed(&[("package", "hk")])?;
    assert_eq!(args.line(), 12);
    assert_eq!(args.get("package"), Some("hk"));
    assert_eq!(args.get("timeout"), Some("30"));
    assert_eq!(args.get("note"), None);
    Ok(())
}

#[test]
fn a_given_value_beats_the_default() -> Result<(), KindError> {
    let args = parsed(&[("timeout", "45"), ("package", "hk")])?;
    assert_eq!(args.get("timeout"), Some("45"));
    assert_eq!(args.number("timeout")?, Some(45));
    assert_eq!(args.number("note")?, None);
    Ok(())
}

#[test]
fn an_unknown_key_is_refused_naming_line_and_keys() {
    assert_eq!(
        refused(&SCHEMA, &[("package", "hk"), ("flavor", "x")]),
        ".actlith:12: kind `build` has no arg `flavor`\n  fix: remove \
         `flavor` or use one of: package, note, timeout"
    );
}

#[test]
fn a_kind_without_args_says_so() {
    assert_eq!(
        refused(&[], &[("x", "1")]),
        ".actlith:12: kind `build` has no arg `x`\n  fix: write `-` in args"
    );
}

#[test]
fn a_missing_required_key_is_refused() {
    assert_eq!(
        refused(&SCHEMA, &[]),
        ".actlith:12: kind `build` needs arg `package`\n  fix: add \
         `package=<value>` to args"
    );
}

#[test]
fn a_repeated_key_is_refused() {
    assert_eq!(
        refused(&SCHEMA, &[("package", "a"), ("package", "b")]),
        ".actlith:12: arg `package` is given twice\n  fix: keep one \
         `package=`"
    );
}

#[test]
fn a_non_number_is_refused() -> Result<(), KindError> {
    let args = parsed(&[("package", "hk"), ("timeout", "soon")])?;
    let error = args.number("timeout").err().map(|e| e.to_string());
    assert_eq!(
        error.as_deref(),
        Some(
            ".actlith:12: arg `timeout` is `soon`, not a whole number\n  \
             fix: write `timeout=<number>`"
        )
    );
    Ok(())
}
