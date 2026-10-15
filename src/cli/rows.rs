//! `detect`, `seed --init` and `import FILE...`: the verbs that propose
//! `## workflows` rows (`.:I`, V34). Dispatch only: the rows come from
//! `src/detect` and `src/import`.

use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use super::exit::{FINDING, Outcome, Ran, USAGE, lines};
use super::ls_files::tracked;
use crate::detect::{SeedError, detect as propose, row_line, seed_init};
use crate::import::import as matched;
use crate::registry::WorkflowRow;

/// `detect`: the proposed rows, one per line.
///
/// # Errors
///
/// Exit 2: the tracked file list cannot be read.
pub(crate) fn detect(root: &Path) -> Ran {
    let rows = proposed(root)?;
    Ok(Outcome::ok(lines(rows.iter().map(row_line))))
}

/// `seed --init`: write the first `.actlith` from `detect`'s rows.
///
/// # Errors
///
/// Exit 2 without `--init` or when the file cannot be written; exit 1
/// when `.actlith` is already there (`src/detect:V32`).
pub(crate) fn seed(root: &Path, init: bool) -> Ran {
    if !init {
        return Err(Outcome::fail(
            USAGE,
            "seed: nothing to do without --init\n  fix: run `actlith seed \
             --init`",
        ));
    }
    let rows = proposed(root)?;
    let path = seed_init(root, &rows).map_err(|e| refused(&e))?;
    Ok(Outcome::ok(format!("wrote {}\n", path.display())))
}

/// The rows `detect` proposes for the files tracked under `root`.
fn proposed(root: &Path) -> Result<Vec<WorkflowRow>, Outcome> {
    let files = tracked(root)?;
    let lock = fs::read_to_string(root.join("flake.lock")).ok();
    Ok(propose(&files, lock.as_deref()))
}

/// Why `seed --init` wrote nothing, with the fix.
fn refused(error: &SeedError) -> Outcome {
    let fix = match error {
        SeedError::Exists(_) => "edit it by hand, or delete it to seed again",
        SeedError::Write(..) => "make the root writable, or pass --root DIR",
    };
    Outcome::fail(error.code(), format!("{error}\n  fix: {fix}"))
}

/// `import FILE...`: one row per file that a kind renders exactly; exit
/// 1 naming the first difference of each that does not, 2 for one that
/// cannot be read. Never stops early: the code is the worst.
pub(crate) fn import(files: &[PathBuf]) -> Outcome {
    let each = files.iter().map(|file| one(file));
    each.fold(Outcome::ok(""), |all, next| all.and(&next))
}

/// The row of one workflow file, or why there is none.
fn one(file: &Path) -> Outcome {
    let text = match fs::read_to_string(file) {
        Ok(text) => text,
        Err(error) => return Outcome::io(file, "read", &error),
    };
    match matched(&text) {
        Ok(row) => named(file, &row),
        Err(why) => {
            Outcome::fail(FINDING, format!("{}: {why}", file.display()))
        }
    }
}

/// `row` when `file` is `<id>.yml`, the file `gen` would write for it.
fn named(file: &Path, row: &WorkflowRow) -> Outcome {
    if file.file_stem() == Some(OsStr::new(&row.id)) {
        return Outcome::ok(format!("{}\n", row_line(row)));
    }
    let id = &row.id;
    Outcome::fail(
        FINDING,
        format!(
            "{}: renders as row `{id}`, which `gen` writes as `{id}.yml`\n  \
             fix: rename the file to `{id}.yml`, or list it under \
             `## hand-written`",
            file.display()
        ),
    )
}

#[cfg(test)]
mod tests;
