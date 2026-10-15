use crate::yaml::{Comment, Expr, Part, Text};

#[test]
fn expr_renders_inside_delimiters_trimmed() {
    let e = Expr::new("  github.ref ");
    assert_eq!(e.as_ref().map(Expr::inner), Some("github.ref"));
    assert_eq!(e.map(|e| e.render()).as_deref(), Some("${{ github.ref }}"));
}

#[test]
fn expr_that_would_end_or_nest_early_is_refused() {
    assert_eq!(Expr::new("a }} ${{ b"), None);
    assert_eq!(Expr::new("a }}"), None);
    assert_eq!(Expr::new("${{ a"), None);
}

#[test]
fn expr_empty_or_with_control_char_is_refused() {
    assert_eq!(Expr::new(""), None);
    assert_eq!(Expr::new("   "), None);
    assert_eq!(Expr::new("a\nb"), None);
}

#[test]
fn text_joins_literal_and_expression_parts() -> Result<(), &'static str> {
    let e = Expr::new("github.ref").ok_or("expr")?;
    let t = Text::lit("ci-").then_expr(e.clone()).then_lit("-x");
    assert_eq!(t.render(), "ci-${{ github.ref }}-x");
    assert_eq!(t.parts().get(1), Some(&Part::Expr(e.clone())));
    assert_eq!(Text::expr(e).render(), "${{ github.ref }}");
    Ok(())
}

#[test]
fn literal_never_opens_an_expression() {
    let t = Text::from("echo ${{ secrets.X }}");
    assert_eq!(t.render(), "echo ${{ '${{' }} secrets.X }}");
}

#[test]
fn default_text_is_empty() {
    assert_eq!(Text::default().render(), "");
}

#[test]
fn comment_lines_are_hashed() {
    let c = Comment::new("one\n\ntwo");
    assert_eq!(c.lines(), ["# one", "#", "# two"]);
    assert_eq!(c.inline(), "# one  two");
}

#[test]
fn comment_control_chars_become_spaces_trailing_dropped() {
    let c = Comment::new("a\rb\tc  ");
    assert_eq!(c.lines(), ["# a b c"]);
}

#[test]
fn comment_empty_inline_is_bare_hash() {
    assert_eq!(Comment::new("").inline(), "#");
}
