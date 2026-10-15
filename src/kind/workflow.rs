//! Workflow-level hardening every kind gets (V3): triggers that cannot
//! say `pull_request_target`, top `permissions: contents: read`, and the
//! `<id>-${{ github.ref }}` concurrency group.

use std::collections::BTreeMap;

use crate::yaml::{
    Concurrency, Env, Expr, Job, Level, Permissions, Push, Scope, Text,
    Trigger, Workflow,
};

/// Push to `main`, `pull_request` and `workflow_dispatch`: the push and
/// PR kinds (`gate`, `build`).
pub(crate) fn on_push_and_pr() -> Trigger {
    Trigger {
        push: Some(Push {
            branches: vec!["main".to_owned()],
        }),
        pull_request: true,
        schedule: Vec::new(),
        workflow_dispatch: true,
    }
}

/// `schedule` at `cron` and `workflow_dispatch`: the bump kinds.
pub(crate) fn on_schedule(cron: &str) -> Trigger {
    Trigger {
        schedule: vec![cron.to_owned()],
        workflow_dispatch: true,
        ..Trigger::default()
    }
}

/// `contents: read`, the top-level permissions of every workflow.
pub(crate) fn read_only() -> Permissions {
    Permissions(BTreeMap::from([(Scope::Contents, Level::Read)]))
}

/// Group `<id>-${{ github.ref }}`; in-progress runs are cancelled unless
/// the workflow is scheduled (a cancelled bump can leave a half-pushed
/// branch).
pub(crate) fn concurrency(id: &str, on: &Trigger) -> Concurrency {
    let group = Text::lit(&format!("{id}-"));
    Concurrency {
        group: Expr::new("github.ref")
            .into_iter()
            .fold(group, Text::then_expr),
        cancel_in_progress: on.schedule.is_empty(),
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
        concurrency: Some(concurrency(id, &on)),
        on,
        permissions: read_only(),
        env: Env::new(),
        jobs,
    }
}

#[cfg(test)]
mod tests;
