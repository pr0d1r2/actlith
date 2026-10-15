//! [`actlith`]: the entry point `main` calls (RED stub).

use std::ffi::OsString;
use std::io::{self, Write};
use std::process::ExitCode;

use super::exit::USAGE;
use crate::fetch::{Gh, Resolver};

/// Run `actlith` on this process's argv and streams.
#[must_use]
pub fn actlith() -> ExitCode {
    let (mut out, mut err) = (io::stdout().lock(), io::stderr().lock());
    let streams: [&mut dyn Write; 2] = [&mut out, &mut err];
    ExitCode::from(run(std::env::args_os(), streams, &Gh::new()))
}

/// Parse `args`, run the verb, print; the exit code (RED stub: 2).
pub(crate) fn run(
    args: impl IntoIterator<Item = OsString>,
    streams: [&mut dyn Write; 2],
    resolver: &dyn Resolver,
) -> u8 {
    let _ = (args.into_iter().count(), streams, resolver);
    USAGE
}

#[cfg(test)]
mod tests;
