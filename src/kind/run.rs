//! A `run:` step (V3): one line, never an expression inside it. A value
//! GitHub evaluates reaches the command through `env:`, so it is never
//! spliced into shell text.

use super::error::KindError;
use super::step::step;
use crate::yaml::{Action, Env, Expr, Step, Text};

/// A `run: <command>` step, with each `(name, expr)` passed as an env
/// variable. Refused when the command is empty, spans lines or holds
/// `${{`, or a variable name is not a shell name.
pub(crate) fn run(
    line: usize,
    command: &str,
    env: &[(&str, Expr)],
) -> Result<Step, KindError> {
    check(line, command)?;
    let mut vars = Env::new();
    for (name, value) in env {
        vars.insert(variable(line, name)?, Text::expr(value.clone()));
    }
    Ok(Step {
        env: vars,
        ..step(Action::Run(command.to_owned()))
    })
}

fn check(line: usize, command: &str) -> Result<(), KindError> {
    let what = if command.trim().is_empty() {
        "is empty"
    } else if command.contains(['\n', '\r']) {
        "spans more than one line"
    } else if command.contains("${{") {
        "holds `${{`, an expression inside `run:`"
    } else {
        return Ok(());
    };
    Err(KindError::row(
        line,
        format!("command `{}` {what}", command.escape_debug()),
        "write one command line; pass values through env, longer logic \
         through a script",
    ))
}

fn variable(line: usize, name: &str) -> Result<String, KindError> {
    let word = |c: char| c.is_ascii_alphanumeric() || c == '_';
    let first = name.chars().next().filter(|c| !c.is_ascii_digit());
    if first.is_some() && name.chars().all(word) {
        return Ok(name.to_owned());
    }
    Err(KindError::row(
        line,
        format!(
            "env name `{}` is not a shell variable name",
            name.escape_debug()
        ),
        "use letters, digits and `_`, not starting with a digit",
    ))
}

#[cfg(test)]
mod tests;
