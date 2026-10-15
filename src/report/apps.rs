//! The GitHub App each v1 bot assumes (`src/bots`): actlith writes only
//! the config file; installing the app is the user's action (`.:V41`).

/// What the user does about each app.
pub(crate) const NOTE: &str = "install the app; free tier per docs/SERVICES.md";

/// Each v1 bot and the app it assumes, as `docs/SERVICES.md` names it.
const APPS: [(&str, &str); 3] = [
    ("coderabbit", "CodeRabbit"),
    ("greptile", "Greptile"),
    ("socket", "Socket"),
];

/// The app `bot` assumes, if it is a v1 bot.
pub(crate) fn app(bot: &str) -> Option<&'static str> {
    APPS.into_iter()
        .find(|&(b, _)| b == bot)
        .map(|(_, app)| app)
}

/// The v1 bot names, as a fix hint lists them.
pub(crate) fn names() -> String {
    let quoted: Vec<String> =
        APPS.iter().map(|(bot, _)| format!("`{bot}`")).collect();
    quoted.join(", ")
}

#[cfg(test)]
mod tests;
