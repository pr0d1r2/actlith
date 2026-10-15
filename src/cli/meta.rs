//! `completions <shell>` and `man` (RED stub).

use clap_complete::Shell;

use super::exit::Outcome;

/// `completions SHELL` (RED stub).
pub(crate) fn completions(_shell: Shell) -> Outcome {
    Outcome::default()
}

/// `man` (RED stub).
pub(crate) fn man() -> Outcome {
    Outcome::default()
}

#[cfg(test)]
mod tests;
