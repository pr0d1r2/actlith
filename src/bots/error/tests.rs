use crate::bots::BotError;

#[test]
fn new_keeps_line_what_and_fix() {
    let err = BotError::new(7, "unknown bot `x`", "use socket");
    assert_eq!(err.line, 7);
    assert_eq!(err.what, "unknown bot `x`");
    assert_eq!(err.fix, "use socket");
}

#[test]
fn display_names_the_line_and_the_fix() {
    let err = BotError::new(7, "unknown bot `x`", "use socket");
    assert_eq!(
        err.to_string(),
        ".actlith:7: unknown bot `x`\n  fix: use socket"
    );
}
