//! Steps use only pinned actions (T17, `src/pin` V2).

use crate::kind::{KindError, step, uses};
use crate::pin::{PinError, PinTable};
use crate::yaml::{Action, Step, With};

fn reference(step: &Step) -> Option<&str> {
    match &step.action {
        Action::Uses(u) => Some(u.reference.as_str()),
        Action::Run(_) => None,
    }
}

#[test]
fn a_bare_step_has_only_its_action() {
    let bare = step(Action::Run("true".to_owned()));
    assert_eq!(bare.action, Action::Run("true".to_owned()));
    assert!(bare.comments.is_empty() && bare.env.is_empty());
    assert_eq!((bare.name, bare.id, bare.condition), (None, None, None));
}

#[test]
fn a_uses_step_is_pinned_by_sha() -> Result<(), KindError> {
    let pinned =
        uses(&PinTable::builtin(), "actions/checkout", With::default())?;
    assert_eq!(
        reference(&pinned),
        Some("actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1")
    );
    Ok(())
}

#[test]
fn an_action_without_a_pin_is_refused() {
    let refused = uses(&PinTable::builtin(), "a/b", With::default()).err();
    let missing = PinError::Missing {
        action: "a/b".to_owned(),
    };
    assert_eq!(refused, Some(KindError::Pin(missing)));
}
