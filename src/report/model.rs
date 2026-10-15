//! What `report` says about a `.actlith`, before any format prints it
//! (T28). Shape only; every list is sorted by its first field (C).

use crate::pin::Uses;

/// The whole report.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Report {
    /// One per `## workflows` row, by id.
    pub(crate) workflows: Vec<Workflow>,
    /// One per `## hand-written` row, by path.
    pub(crate) hand_written: Vec<Manual>,
    /// One per `## bots` row, by bot.
    pub(crate) bots: Vec<App>,
}

/// A generated workflow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Workflow {
    /// File stem of `.github/workflows/<id>.yml`.
    pub(crate) id: String,
    /// The kind name.
    pub(crate) kind: &'static str,
    /// Each system with its runner label, in row order.
    pub(crate) systems: Vec<Hosted>,
    /// The cachix cache, if any.
    pub(crate) cache: Option<String>,
    /// Every action the workflow uses, pinned, by action.
    pub(crate) actions: Vec<Uses>,
    /// Every secret the workflow reads (`src/kind:V14`), by name.
    pub(crate) secrets: Vec<Need>,
}

/// A nix system and the hosted runner it runs on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Hosted {
    pub(crate) system: String,
    pub(crate) runner: &'static str,
}

/// A secret and what the workflow reads it for.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Need {
    pub(crate) name: String,
    pub(crate) purpose: &'static str,
}

/// A workflow file actlith does not generate, and why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Manual {
    pub(crate) path: String,
    pub(crate) reason: String,
}

/// The GitHub App a `## bots` row assumes is installed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct App {
    /// The bot name as the row writes it.
    pub(crate) bot: String,
    /// The app's name.
    pub(crate) name: &'static str,
    /// What the user does about it.
    pub(crate) note: &'static str,
}
