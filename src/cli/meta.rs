//! `completions <shell>` and `man`: generated from the argv table, so
//! they never drift from what parses (`.:I`).

use clap_complete::Shell;
use clap_mangen::Man;

use super::args::command;
use super::exit::Outcome;

/// `completions SHELL`: the completion script on stdout.
pub(crate) fn completions(shell: Shell) -> Outcome {
    let mut bytes = Vec::new();
    clap_complete::generate(shell, &mut command(), "actlith", &mut bytes);
    Outcome::ok(String::from_utf8_lossy(&bytes))
}

/// `man`: the man page in roff on stdout.
pub(crate) fn man() -> Outcome {
    let mut bytes = Vec::new();
    // Rendering into a `Vec` cannot fail: the only error is a write's.
    let _ = Man::new(command()).render(&mut bytes);
    Outcome::ok(String::from_utf8_lossy(&bytes))
}

#[cfg(test)]
mod tests;
