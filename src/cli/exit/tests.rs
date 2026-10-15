//! Exit codes and `Outcome` (V34).

use std::io;
use std::path::Path;

use super::{FINDING, OK, Outcome, USAGE, lines};

#[test]
fn the_codes_are_zero_one_two() {
    assert_eq!((OK, FINDING, USAGE), (0, 1, 2));
}

#[test]
fn ok_prints_on_stdout_and_exits_zero() {
    let ok = Outcome::ok("row\n");
    assert_eq!((ok.code, &*ok.out, &*ok.err), (0, "row\n", ""));
}

#[test]
fn fail_ends_stderr_in_exactly_one_newline() {
    assert_eq!(Outcome::fail(USAGE, "bad").err, "bad\n");
    assert_eq!(Outcome::fail(FINDING, "bad\n").err, "bad\n");
    assert_eq!(Outcome::fail(FINDING, "bad").code, 1);
}

#[test]
fn io_names_the_path_the_verb_and_a_fix() {
    let error = io::Error::other("denied");
    let failed = Outcome::io(Path::new("a.yml"), "read", &error);
    assert_eq!(failed.code, 2);
    assert!(failed.err.starts_with("a.yml: cannot read: denied\n"));
    assert!(failed.err.contains("fix: "));
}

#[test]
fn and_joins_both_streams_and_keeps_the_larger_code() {
    let both = Outcome::ok("a\n").and(&Outcome::fail(FINDING, "b"));
    assert_eq!((both.code, &*both.out, &*both.err), (1, "a\n", "b\n"));
    let back = Outcome::fail(USAGE, "c").and(&Outcome::ok("d\n"));
    assert_eq!(back.code, 2);
}

#[test]
fn lines_ends_every_item_in_a_newline() {
    assert_eq!(lines(["a", "b"]), "a\nb\n");
    assert_eq!(lines(Vec::<String>::new()), "");
}
