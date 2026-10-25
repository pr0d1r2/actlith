//! Where a system's jobs run on GitHub (V53): the repository's own
//! runner when `## runners` names one, otherwise the hosted label from
//! the one table (V15). pipelith's `Runners::pick` makes the choice,
//! told whether the table has a label for the system.

use super::table::{System, runner};
pub(crate) use crate::core::runner::Runners;

#[cfg(test)]
mod tests;

/// The runner a system's jobs run on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Host<'a> {
    /// A GitHub-hosted runner, by its label in the table.
    Hosted(&'static str),
    /// The repository's own runner: one carrying every one of these
    /// labels (`## runners`). Its jobs run only for trusted events
    /// (`src/kind` V54).
    #[cfg_attr(
        not(test),
        allow(dead_code, reason = "made once host reads `## runners` (T72)")
    )]
    Own(&'a [String]),
}

/// The host of `system`: its `## runners` labels when `runners` has a
/// row for it, otherwise its hosted label. Every system has a hosted
/// label (V53), so pipelith's `Runners::pick` is asked with a default
/// and never refuses: a checked row cannot fail at render.
pub(crate) fn host(_runners: &Runners, system: System) -> Host<'_> {
    Host::Hosted(runner(system))
}
