//! The report as indented plain text, for a terminal: a heading per
//! section, an entry per row two spaces in, its details four spaces in.

use super::model::{App, Manual, Report, Workflow};

/// `report` as text: `workflows`, `hand-written`, `bots`; `-` for an
/// empty section.
pub(crate) fn text(report: &Report) -> String {
    let mut out = String::new();
    section(&mut out, "workflows", report.workflows.iter().map(workflow));
    section(
        &mut out,
        "hand-written",
        report.hand_written.iter().map(manual),
    );
    section(&mut out, "bots", report.bots.iter().map(app));
    out
}

fn section(
    out: &mut String,
    heading: &str,
    entries: impl Iterator<Item = String>,
) {
    out.push_str(heading);
    out.push('\n');
    let before = out.len();
    entries.for_each(|e| out.push_str(&e));
    if out.len() == before {
        out.push_str("  -\n");
    }
}

fn workflow(w: &Workflow) -> String {
    let cache = w.cache.as_deref().unwrap_or("-");
    let head = format!("  {}: kind {}, cache {cache}\n", w.id, w.kind);
    let systems = w.systems.iter().map(|h| (h.system.clone(), h.runner));
    let systems = systems.map(|(s, r)| format!("    system {s} on {r}\n"));
    let actions = w
        .actions
        .iter()
        .map(|u| format!("    action {} {}\n", u.reference(), u.version));
    let secrets = w
        .secrets
        .iter()
        .map(|n| format!("    secret {}: {}\n", n.name, n.purpose));
    let lines = systems.chain(actions).chain(secrets);
    std::iter::once(head).chain(lines).collect()
}

fn manual(m: &Manual) -> String {
    format!("  {}: {}\n", m.path, m.reason)
}

fn app(a: &App) -> String {
    format!("  {}: {}; {}\n", a.bot, a.name, a.note)
}

#[cfg(test)]
mod tests;
