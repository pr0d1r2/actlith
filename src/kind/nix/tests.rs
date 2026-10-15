//! install-nix trusts the row's cache (T17).

use crate::kind::{Cache, KindError, install_nix};
use crate::pin::PinTable;
use crate::yaml::{Action, Input, Step, Text, Value, With};

fn with(step: &Step) -> With {
    match &step.action {
        Action::Uses(u) => u.with.clone(),
        Action::Run(_) => With::default(),
    }
}

#[test]
fn with_a_cache_the_substituter_is_trusted() -> Result<(), KindError> {
    let cache = Cache::parse(1, "pr0d1r2")?;
    let step = install_nix(&PinTable::builtin(), Some(&cache))?;
    let config = Value::Text(Text::lit(&cache.nix_config()));
    let want = With::default().set("extra_nix_config", Input::new(config));
    assert_eq!(with(&step), want);
    Ok(())
}

#[test]
fn without_a_cache_nothing_extra_is_trusted() -> Result<(), KindError> {
    let step = install_nix(&PinTable::builtin(), None)?;
    assert_eq!(with(&step), With::default());
    Ok(())
}
