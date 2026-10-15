//! A bot row's `args` cell against the bot's own schema (V38).

use super::error::BotError;
use crate::registry::BotRow;

/// Every key of `row` is one of `known`.
///
/// # Errors
///
/// The first key the bot does not take, naming the row's line.
pub(crate) fn known(row: &BotRow, known: &[&str]) -> Result<(), BotError> {
    let Some(arg) = row.args.iter().find(|a| !known.contains(&a.key.as_str()))
    else {
        return Ok(());
    };
    let what = format!("bot `{}` has no arg `{}`", row.bot, arg.key);
    Err(BotError::new(row.line, what, schema(known)))
}

/// The fix hint naming the args a bot takes.
fn schema(known: &[&str]) -> String {
    if known.is_empty() {
        return "it takes no args; write `-`".to_owned();
    }
    let keys: Vec<String> = known.iter().map(|k| format!("`{k}`")).collect();
    format!("its args: {}", keys.join(", "))
}

/// The value of `key`, which the bot needs.
///
/// # Errors
///
/// The row leaves `key` out; `hint` is the value to write.
pub(crate) fn required<'a>(
    row: &'a BotRow,
    key: &str,
    hint: &str,
) -> Result<&'a str, BotError> {
    let found = row.args.iter().find(|a| a.key == key);
    found.map(|a| a.value.as_str()).ok_or_else(|| {
        let what = format!("bot `{}` needs arg `{key}`", row.bot);
        BotError::new(row.line, what, format!("add `{key}={hint}`"))
    })
}

#[cfg(test)]
mod tests;
