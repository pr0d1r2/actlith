//! The GitHub App each v1 bot assumes (`src/bots`): actlith writes only
//! the config file; installing the app is the user's action (`.:V41`).

/// What the user does about each app.
pub(crate) const NOTE: &str = "";

/// The app `bot` assumes, if it is a v1 bot.
pub(crate) fn app(_bot: &str) -> Option<&'static str> {
    None
}

#[cfg(test)]
mod tests;
