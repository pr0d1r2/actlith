//! A data row into its section's typed row; one key per section given
//! once (registry T13).

use std::collections::BTreeMap;

use super::cells::{args, fixed, list, optional, required, split};
use super::error::RegistryError;
use super::model::{ActionPin, BotRow, HandWritten, Registry, WorkflowRow};
use super::table::Table;

/// Type `row` (on `line`) as a row of `table` and append it to `reg`.
///
/// # Errors
///
/// A bad cell count, an empty required cell, a malformed list, arg, id
/// or SHA.
pub(crate) fn add(
    reg: &mut Registry,
    table: Table,
    line: usize,
    row: &str,
) -> Result<(), RegistryError> {
    let cells = split(row);
    match table {
        Table::Workflows => reg.workflows.push(workflow(line, cells)?),
        Table::Actions => reg.actions.push(action(line, cells)?),
        Table::HandWritten => reg.hand_written.push(hand(line, cells)?),
        Table::Bots => reg.bots.push(bot(line, cells)?),
    }
    Ok(())
}

/// `id|kind|systems|cache|args`.
fn workflow(
    line: usize,
    cells: Vec<String>,
) -> Result<WorkflowRow, RegistryError> {
    let [id, kind, systems, cache, cell] =
        fixed(line, Table::Workflows, cells)?;
    Ok(WorkflowRow {
        line,
        id: file_name(line, required(line, "id", id)?)?,
        kind: required(line, "kind", kind)?,
        systems: list(line, "systems", &systems)?,
        cache: optional(&cache),
        args: args(line, &cell)?,
    })
}

/// `action|version|sha`.
fn action(line: usize, cells: Vec<String>) -> Result<ActionPin, RegistryError> {
    let [action, version, sha] = fixed(line, Table::Actions, cells)?;
    Ok(ActionPin {
        line,
        action: required(line, "action", action)?,
        version: required(line, "version", version)?,
        sha: full_sha(line, required(line, "sha", sha)?)?,
    })
}

/// `path|reason`.
fn hand(line: usize, cells: Vec<String>) -> Result<HandWritten, RegistryError> {
    let [path, reason] = fixed(line, Table::HandWritten, cells)?;
    Ok(HandWritten {
        line,
        path: required(line, "path", path)?,
        reason: required(line, "reason", reason)?,
    })
}

/// `bot|args`.
fn bot(line: usize, cells: Vec<String>) -> Result<BotRow, RegistryError> {
    let [bot, cell] = fixed(line, Table::Bots, cells)?;
    Ok(BotRow {
        line,
        bot: required(line, "bot", bot)?,
        args: args(line, &cell)?,
    })
}

/// A workflow id is a plain file stem: `[A-Za-z0-9._-]`, no leading dot.
fn file_name(line: usize, id: String) -> Result<String, RegistryError> {
    let plain = |c: char| c.is_ascii_alphanumeric() || "._-".contains(c);
    if id.chars().all(plain) && !id.starts_with('.') {
        return Ok(id);
    }
    let what = format!("id `{id}` is not a file name");
    let fix = "use letters, digits, `.`, `_`, `-`; no leading `.`";
    Err(RegistryError::new(line, what, fix))
}

/// A pin is a full commit SHA: 40 lowercase hex digits.
fn full_sha(line: usize, sha: String) -> Result<String, RegistryError> {
    let hex = |c: char| c.is_ascii_digit() || ('a'..='f').contains(&c);
    if sha.len() == 40 && sha.chars().all(hex) {
        return Ok(sha);
    }
    let what = format!("sha `{sha}` is not a full commit SHA");
    let fix = "write the 40-hex commit SHA (`actlith pin` resolves it)";
    Err(RegistryError::new(line, what, fix))
}

/// Each section's key (workflow id, action, path, bot) appears once.
///
/// # Errors
///
/// The second row with a key, naming the first one's line.
pub(crate) fn unique(reg: &Registry) -> Result<(), RegistryError> {
    once("id", reg.workflows.iter().map(|r| (r.line, r.id.as_str())))?;
    once(
        "action",
        reg.actions.iter().map(|r| (r.line, r.action.as_str())),
    )?;
    let paths = reg.hand_written.iter().map(|r| (r.line, r.path.as_str()));
    once("path", paths)?;
    once("bot", reg.bots.iter().map(|r| (r.line, r.bot.as_str())))
}

/// The first `(line, key)` whose key was seen before is refused.
fn once<'a>(
    column: &str,
    keys: impl Iterator<Item = (usize, &'a str)>,
) -> Result<(), RegistryError> {
    let mut seen: BTreeMap<&str, usize> = BTreeMap::new();
    for (line, key) in keys {
        if let Some(first) = seen.insert(key, line) {
            let what =
                format!("{column} `{key}` given twice, first on line {first}");
            return Err(RegistryError::new(line, what, "keep one row per key"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
