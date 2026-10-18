//! `.actlith` rows into the sorted [`Report`] (T28, C): each workflow row
//! checked and built by its v1 kind, so the actions listed are the ones
//! its file uses and the secrets the ones its kind declares
//! (`src/kind:V14`).

use std::collections::BTreeSet;

use super::apps::{self, NOTE, app};
use super::error::ReportError;
use super::kinds::{lookup, names};
use super::model::{App, Hosted, Manual, Need, Report, Workflow};
use crate::kind::{Kind, KindError, Row, check};
use crate::pin::{PinTable, Uses};
use crate::registry::{BotRow, Registry, RegistryError, WorkflowRow};
use crate::system::runner;
use crate::yaml::{self, Action};

/// The report of `registry`, its actions pinned from `pins`.
///
/// # Errors
///
/// The first row naming no v1 kind or bot, or one its kind refuses.
pub(crate) fn collect(
    registry: &Registry,
    pins: &PinTable,
) -> Result<Report, ReportError> {
    let rows = registry.workflows.iter();
    let mut workflows: Vec<Workflow> =
        rows.map(|r| workflow(r, pins)).collect::<Result<_, _>>()?;
    workflows.sort_by(|a, b| a.id.cmp(&b.id));
    let mut bots: Vec<App> =
        registry.bots.iter().map(bot).collect::<Result<_, _>>()?;
    bots.sort_by(|a, b| a.bot.cmp(&b.bot));
    Ok(Report {
        workflows,
        hand_written: manual(registry),
        bots,
    })
}

/// One row, checked and built by its kind.
fn workflow(
    row: &WorkflowRow,
    pins: &PinTable,
) -> Result<Workflow, ReportError> {
    let kind = lookup(&row.kind).ok_or_else(|| unknown_kind(row))?;
    let checked = check(kind, row)?;
    let built = kind.build(&checked, pins)?;
    Ok(Workflow {
        id: checked.id.clone(),
        kind: kind.name(),
        systems: hosted(&checked),
        cache: checked.cache.as_ref().map(|c| c.name().to_owned()),
        actions: actions(&built, pins)?,
        secrets: secrets(kind, &checked),
        xcode: checked.args.get("xcode").map(str::to_owned),
    })
}

fn unknown_kind(row: &WorkflowRow) -> ReportError {
    ReportError::Row(RegistryError::new(
        row.line,
        format!("unknown kind `{}`", row.kind.escape_debug()),
        format!("use one of {}", names()),
    ))
}

/// The row's systems with their runner labels; a checked row has one
/// for each.
fn hosted(row: &Row) -> Vec<Hosted> {
    let label = |&s| runner(s).ok().map(|r| (s, r));
    let pairs = row.systems.iter().filter_map(label);
    pairs
        .map(|(system, runner)| Hosted {
            system: system.to_string(),
            runner,
        })
        .collect()
}

/// Every action a step of `built` uses, once each, by name, as pinned.
fn actions(
    built: &yaml::Workflow,
    pins: &PinTable,
) -> Result<Vec<Uses>, KindError> {
    let steps = built.jobs.values().flat_map(|job| &job.steps);
    let names: BTreeSet<&str> = steps
        .filter_map(|step| match &step.action {
            Action::Uses(uses) => uses.reference.split_once('@'),
            Action::Run(_) => None,
        })
        .map(|(action, _)| action)
        .collect();
    let pinned = names.into_iter().map(|action| pins.uses(action));
    Ok(pinned.collect::<Result<_, _>>()?)
}

/// The secrets `kind` declares for `row`, once each, by name.
fn secrets(kind: &dyn Kind, row: &Row) -> Vec<Need> {
    let needs = kind.secrets(row).into_iter().map(|s| Need {
        name: s.name().to_owned(),
        purpose: s.purpose(),
    });
    needs.collect::<BTreeSet<_>>().into_iter().collect()
}

/// `## hand-written` rows, by path.
fn manual(registry: &Registry) -> Vec<Manual> {
    let mut rows: Vec<Manual> = registry
        .hand_written
        .iter()
        .map(|h| Manual {
            path: h.path.clone(),
            reason: h.reason.clone(),
        })
        .collect();
    rows.sort_by(|a, b| a.path.cmp(&b.path));
    rows
}

/// The app a `## bots` row assumes, or a row error naming its line.
fn bot(row: &BotRow) -> Result<App, ReportError> {
    let name = app(&row.bot).ok_or_else(|| {
        ReportError::Row(RegistryError::new(
            row.line,
            format!("unknown bot `{}`", row.bot.escape_debug()),
            format!("use one of {}", apps::names()),
        ))
    })?;
    Ok(App {
        bot: row.bot.clone(),
        name,
        note: NOTE,
    })
}

#[cfg(test)]
mod tests;
