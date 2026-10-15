//! What both bump kinds share: the `token` arg naming the secret that
//! pushes and opens the PR (`src/kind:V14`, R5), the one runner the job
//! runs on, the checkout that keeps credentials because the job pushes
//! (`src/kind:V3`), and the job's `contents` and `pull-requests` writes.

use crate::kind::{
    ArgSpec, Args, Checkout, KindError, Row, Secret, checkout, job, timeout,
    widen,
};
use crate::pin::PinTable;
use crate::system::{System, runner};
use crate::yaml::{
    Action, Comment, Input, Job, Level, Scope, Step, Text, Value,
};

/// The secret a row means when it gives no `token`.
pub(crate) const GITHUB_TOKEN: &str = "GITHUB_TOKEN";

/// `token`, the secret that pushes the branch and opens the PR (R5).
pub(crate) const TOKEN: ArgSpec = ArgSpec {
    key: "token",
    required: false,
    default: Some(GITHUB_TOKEN),
};

/// A bump kind runs one job on one Linux runner.
pub(crate) const SYSTEMS: [System; 1] = [System::X86_64Linux];

/// What a bump kind reads its token for (`report`, V14).
const PURPOSE: &str = "push the bump branch and open its PR";

/// The R5 limit and V26, written at the top of every bump workflow.
pub(crate) const LIMITS: &str = "Known limit (R5): a PR opened with \
    GITHUB_TOKEN does not trigger other\nworkflows, so the gate does not \
    run on it until someone pushes or reruns\nit. args `token=<SECRET>` \
    names a GitHub App or fine-grained token\nsecret that lifts this.\n\n\
    Runs are never cancelled (V26): a cancelled run can leave a half-pushed\n\
    branch. A branch already on origin means its PR is open, so a run\n\
    exits 0 and opens no second one.";

const KEEP_WHY: &str = "Credentials kept: this job pushes the bump branch.";

/// The secret the row's `token` names; refused unless it is a secret
/// name.
pub(crate) fn token(args: &Args) -> Result<Secret, KindError> {
    let name = args.get(TOKEN.key).unwrap_or(GITHUB_TOKEN);
    Secret::new(name, PURPOSE).ok_or_else(|| {
        KindError::row(
            args.line(),
            format!("token `{}` is not a secret name", name.escape_debug()),
            "write `token=<SECRET>`: letters, digits and `_`",
        )
    })
}

/// The runner label of the row's one system.
pub(crate) fn runs_on(row: &Row) -> Result<Text, KindError> {
    let [system] = row.systems.as_slice() else {
        let count = row.systems.len();
        return Err(KindError::row(
            row.line,
            format!("a bump kind runs one job, {count} systems given"),
            "list one system, or `-` for x86_64-linux",
        ));
    };
    runner(*system).map(Text::lit).map_err(|e| {
        KindError::row(row.line, e.to_string(), "write `-` in systems")
    })
}

/// The checkout of a job that pushes: credentials kept, and checked out
/// with `token` unless it is `GITHUB_TOKEN`, so the push is made by the
/// token that can trigger the PR's workflows (R5).
pub(crate) fn pushing_checkout(
    pins: &PinTable,
    token: &Secret,
) -> Result<Step, KindError> {
    let how = Checkout {
        pushes: true,
        ..Checkout::default()
    };
    let mut step = checkout(pins, &how)?;
    step.comments = vec![Comment::new(KEEP_WHY)];
    if let Action::Uses(uses) = &mut step.action
        && token.name() != GITHUB_TOKEN
    {
        let value = Value::Text(Text::expr(token.expr()));
        uses.with =
            std::mem::take(&mut uses.with).set("token", Input::new(value));
    }
    Ok(step)
}

/// The one bump job: on the row's runner, timed out, writing contents
/// (the branch) and pull requests (the PR) only.
pub(crate) fn bump_job(row: &Row, steps: Vec<Step>) -> Result<Job, KindError> {
    let mut bump = job(runs_on(row)?, timeout(&row.args)?, steps);
    bump.permissions = Some(widen(&[
        (Scope::Contents, Level::Write),
        (Scope::PullRequests, Level::Write),
    ]));
    Ok(bump)
}

/// The row's `key` as a plain word (a path, flake attribute or branch
/// prefix): ASCII letters, digits and `._/#+:@-`, not starting with `-`.
/// It reaches the shell through `env:` only; the check keeps it readable
/// and refuses what no path or ref is.
pub(crate) fn plain<'a>(
    args: &'a Args,
    key: &str,
) -> Result<&'a str, KindError> {
    let value = args.get(key).unwrap_or_default();
    let word = |c: char| c.is_ascii_alphanumeric() || "._/#+:@-".contains(c);
    if !value.is_empty() && !value.starts_with('-') && value.chars().all(word) {
        return Ok(value);
    }
    Err(KindError::row(
        args.line(),
        format!(
            "arg `{key}` is `{}`, not a plain word",
            value.escape_debug()
        ),
        format!("write `{key}=` with letters, digits and `._/#+:@-`"),
    ))
}

#[cfg(test)]
mod tests;
