//! The typed rows of a parsed `.actlith` (registry I). Shape only: kind,
//! system, arg and bot names are checked by the nodes that own them
//! (registry V10).

/// A parsed `.actlith`. A section the file leaves out is empty
/// (registry V12).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Registry {
    /// `## workflows` rows, in file order.
    pub(crate) workflows: Vec<WorkflowRow>,
    /// `## actions` rows: pins overriding the built-in table.
    pub(crate) actions: Vec<ActionPin>,
    /// `## hand-written` rows: workflow files actlith does not generate.
    pub(crate) hand_written: Vec<HandWritten>,
    /// `## bots` rows (`src/bots` I).
    pub(crate) bots: Vec<BotRow>,
    /// `## runners` rows: a system's own runner labels, checked by
    /// pipelith (`src/system` V53).
    pub(crate) runners: crate::core::runner::Runners,
}

/// A `key=value` arg: pipelith's (src/core).
pub(crate) use crate::core::registry::Arg;

/// A `## workflows` row: pipelith's neutral `kind::RawRow` (src/core).
pub(crate) type WorkflowRow = crate::core::kind::RawRow;

/// A `## actions` row: `action|version|sha`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ActionPin {
    /// The 1-based line the row is on.
    pub(crate) line: usize,
    /// `owner/repo` (or `owner/repo/path`).
    pub(crate) action: String,
    /// The tag the SHA was resolved from.
    pub(crate) version: String,
    /// The full 40-hex commit SHA.
    pub(crate) sha: String,
}

/// A `## hand-written` row: `path|reason`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct HandWritten {
    /// The 1-based line the row is on.
    pub(crate) line: usize,
    /// Repository-relative path of the workflow file.
    pub(crate) path: String,
    /// Why it is not generated; never empty.
    pub(crate) reason: String,
}

/// A `## bots` row: `bot|args` (`src/bots` I).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BotRow {
    /// The 1-based line the row is on.
    pub(crate) line: usize,
    /// The bot name, validated by `src/bots`.
    pub(crate) bot: String,
    /// `key=value` pairs, in cell order.
    pub(crate) args: Vec<Arg>,
}
