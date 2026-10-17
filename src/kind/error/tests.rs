//! A kind error prints as the row or pin error it carries (T17, T65).

use crate::kind::KindError;
use crate::pin::PinError;

#[test]
fn a_row_error_names_its_line_and_fix() {
    let error = KindError::row(7, "arg `x` is unknown", "remove `x`");
    assert_eq!(
        error.to_string(),
        ".actlith:7: arg `x` is unknown\n  fix: remove `x`"
    );
}

#[test]
fn a_pin_error_prints_as_the_pin_error() {
    let pin = PinError::Missing {
        action: "a/b".to_owned(),
    };
    let error = KindError::from(pin.clone());
    assert_eq!(error, KindError::Pin(pin.clone()));
    assert_eq!(error.to_string(), pin.to_string());
}

#[test]
fn a_pipelith_refusal_is_a_row_error_at_its_line() {
    let neutral = crate::core::kind::KindError::new(9, "bad", "fix it");
    let error = KindError::from(neutral);
    assert_eq!(error, KindError::row(9, "bad", "fix it"));
    assert_eq!(error.to_string(), ".actlith:9: bad\n  fix: fix it");
}
