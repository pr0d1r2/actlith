//! GitHub's `${{ }}` through pipelith's `Expr` and `Text` (V5).

use std::any::TypeId;

use super::GITHUB;
use crate::core::yaml::{Expr, Text};

#[test]
fn expr_renders_inside_delimiters_trimmed() {
    let e = Expr::new(&GITHUB, "  github.ref ");
    assert_eq!(e.as_ref().map(Expr::inner), Some("github.ref"));
    assert_eq!(e.as_ref().map(Expr::text), Some("${{ github.ref }}"));
}

#[test]
fn expr_that_would_end_or_nest_early_is_refused() {
    assert_eq!(Expr::new(&GITHUB, "a }} ${{ b"), None);
    assert_eq!(Expr::new(&GITHUB, "a }}"), None);
    assert_eq!(Expr::new(&GITHUB, "${{ a"), None);
}

#[test]
fn literal_never_opens_an_expression() {
    let t = Text::from("echo ${{ secrets.X }}");
    assert_eq!(t.render(&GITHUB), "echo ${{ '${{' }} secrets.X }}");
}

#[test]
fn yaml_text_is_pipelith_text() {
    let ours = TypeId::of::<crate::yaml::Text>();
    assert_eq!(ours, TypeId::of::<Text>());
}
