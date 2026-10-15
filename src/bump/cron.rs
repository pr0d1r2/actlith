//! The `cron` arg of a bump kind: five cron fields, or a row error
//! naming the line (`src/registry` V10). RED stub.

use crate::kind::{Args, KindError};

/// The row's `cron`, its five fields joined by single spaces.
pub(crate) fn cron(args: &Args) -> Result<String, KindError> {
    Err(KindError::row(args.line(), "stub", "stub"))
}

#[cfg(test)]
mod tests;
