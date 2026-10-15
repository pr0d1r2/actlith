//! A step that uses a pinned action (`src/pin` V2): the only way a kind
//! writes `uses:`.

use super::error::KindError;
use crate::pin::PinTable;
use crate::yaml::{Action, Env, Step, With};

/// A step doing `action`, with no name, id, condition or env.
pub(crate) fn step(action: Action) -> Step {
    Step {
        comments: Vec::new(),
        name: None,
        id: None,
        condition: None,
        action,
        env: Env::new(),
    }
}

/// A step using `action` at its pin from `pins`, with `with` inputs.
pub(crate) fn uses(
    pins: &PinTable,
    action: &str,
    with: With,
) -> Result<Step, KindError> {
    let pin = pins.uses(action)?;
    Ok(step(Action::Uses(pin.step(with))))
}

#[cfg(test)]
mod tests;
