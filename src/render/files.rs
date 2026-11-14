//! `render` (T38, V31): every `## workflows` row checked against its v1
//! kind and built into `.github/workflows/<id>.yml`, every seeded script
//! a rendered kind needs once, every `## bots` file; sorted bytewise by
//! path (C). Pure: no I/O (`.:V1`).

use std::collections::BTreeSet;

use super::error::RenderError;
use super::kinds::{Entry, lookup, names};
use crate::bots::{File, bot_files};
use crate::kind::{
    Checked, READ_SCRIPT, RUNNER_SYSTEM, check, hosts, read_secret,
};
use crate::pin::PinTable;
use crate::registry::{Registry, RegistryError, WorkflowRow};
use crate::script::scripts;
use crate::system::Runners;

/// Every file `.actlith` asks for, sorted bytewise by path.
///
/// # Errors
///
/// The first row naming no v1 kind or one its kind refuses (naming the
/// row's line), a refused pin or bot row, or two files at one path.
pub(crate) fn render(registry: &Registry) -> Result<Vec<File>, RenderError> {
    let pins = PinTable::builtin().with_overrides(&registry.actions)?;
    let mut needed = BTreeSet::new();
    let mut files = Vec::new();
    for row in &registry.workflows {
        let entry = kind(row)?;
        let checked = check(entry.kind, row, &registry.caches)?;
        files.push(workflow(entry, &checked, &pins, &registry.runners)?);
        needed.extend(entry.scripts.iter().copied());
        needed.extend(entry.kind.scripts(&checked));
        needed.extend(reads_netrc(&checked).then_some(READ_SCRIPT));
        needed.extend(guards(entry, &checked, &registry.runners));
    }
    files.extend(seeded(&needed));
    files.extend(bot_files(&registry.bots)?);
    sorted(files)
}

/// [`RUNNER_SYSTEM`] when a `gate` or `build` row has a system on the
/// repository's own runner: its job starts with that guard (`src/kind` V67).
fn guards(
    entry: Entry,
    row: &Checked,
    runners: &Runners,
) -> Option<&'static str> {
    let guarded = matches!(entry.kind.name(), "gate" | "build");
    (guarded && !hosts(row, runners).own.is_empty()).then_some(RUNNER_SYSTEM)
}

/// Whether `row`'s cache has a read netrc, so every kind's workflow runs
/// [`READ_SCRIPT`] (`src/kind/cache/netrc` V66).
fn reads_netrc(row: &Checked) -> bool {
    row.cache.as_ref().and_then(read_secret).is_some()
}

/// The v1 kind `row` names, or a row error listing them.
fn kind(row: &WorkflowRow) -> Result<Entry, RenderError> {
    lookup(&row.kind).ok_or_else(|| {
        RenderError::Row(RegistryError::new(
            row.line,
            format!("unknown kind `{}`", row.kind.escape_debug()),
            format!("use one of {}", names()),
        ))
    })
}

/// `.github/workflows/<id>.yml`: the checked `row` built by its kind.
fn workflow(
    entry: Entry,
    row: &Checked,
    pins: &PinTable,
    runners: &Runners,
) -> Result<File, RenderError> {
    let built = entry.kind.build(row, pins, runners)?;
    Ok(File {
        path: format!(".github/workflows/{}.yml", row.id),
        bytes: crate::yaml::render(&built),
    })
}

/// Every seeded script whose path is in `needed`.
fn seeded(needed: &BTreeSet<&str>) -> Vec<File> {
    scripts()
        .iter()
        .filter(|(path, _)| needed.contains(path))
        .map(|&(path, bytes)| File {
            path: path.to_owned(),
            bytes: bytes.to_owned(),
        })
        .collect()
}

/// `files` sorted by path, refused when two share one.
fn sorted(mut files: Vec<File>) -> Result<Vec<File>, RenderError> {
    files.sort_by(|a, b| a.path.cmp(&b.path));
    let twice = files.windows(2).find_map(|pair| match pair {
        [a, b] if a.path == b.path => Some(a.path.clone()),
        _ => None,
    });
    twice.map_or(Ok(files), |path| Err(RenderError::Duplicate(path)))
}

#[cfg(test)]
mod tests;
