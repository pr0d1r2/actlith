//! Kind `gate` (T19): the hk gate on every system, the shape of pklith's
//! `ci.yml` (C). One job `gate` (stable, `src/kind` V9) per system;
//! checkout keeps full history, cachix pushes except from fork PRs, then
//! the one gate command runs. A macOS row may pin its Xcode (T69, V52).

use std::collections::BTreeMap;

use crate::kind::{
    ArgSpec, CachixPush, Checkout, GATE, Kind, KindError, KindSpec, Row,
    Secret, XCODE, cachix, checkout, install_nix, job, matrix_runner,
    on_push_and_pr, run, runner_matrix, workflow,
};
use crate::pin::PinTable;
use crate::yaml::{Comment, Step, Text, Workflow};

/// The job id and name; `pklith protect` reads it (`src/kind` V9).
const JOB: &str = "gate";

/// The gate run when the row gives no `command`.
const COMMAND: &str = "nix develop --command hk check --all --check";

/// The Xcode check: the pinned Xcode is on the image, else the job fails
/// naming its path; never the image's default (V52).
const XCODE_CHECK: &str = "test -d \"$DEVELOPER_DIR\" || { echo \
    \"::error::missing $DEVELOPER_DIR\"; exit 1; }; xcodebuild -version";

const DEPTH_WHY: &str = "Full history: tdd-order reads the push range, and \
    a later semver\nstep compares with the last release tag.";

/// The kind `gate`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Gate;

impl Kind for Gate {
    fn spec(&self) -> &'static KindSpec {
        &GATE
    }

    fn forge_args(&self) -> &'static [ArgSpec] {
        &[XCODE]
    }

    fn secrets(&self, row: &Row) -> Vec<Secret> {
        let push = row.cache.as_ref().and(CachixPush::SkipForkPr.secret());
        push.into_iter().collect()
    }

    fn build(&self, row: &Row, pins: &PinTable) -> Result<Workflow, KindError> {
        let xcode = xcode(row)?;
        let steps = steps(row, pins, xcode)?;
        let mut gate = job(matrix_runner(), row.args.timeout()?, steps);
        if let Some(version) = xcode {
            let dir =
                format!("/Applications/Xcode_{version}.app/Contents/Developer");
            gate.env.insert("DEVELOPER_DIR".to_owned(), Text::lit(&dir));
        }
        gate.name = Some(JOB.to_owned());
        gate.matrix = Some(runner_matrix(row)?);
        let jobs = BTreeMap::from([(JOB.to_owned(), gate)]);
        Ok(workflow(&row.id, on_push_and_pr(), jobs))
    }
}

/// The row's `xcode` version: digits and dots, on darwin systems only.
fn xcode(row: &Row) -> Result<Option<&str>, KindError> {
    let Some(version) = row.args.get("xcode") else {
        return Ok(None);
    };
    let number =
        |p: &str| !p.is_empty() && p.bytes().all(|b| b.is_ascii_digit());
    if !version.split('.').all(number) {
        let what = format!(
            "arg `xcode` value `{}` is not an Xcode version",
            version.escape_debug()
        );
        let fix = "write digits and dots, as in `xcode=26.4`";
        return Err(KindError::row(row.line, what, fix));
    }
    darwin_only(row)?;
    Ok(Some(version))
}

/// Refused when a system of the row is Linux: no Xcode there (V52).
fn darwin_only(row: &Row) -> Result<(), KindError> {
    let linux = row.systems.iter().find(|s| s.name().ends_with("-linux"));
    linux.map_or(Ok(()), |linux| {
        Err(KindError::row(
            row.line,
            format!(
                "arg `xcode` needs a macOS runner; system `{linux}` is Linux"
            ),
            "list only darwin systems, such as `aarch64-darwin`",
        ))
    })
}

/// The first step of an `xcode` row, a comment above it saying why.
fn xcode_check(row: &Row, version: &str) -> Result<Step, KindError> {
    let mut check = run(row.line, XCODE_CHECK, &[])?;
    check.comments = vec![Comment::new(&format!(
        "xcode={version}: this Xcode, never the image's default. An image \
         without\nit fails here, and the log names the Xcode the gate ran \
         with."
    ))];
    Ok(check)
}

/// The Xcode check when the row pins one, checkout, install-nix, cachix
/// when the row has a cache, the gate.
fn steps(
    row: &Row,
    pins: &PinTable,
    xcode: Option<&str>,
) -> Result<Vec<Step>, KindError> {
    let how = Checkout {
        pushes: false,
        fetch_depth: Some(0),
        why_depth: Some(Comment::new(DEPTH_WHY)),
    };
    let check = xcode.map(|v| xcode_check(row, v)).transpose()?;
    let mut steps: Vec<Step> = check.into_iter().collect();
    steps.push(checkout(pins, &how)?);
    steps.push(install_nix(pins, row.cache.as_ref())?);
    if let Some(cache) = &row.cache {
        steps.push(cachix(pins, cache, CachixPush::SkipForkPr)?);
    }
    let command = row.args.get("command").unwrap_or(COMMAND);
    steps.push(run(row.line, command, &[])?);
    Ok(steps)
}

#[cfg(test)]
mod tests;
