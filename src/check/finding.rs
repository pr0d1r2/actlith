//! One `check` finding: the path, what is wrong, and the fix (V7, V38).

use std::fmt;

/// What is wrong with a path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Problem {
    /// A workflow file no row generates and no `## hand-written` row lists.
    Undeclared,
    /// A `## hand-written` path with no file behind it.
    Missing,
    /// A `## hand-written` path that a `## workflows` row also renders.
    AlsoRendered,
    /// A `## hand-written` row with an empty reason.
    NoReason,
    /// A bot config file no `## bots` row renders.
    BotWithoutRow,
}

/// A finding on one path; `line` is the `.actlith` row it came from.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Finding {
    /// The repository-relative path at fault.
    pub(crate) path: String,
    /// The `.actlith` line of the row at fault, if a row is.
    pub(crate) line: Option<usize>,
    /// What is wrong.
    pub(crate) problem: Problem,
}

impl Problem {
    /// What is wrong, in words.
    pub(crate) fn what(self) -> &'static str {
        let _ = self;
        ""
    }

    /// How to fix it, in words.
    pub(crate) fn fix(self) -> &'static str {
        let _ = self;
        ""
    }
}

impl fmt::Display for Finding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path)
    }
}

#[cfg(test)]
mod tests;
