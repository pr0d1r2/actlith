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
    let _ = (command, env);
    Err(KindError::row(line, "stub", "stub"))
}

#[cfg(test)]
mod tests;
