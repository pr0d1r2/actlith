//! A job on the repository's own runner builds the system its row names,
//! never the runner's (V67). The first nix step of such a job is the
//! seeded `runner-system.sh` guard; a `## runners` row whose labels name
//! another architecture or OS is refused unless the row chose emulation
//! with `emulate=<system>` (Rosetta, binfmt).

use super::error::KindError;
use super::job::Hosts;
use super::run::run;
use crate::core::kind::Row;
use crate::system::{System, parse_system};
use crate::yaml::{Comment, Step};

/// The seeded script every own-runner job starts with (V67).
pub(crate) const RUNNER_SYSTEM: &str = "scripts/actlith/runner-system.sh";

const WHY: &str = "The job's system, not the runner's: stops unless this \
    runner's nix builds it\nnatively or the row chose emulation, which then \
    sets `system` for every later\nstep.";

/// The systems the row chose to emulate: `emulate=<system>[+<system>]`.
pub(crate) fn emulated(row: &Row) -> Vec<System> {
    let value = row.args.get("emulate").unwrap_or_default();
    value
        .split('+')
        .filter_map(|s| parse_system(s).ok())
        .collect()
}

/// `emulate`, when present, is `+`-joined nix systems.
pub(crate) fn emulate_value(row: &Row) -> Result<(), KindError> {
    let Some(value) = row.args.get("emulate") else {
        return Ok(());
    };
    let bad = value.split('+').find_map(|s| parse_system(s).err());
    bad.map_or(Ok(()), |error| {
        Err(KindError::row(
            row.line,
            format!("arg `emulate` value `{}`: {error}", value.escape_debug()),
            "write systems joined by `+`, as in `emulate=x86_64-darwin`",
        ))
    })
}

/// The guard step for `system`'s own-runner job, first after install-nix.
pub(crate) fn guard(row: &Row, system: System) -> Result<Step, KindError> {
    let mark = if emulated(row).contains(&system) {
        "EMULATE=true "
    } else {
        ""
    };
    let command = format!("{mark}bash {RUNNER_SYSTEM} {system}");
    let mut step = run(row.line, &command, &[])?;
    step.comments = vec![Comment::new(WHY)];
    Ok(step)
}

/// Whether `label` names an architecture or OS other than `system`'s.
fn names_other(system: System, label: &str) -> bool {
    let Some((arch, os)) = system.name().split_once('-') else {
        return false;
    };
    match label.to_ascii_lowercase().as_str() {
        "x64" | "x86_64" | "amd64" => arch != "x86_64",
        "arm64" | "aarch64" => arch != "aarch64",
        "macos" | "darwin" => os != "darwin",
        "linux" => os != "linux",
        "windows" => os != "windows",
        _ => false,
    }
}

/// The error for `label` on `system`'s `## runners` row.
fn silent(line: usize, system: System, label: &str) -> KindError {
    KindError::row(
        line,
        format!(
            "`## runners` row for `{system}` has label `{label}`, which \
             names another architecture or OS: the job would run on that \
             hardware, so it would gate the runner's system, not `{system}`"
        ),
        format!(
            "give `{system}` a runner of its own hardware, or choose \
             Rosetta/binfmt with `emulate={system}` in the row's args"
        ),
    )
}

/// Refused: an own-runner system whose labels name a different
/// architecture or OS, without `emulate=<system>` on the row (V67).
pub(crate) fn refuse_silent(
    row: &Row,
    hosts: &Hosts<'_>,
) -> Result<(), KindError> {
    let chosen = emulated(row);
    let found = hosts.own.iter().find_map(|&(system, labels)| {
        let label = labels.iter().find(|l| names_other(system, l));
        label
            .filter(|_| !chosen.contains(&system))
            .map(|l| (system, l))
    });
    found.map_or(Ok(()), |(system, label)| {
        Err(silent(row.line, system, label))
    })
}

#[cfg(test)]
mod tests;
