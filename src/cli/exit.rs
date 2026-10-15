//! Exit codes (V34) and [`Outcome`]: what a verb prints on each stream
//! and the code it exits with.

use std::fmt::{Display, Write as _};
use std::io;
use std::path::Path;

/// Done; nothing found.
pub(crate) const OK: u8 = 0;
/// A finding: a stale, missing or undeclared file, an import mismatch, a
/// seed refused over an existing `.actlith`.
pub(crate) const FINDING: u8 = 1;
/// Usage, unreadable input, a refused row or a newer format.
pub(crate) const USAGE: u8 = 2;

/// What a verb prints: results on stdout, diagnostics on stderr.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Outcome {
    /// The exit code.
    pub(crate) code: u8,
    /// The results, for stdout.
    pub(crate) out: String,
    /// The diagnostics, for stderr; each names its fix.
    pub(crate) err: String,
}

/// A verb's result: `Err` stops it early. Both are printed alike.
pub(crate) type Ran = Result<Outcome, Outcome>;

impl Outcome {
    /// Exit 0 printing `out`.
    pub(crate) fn ok(out: impl Into<String>) -> Self {
        Self {
            code: OK,
            out: out.into(),
            err: String::new(),
        }
    }

    /// Exit `code` with `err` on stderr, ending in one newline.
    pub(crate) fn fail(code: u8, err: impl Display) -> Self {
        let mut err = err.to_string();
        if !err.ends_with('\n') {
            err.push('\n');
        }
        let out = String::new();
        Self { code, out, err }
    }

    /// Exit 2: `verb` failed on `path` with `error`.
    pub(crate) fn io(path: &Path, verb: &str, error: &io::Error) -> Self {
        let path = path.display();
        Self::fail(
            USAGE,
            format!(
                "{path}: cannot {verb}: {error}\n  fix: check the path and \
                 its permissions"
            ),
        )
    }

    /// `self` then `next` on each stream; the larger code.
    #[must_use]
    pub(crate) fn and(mut self, next: &Self) -> Self {
        self.code = self.code.max(next.code);
        self.out.push_str(&next.out);
        self.err.push_str(&next.err);
        self
    }
}

/// Each item on its own line, every line ending in a newline.
pub(crate) fn lines<T: Display>(items: impl IntoIterator<Item = T>) -> String {
    items.into_iter().fold(String::new(), |mut text, item| {
        let _ = writeln!(text, "{item}");
        text
    })
}

#[cfg(test)]
mod tests;
