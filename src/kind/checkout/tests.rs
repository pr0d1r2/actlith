//! Checkout drops credentials unless the job pushes (T17, V3).

use crate::kind::{Checkout, KindError, checkout};
use crate::pin::PinTable;
use crate::yaml::{Action, Comment, Input, Step, Value, With};

fn with(step: &Step) -> With {
    match &step.action {
        Action::Uses(u) => u.with.clone(),
        Action::Run(_) => With::default(),
    }
}

fn keep(pushes: bool) -> With {
    With::default().set("persist-credentials", Input::new(Value::Bool(pushes)))
}

#[test]
fn a_job_that_does_not_push_drops_credentials() -> Result<(), KindError> {
    let step = checkout(&PinTable::builtin(), &Checkout::default())?;
    assert_eq!(with(&step), keep(false));
    Ok(())
}

#[test]
fn a_job_that_pushes_keeps_credentials() -> Result<(), KindError> {
    let how = Checkout {
        pushes: true,
        ..Checkout::default()
    };
    assert_eq!(with(&checkout(&PinTable::builtin(), &how)?), keep(true));
    Ok(())
}

#[test]
fn fetch_depth_is_set_with_its_reason() -> Result<(), KindError> {
    let why = Comment::new("tags");
    let how = Checkout {
        pushes: false,
        fetch_depth: Some(0),
        why_depth: Some(why.clone()),
    };
    let depth = Input {
        comments: vec![why],
        value: Value::Int(0),
    };
    let want = keep(false).set("fetch-depth", depth);
    assert_eq!(with(&checkout(&PinTable::builtin(), &how)?), want);
    Ok(())
}

#[test]
fn checkout_without_a_pin_is_refused() {
    let none = PinTable::default();
    assert!(checkout(&none, &Checkout::default()).is_err());
}
