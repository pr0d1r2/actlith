//! The `cron` arg of a bump kind: five cron fields, or a row error
//! naming the line (`src/registry` V10). A schedule GitHub would reject
//! is refused here, not after the file is pushed.

use crate::kind::{Args, KindError};

/// The row's `cron`, its five fields joined by single spaces.
pub(crate) fn cron(args: &Args) -> Result<String, KindError> {
    let value = args.get("cron").unwrap_or_default();
    let fields: Vec<&str> = value.split_whitespace().collect();
    if fields.len() == 5 && fields.iter().all(|f| field(f)) {
        return Ok(fields.join(" "));
    }
    Err(KindError::row(
        args.line(),
        format!("cron `{}` is not five cron fields", value.escape_debug()),
        "write `cron=<minute> <hour> <day> <month> <weekday>`, \
         e.g. `cron=50 6 * * *`",
    ))
}

/// One field: numbers, names, `*`, `-` ranges and `/` steps.
fn field(f: &str) -> bool {
    f.chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '*' | '-' | '/'))
}

#[cfg(test)]
mod tests;
