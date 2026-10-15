//! Every refusal prints the error it carries; a duplicate names its path.

use crate::bots::BotError;
use crate::kind::KindError;
use crate::pin::PinError;
use crate::registry::RegistryError;
use crate::render::RenderError;

#[test]
fn a_row_error_prints_its_line_and_fix() {
    let error = RenderError::Row(RegistryError::new(4, "bad", "good"));
    assert_eq!(error.to_string(), ".actlith:4: bad\n  fix: good");
}

#[test]
fn a_kind_error_prints_as_itself() {
    let kind = KindError::row(5, "what", "fix");
    assert_eq!(
        RenderError::from(kind.clone()).to_string(),
        kind.to_string()
    );
}

#[test]
fn a_pin_error_prints_as_itself() {
    let pin = PinError::Missing {
        action: "a/b".to_owned(),
    };
    assert_eq!(RenderError::from(pin.clone()).to_string(), pin.to_string());
}

#[test]
fn a_bot_error_prints_as_itself() {
    let bot = BotError::new(6, "what", "fix");
    assert_eq!(RenderError::from(bot.clone()).to_string(), bot.to_string());
}

#[test]
fn a_duplicate_names_its_path() {
    let error = RenderError::Duplicate("a.yml".to_owned());
    assert_eq!(
        error.to_string(),
        "two files would be written at `a.yml`\n  fix: give each \
         `## workflows` row its own id"
    );
}
