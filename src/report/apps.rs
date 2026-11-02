//! The GitHub App each v1 bot assumes (`src/bots`): actlith writes only
//! the config file; installing the app is the user's action (`.:V41`).

/// What the user does about each app.
pub(crate) const NOTE: &str = "install the app; free tier per docs/SERVICES.md";

/// Each v1 bot and the app it assumes, as `docs/SERVICES.md` names it.
/// The app `bot` assumes, if it is a v1 bot.
pub(crate) fn app(bot: &str) -> Option<&'static str> {
    crate::bots::app_name(bot)
}

#[cfg(test)]
mod tests;
