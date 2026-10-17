//! Kind `upstream` (T23): poll an upstream release with the repo's own
//! bump script and propose the bump as a PR through the seeded
//! `open-pr.sh`, idempotent per V26. Shape: nix-hk `upstream.yml`
//! (`.:R3`); the cache is only read, the PR's own CI pushes.

use crate::yaml::GITHUB;
use std::collections::BTreeMap;

use super::cron::cron;
use super::shared::{LIMITS, bump_job, plain, pushing_checkout, token};
use crate::kind::{
    CachixPush, Kind, KindError, KindSpec, Row, Secret, UPSTREAM, cachix,
    install_nix, on_schedule, run, workflow,
};
use crate::pin::PinTable;
use crate::yaml::{Comment, Expr, Step, Text, Workflow};

const WHY: &str = "Poll upstream for a new release and propose the bump as \
    a PR. Pull model:\nno cross-repo token, nothing upstream needs to know \
    this repo exists.\nThe bump script prints `NEW_VERSION=<version>`; \
    nothing printed, no PR.";

const CACHE_WHY: &str = "Read-only here. The push happens on the PR's own \
    CI run, once a human\nhas looked at it.";

/// The bump script's stdout is the step's output; `pipefail` keeps its
/// failure from hiding behind `tee`.
const BUMP: &str = "set -o pipefail; \"./$SCRIPT\" | tee -a \"$GITHUB_OUTPUT\"";

/// Branch, title and body reach `open-pr.sh` through `env:`, never
/// spliced into the command (`src/kind:V3`).
const OPEN: &str =
    "bash scripts/actlith/open-pr.sh \"$BRANCH\" \"$TITLE\" \"$BODY\"";

const VERSION: &str = "steps.bump.outputs.NEW_VERSION";

/// Kind `upstream`.
pub(crate) struct Upstream;

impl Kind for Upstream {
    fn spec(&self) -> &'static KindSpec {
        &UPSTREAM
    }

    /// The token only: the cachix step never pushes, so it reads no
    /// `CACHIX_AUTH_TOKEN`.
    fn secrets(&self, row: &Row) -> Vec<Secret> {
        token(&row.args).into_iter().collect()
    }

    fn build(&self, row: &Row, pins: &PinTable) -> Result<Workflow, KindError> {
        let bump = bump_job(row, steps(row, pins)?)?;
        let jobs = BTreeMap::from([("bump".to_owned(), bump)]);
        let mut built = workflow(&row.id, on_schedule(&cron(&row.args)?), jobs);
        built.comments = vec![Comment::new(&format!("{WHY}\n\n{LIMITS}"))];
        Ok(built)
    }
}

fn steps(row: &Row, pins: &PinTable) -> Result<Vec<Step>, KindError> {
    let token = token(&row.args)?;
    let mut steps = vec![
        pushing_checkout(pins, &token)?,
        install_nix(pins, row.cache.as_ref())?,
    ];
    if let Some(cache) = &row.cache {
        let mut read = cachix(pins, cache, CachixPush::ReadOnly)?;
        read.comments = vec![Comment::new(CACHE_WHY)];
        steps.push(read);
    }
    steps.push(bump(row, &token)?);
    steps.push(open(row, &token)?);
    Ok(steps)
}

fn bump(row: &Row, token: &Secret) -> Result<Step, KindError> {
    let script = plain(&row.args, "script")?;
    let mut step = run(row.line, BUMP, &[("GH_TOKEN", token.expr())])?;
    step.id = Some("bump".to_owned());
    step.env.insert("SCRIPT".to_owned(), Text::lit(script));
    Ok(step)
}

fn open(row: &Row, token: &Secret) -> Result<Step, KindError> {
    let prefix = plain(&row.args, "branch-prefix")?;
    let script = plain(&row.args, "script")?;
    let body = format!("Bumped by {script}, which ran before this PR.");
    let mut step = run(row.line, OPEN, &[("GH_TOKEN", token.expr())])?;
    step.condition = Expr::new(&GITHUB, &format!("{VERSION} != ''"));
    let mut put = |key: &str, text| step.env.insert(key.to_owned(), text);
    put("BRANCH", versioned(prefix));
    put("TITLE", versioned("build: bump to "));
    put("BODY", Text::lit(&body));
    Ok(step)
}

/// `lead` followed by the version the bump step printed.
fn versioned(lead: &str) -> Text {
    Expr::new(&GITHUB, VERSION)
        .into_iter()
        .fold(Text::lit(lead), Text::then_expr)
}

#[cfg(test)]
mod tests;
