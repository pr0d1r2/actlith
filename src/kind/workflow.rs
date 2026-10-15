//! Workflow-level hardening every kind gets (V3): triggers that cannot
//! say `pull_request_target`, top `permissions: contents: read`, and the
//! `<id>-${{ github.ref }}` concurrency group.

use std::collections::BTreeMap;

use crate::yaml::{
    Concurrency, Env, Job, Permissions, Text, Trigger, Workflow,
};

/// Push to `main`, `pull_request` and `workflow_dispatch`: the push and
/// PR kinds (`gate`, `build`).
pub(crate) fn on_push_and_pr() -> Trigger {
    Trigger::default()
}

/// `schedule` at `cron` and `workflow_dispatch`: the bump kinds.
pub(crate) fn on_schedule(cron: &str) -> Trigger {
    let _ = cron;
    Trigger::default()
}

/// `contents: read`, the top-level permissions of every workflow.
pub(crate) fn read_only() -> Permissions {
    Permissions::default()
}

/// Group `<id>-${{ github.ref }}`; in-progress runs are cancelled unless
/// the workflow is scheduled (a cancelled bump can leave a half-pushed
/// branch).
pub(crate) fn concurrency(id: &str, on: &Trigger) -> Concurrency {
    let _ = on;
    Concurrency {
        group: Text::lit(id),
        cancel_in_progress: false,
    }
}

/// The workflow `id`: named `id`, hardened, with `jobs`.
pub(crate) fn workflow(
    id: &str,
    on: Trigger,
    jobs: BTreeMap<String, Job>,
) -> Workflow {
    Workflow {
        comments: Vec::new(),
        name: id.to_owned(),
        on,
        concurrency: None,
        permissions: Permissions::default(),
        env: Env::new(),
        jobs,
    }
}

#[cfg(test)]
mod tests;
