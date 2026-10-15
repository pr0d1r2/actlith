use super::{Entry, Node, Value, document};
use crate::yaml::{Comment, Expr, Text};

fn text(s: &str) -> Node {
    Node::Value(Value::Text(Text::lit(s)))
}

#[test]
fn document_starts_with_marker_then_comments_and_blank_separated_keys() {
    let entries = [
        Entry::fixed("name", text("ci")),
        Entry::fixed("on", Node::Null),
    ];
    let out = document(&[Comment::new("top")], &entries);
    assert_eq!(out, "---\n# top\nname: ci\n\non:\n");
}

#[test]
fn nested_maps_indent_two_and_empty_map_is_braces() {
    let inner = vec![
        Entry::fixed("n", Node::Value(Value::Int(0))),
        Entry::fixed("b", Node::Value(Value::Bool(false))),
        Entry::fixed("e", Node::Map(Vec::new())),
    ];
    let out = document(&[], &[Entry::fixed("a", Node::Map(inner))]);
    assert_eq!(out, "---\na:\n  n: 0\n  b: false\n  e: {}\n");
}

#[test]
fn sequence_items_carry_comments_and_trailing_comments() {
    let mut first = Entry::fixed("uses", text("x@1"));
    first.comments.push(Comment::new("why"));
    first.trailing = Some(Comment::new("v1"));
    let item = vec![first, Entry::fixed("with", Node::Null)];
    let seq = Node::Seq(vec![item, Vec::new()]);
    let out = document(&[], &[Entry::fixed("steps", seq)]);
    let want = "---\nsteps:\n  # why\n  - uses: x@1  # v1\n    with:\n  - {}\n";
    assert_eq!(out, want);
}

#[test]
fn user_keys_are_quoted_when_needed() {
    let out = document(&[], &[Entry::user("on", text("x"))]);
    assert_eq!(out, "---\n'on': x\n");
}

#[test]
fn short_list_is_flow_long_list_is_block() {
    let short = Node::List(vec![Text::lit("a"), Text::lit("b c")]);
    let out = document(&[], &[Entry::fixed("k", short)]);
    assert_eq!(out, "---\nk: [a, b c]\n");
    let word = "w".repeat(70);
    let long = Node::List(vec![Text::lit(&word), Text::lit(&word)]);
    let out = document(&[], &[Entry::fixed("k", long)]);
    assert_eq!(out, format!("---\nk:\n  - {word}\n  - {word}\n"));
}

#[test]
fn multiline_text_is_literal_block_with_blank_lines_bare() {
    let out = document(&[], &[Entry::fixed("k", text("a\n\nb\n"))]);
    assert_eq!(out, "---\nk: |\n  a\n\n  b\n");
}

#[test]
fn long_expression_folds_long_word_stays_one_line() -> Result<(), &'static str>
{
    let long = format!("a == '{}' && b", "x".repeat(100));
    let e = Text::expr(Expr::new(&long).ok_or("expr")?);
    let out = document(&[], &[Entry::fixed("k", Node::Value(Value::Text(e)))]);
    let want = format!(
        "---\nk: >-\n  ${{{{ a == '{}' &&\n  b }}}}\n",
        "x".repeat(100)
    );
    assert_eq!(out, want);
    let word = "w".repeat(130);
    let out = document(&[], &[Entry::fixed("k", text(&word))]);
    assert_eq!(out, format!("---\nk: {word}\n"));
    Ok(())
}

#[test]
fn trailing_comment_counts_toward_width() {
    let value = "v ".repeat(55) + "v";
    let mut entry = Entry::fixed("k", text(&value));
    entry.trailing = Some(Comment::new("note"));
    let out = document(&[], &[entry]);
    assert!(out.starts_with("---\nk: >-  # note\n"));
}
