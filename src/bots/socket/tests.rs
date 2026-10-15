//! Socket's docs (docs.socket.dev/docs/socket-yml), quoted: "All fields
//! listed here are optional, except for the version field." The version
//! is `2`.

use super::{PATH, file};
use crate::registry::{Arg, BotRow};

const GOLDEN: &str = include_str!("../../../tests/fixtures/bots/socket.yml");

fn row(args: &[(&str, &str)]) -> BotRow {
    BotRow {
        line: 5,
        bot: "socket".to_owned(),
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
fn renders_the_golden_file() {
    let got = file(&row(&[]));
    assert_eq!(got.as_ref().map(|f| f.path.as_str()), Ok(PATH));
    assert_eq!(got.map(|f| f.bytes), Ok(GOLDEN.to_owned()));
}

#[test]
fn the_file_starts_with_the_generated_header() {
    let got = file(&row(&[])).map(|f| f.bytes).unwrap_or_default();
    assert!(got.starts_with(crate::yaml::HEADER));
}

#[test]
fn any_arg_is_refused() {
    let err = file(&row(&[("x", "1")])).err().unwrap_or_default();
    assert_eq!(err.line, 5);
    assert_eq!(err.what, "bot `socket` has no arg `x`");
}
