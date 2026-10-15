//! Greptile's config reference (code-review/greptile-config-reference),
//! Filters table, quoted:
//!
//! - `disabledLabels` `string[]`: "Skip review for PRs with these labels."
//! - `ignoreKeywords` `string`: "Newline-separated keywords. Skip PRs with
//!   these in title/description"

use super::{PATH, file};
use crate::registry::{Arg, BotRow};

const GOLDEN: &str = include_str!("../../../tests/fixtures/bots/greptile.json");

fn row(args: &[(&str, &str)]) -> BotRow {
    BotRow {
        line: 3,
        bot: "greptile".to_owned(),
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
fn json_carries_no_header_comment() {
    let got = file(&row(&[])).map(|f| f.bytes).unwrap_or_default();
    assert!(got.starts_with('{'));
}

#[test]
fn any_arg_is_refused() {
    let err = file(&row(&[("labels", "x")])).err().unwrap_or_default();
    assert_eq!(err.line, 3);
    assert_eq!(err.what, "bot `greptile` has no arg `labels`");
}
