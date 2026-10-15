use crate::bots::{BotError, File, bot_files};
use crate::registry::{BotRow, parse};

const ALL: &str = include_str!("../../../tests/fixtures/bots/all.actlith");

const GOLDEN: [(&str, &str); 3] = [
    (
        ".coderabbit.yaml",
        include_str!("../../../tests/fixtures/bots/coderabbit.yaml"),
    ),
    (
        "greptile.json",
        include_str!("../../../tests/fixtures/bots/greptile.json"),
    ),
    (
        "socket.yml",
        include_str!("../../../tests/fixtures/bots/socket.yml"),
    ),
];

fn rows(text: &str) -> Vec<BotRow> {
    parse(text).map(|r| r.bots).unwrap_or_default()
}

fn refused(text: &str) -> BotError {
    bot_files(&rows(text)).err().unwrap_or_default()
}

#[test]
fn every_v1_bot_renders_its_golden_file_in_row_order() {
    let want: Vec<File> = GOLDEN
        .iter()
        .map(|(path, bytes)| File {
            path: (*path).to_owned(),
            bytes: (*bytes).to_owned(),
        })
        .collect();
    assert_eq!(rows(ALL).len(), 3);
    assert_eq!(bot_files(&rows(ALL)), Ok(want));
}

#[test]
fn no_rows_no_files() {
    assert_eq!(bot_files(&[]), Ok(Vec::new()));
}

#[test]
fn the_same_rows_give_the_same_bytes() {
    assert_eq!(bot_files(&rows(ALL)), bot_files(&rows(ALL)));
}

#[test]
fn an_unknown_bot_names_its_line_and_the_v1_bots() {
    let err = refused("format 1\n## bots\nbot|args\nsocket|-\nbugbot|-\n");
    assert_eq!(err.line, 5);
    assert_eq!(err.what, "unknown bot `bugbot`");
    assert_eq!(err.fix, "use one of `coderabbit`, `greptile`, `socket`");
}

#[test]
fn a_bad_arg_names_its_line() {
    let err = refused("format 1\n## bots\nbot|args\n\nsocket|deep=1\n");
    assert_eq!(err.line, 5);
    assert_eq!(err.what, "bot `socket` has no arg `deep`");
}
