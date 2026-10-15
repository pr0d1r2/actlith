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

/// `Undeclared`: what and fix.
const UNDECLARED: (&str, &str) = (
    "workflow file is neither generated nor listed hand-written",
    "add a `## workflows` row that generates it, or a `## hand-written` \
     row with a reason",
);

/// `Missing`: what and fix.
const MISSING: (&str, &str) = (
    "listed hand-written but the file is missing",
    "restore the file, or drop its `## hand-written` row",
);

/// `AlsoRendered`: what and fix.
const ALSO_RENDERED: (&str, &str) = (
    "listed hand-written but also generated",
    "drop its `## hand-written` row; a `## workflows` row owns it",
);

/// `NoReason`: what and fix.
const NO_REASON: (&str, &str) = (
    "hand-written row has an empty reason",
    "say in the `## hand-written` reason cell why it is not generated",
);

/// `BotWithoutRow`: what and fix.
const BOT_WITHOUT_ROW: (&str, &str) = (
    "bot config file has no `## bots` row",
    "add a `## bots` row for it, or delete the file",
);

impl Problem {
    /// What is wrong and how to fix it, in words.
    fn texts(self) -> (&'static str, &'static str) {
        match self {
            Self::Undeclared => UNDECLARED,
            Self::Missing => MISSING,
            Self::AlsoRendered => ALSO_RENDERED,
            Self::NoReason => NO_REASON,
            Self::BotWithoutRow => BOT_WITHOUT_ROW,
        }
    }

    /// What is wrong, in words.
    pub(crate) fn what(self) -> &'static str {
        self.texts().0
    }

    /// How to fix it, in words.
    pub(crate) fn fix(self) -> &'static str {
        self.texts().1
    }
}

impl fmt::Display for Finding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path)?;
        if let Some(line) = self.line {
            write!(f, " (.actlith line {line})")?;
        }
        let problem = self.problem;
        write!(f, ": {}; fix: {}", problem.what(), problem.fix())
    }
}

#[cfg(test)]
mod tests;
