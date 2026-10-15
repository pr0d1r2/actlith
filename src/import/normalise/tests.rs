//! Normalising (V17): comments, whitespace, blank lines and `---` go,
//! a `>-` block joins its key, block content stays as written.

use crate::import::normalise::{Line, column, normalise};

fn texts(text: &str) -> Vec<String> {
    normalise(text).into_iter().map(|l| l.text).collect()
}

#[test]
fn comments_blank_lines_and_the_marker_go() {
    let text = "---\n# head\nname: ci   \n\n\n  # inner\non:\n  push:\n";
    assert_eq!(texts(text), ["name: ci", "on:", "  push:"]);
}

#[test]
fn kept_lines_keep_their_line_numbers() {
    let numbers: Vec<usize> = normalise("# a\n\nname: ci\n  b: c\n")
        .iter()
        .map(|l| l.number)
        .collect();
    assert_eq!(numbers, [3, 4]);
}

#[test]
fn a_pin_comment_goes_whatever_its_spacing() {
    let sha = "actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1";
    let two = texts(&format!("- uses: {sha}  # v7.0.1\n"));
    let one = texts(&format!("- uses: {sha} # v7\n"));
    assert_eq!(two, [format!("- uses: {sha}")]);
    assert_eq!(one, two);
}

#[test]
fn a_hash_inside_quotes_or_a_word_stays() {
    let text = "a: 'x # y'\nb: \"x # y\"\nc: x#y\nd: \"q\\\" # r\"\n";
    assert_eq!(
        texts(text),
        ["a: 'x # y'", "b: \"x # y\"", "c: x#y", "d: \"q\\\" # r\""]
    );
}

#[test]
fn a_comment_after_a_closed_quote_goes() {
    assert_eq!(texts("a: 'x' # y\n"), ["a: 'x'"]);
}

#[test]
fn literal_block_content_stays_as_written() {
    let text = "      - run: |\n          # not a comment\n\n          b  \n        env:\n";
    let lines = normalise(text);
    let kept: Vec<(&str, bool)> =
        lines.iter().map(|l| (l.text.as_str(), l.block)).collect();
    assert_eq!(
        kept,
        [
            ("      - run: |", false),
            ("          # not a comment", true),
            ("          b", true),
            ("        env:", false),
        ]
    );
}

#[test]
fn a_folded_block_joins_its_key_line() {
    let folded = "    skipPush: >-\n      ${{ a &&\n      b }}\n    x: y\n";
    let flat = "    skipPush: ${{ a && b }}\n    x: y\n";
    assert_eq!(texts(folded), texts(flat));
    let line = normalise(folded).into_iter().next();
    assert_eq!(
        line,
        Some(Line {
            number: 1,
            text: "    skipPush: ${{ a && b }}".to_owned(),
            block: false,
        })
    );
}

#[test]
fn other_block_headers_stay_whole() {
    for indicator in ["|", "|-", "|+", ">", ">+"] {
        let text = format!("k: {indicator}\n  v\n");
        assert_eq!(texts(&text), [format!("k: {indicator}"), "  v".into()]);
    }
}

#[test]
fn a_bar_that_is_not_a_header_opens_nothing() {
    assert_eq!(texts("run: a |\n  b: c\n"), ["run: a |", "  b: c"]);
    assert_eq!(texts("run\nk: v\n"), ["run", "k: v"]);
}

#[test]
fn the_key_column_is_past_a_sequence_marker() {
    assert_eq!(column("  - k: v"), 4);
    assert_eq!(column("  k: v"), 2);
    assert_eq!(column(""), 0);
}
