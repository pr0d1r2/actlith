//! A `## workflows` row checked for its kind by pipelith's
//! `Row::check` (systems, cache, args, timeout and row rules;
//! `src/registry` V10), after actlith names any unknown system in its
//! own words. Every known system has a runner (`src/system` V53), so a
//! checked row is never refused for its systems at render. A kind builds
//! only from a checked row.

use super::contract::Kind;
use super::error::KindError;
use crate::core::kind::{ArgDefault, ArgSpec, Extras, Row};
use crate::registry::WorkflowRow;
use crate::system::parse_system;

/// `xcode`: the Xcode a macOS `gate` job builds with (`src/gate` V52).
/// GitHub-only, since it names a GitHub runner image's install path.
pub(crate) const XCODE: ArgSpec = ArgSpec {
    key: "xcode",
    required: false,
    default: ArgDefault::None,
};

/// `cache-push=false`: a gate may use its cache only as a substituter.
pub(crate) const CACHE_PUSH: ArgSpec = ArgSpec {
    key: "cache-push",
    required: false,
    default: ArgDefault::None,
};

/// The kinds and arg keys only actlith knows (V49): no kind, and the
/// `xcode` key, which the portability check names on any row giving it.
pub(crate) const EXTRAS: Extras<'static> = Extras {
    kinds: &[],
    keys: &[XCODE, CACHE_PUSH],
};

/// `row` checked against `kind`, which takes only the forge keys it
/// names (`Kind::forge_args`).
pub(crate) fn check(
    kind: &dyn Kind,
    row: &WorkflowRow,
) -> Result<Row, KindError> {
    known(row)?;
    let extras = Extras {
        kinds: &[],
        keys: kind.forge_args(),
    };
    let checked = Row::check(kind.spec(), row, &extras)?;
    cache_push_value(&checked)?;
    Ok(checked)
}

/// `cache-push`, when present, is a boolean forge argument.
fn cache_push_value(row: &Row) -> Result<(), KindError> {
    let Some(value) = row.args.get(CACHE_PUSH.key) else {
        return Ok(());
    };
    if matches!(value, "true" | "false") {
        return Ok(());
    }
    Err(KindError::row(
        row.line,
        format!("arg `cache-push` value `{value}` is not true or false"),
        "write `cache-push=true` or `cache-push=false`",
    ))
}

/// Each listed system up to the first repeat (pipelith names that one)
/// is a nix system.
fn known(row: &WorkflowRow) -> Result<(), KindError> {
    let fix = "write nix systems, or `-`";
    let refuse = |e: crate::system::SystemError| {
        KindError::row(row.line, e.to_string(), fix)
    };
    for (n, name) in row.systems.iter().enumerate() {
        if row.systems.iter().take(n).any(|seen| seen == name) {
            return Ok(());
        }
        parse_system(name).map_err(refuse)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests;
