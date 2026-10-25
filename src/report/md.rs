//! The report as Markdown tables, for a PR or an issue: workflows, then
//! their actions and secrets, then hand-written files and bots.

use super::model::{App, Manual, Need, Report, Workflow};
use crate::pin::Uses;

/// `report` as a Markdown page; `None.` for an empty table.
pub(crate) fn md(report: &Report) -> String {
    let w = &report.workflows;
    let sections = [
        table("Workflows", &WORKFLOWS, w.iter().map(workflow).collect()),
        table("Actions", &ACTIONS, w.iter().flat_map(actions).collect()),
        table("Secrets", &SECRETS, w.iter().flat_map(secrets).collect()),
        table("Hand-written", &MANUAL, each(&report.hand_written, manual)),
        table("Bots", &BOTS, each(&report.bots, app)),
    ];
    format!("# actlith report\n{}", sections.concat())
}

const WORKFLOWS: [&str; 4] = ["id", "kind", "systems", "cache"];
const ACTIONS: [&str; 4] = ["workflow", "action", "version", "sha"];
const SECRETS: [&str; 3] = ["workflow", "secret", "purpose"];
const MANUAL: [&str; 2] = ["path", "reason"];
const BOTS: [&str; 3] = ["bot", "app", "note"];

/// `raw` as a code span in a table cell; a backtick in it widens the
/// span to two.
pub(crate) fn code(raw: &str) -> String {
    if raw.contains('`') {
        format!("`` {raw} ``")
    } else {
        format!("`{raw}`")
    }
}

fn each<T>(items: &[T], row: fn(&T) -> Vec<String>) -> Vec<Vec<String>> {
    items.iter().map(row).collect()
}

fn table(heading: &str, header: &[&str], rows: Vec<Vec<String>>) -> String {
    if rows.is_empty() {
        return format!("\n## {heading}\n\nNone.\n");
    }
    let rule: Vec<&str> = header.iter().map(|_| "---").collect();
    let (header, rule) = (line(header), line(&rule));
    let body: String = rows.into_iter().map(|r| line(&r)).collect();
    format!("\n## {heading}\n\n{header}{rule}{body}")
}

/// One table row, `|` in a cell escaped.
fn line<S: AsRef<str>>(cells: &[S]) -> String {
    let cells: Vec<String> = cells
        .iter()
        .map(|c| c.as_ref().replace('|', "\\|"))
        .collect();
    format!("| {} |\n", cells.join(" | "))
}

fn workflow(w: &Workflow) -> Vec<String> {
    let xcode = w
        .xcode
        .as_deref()
        .map_or_else(String::new, |v| format!(" (Xcode {})", code(v)));
    let systems: Vec<String> = w
        .systems
        .iter()
        .map(|h| format!("{} on {}{xcode}", code(&h.system), code(&h.runner)))
        .collect();
    let cache = w.cache.as_deref().map_or_else(|| "-".to_owned(), code);
    vec![code(&w.id), code(w.kind), systems.join(", "), cache]
}

fn actions(w: &Workflow) -> Vec<Vec<String>> {
    let row = |u: &Uses| {
        vec![code(&w.id), code(&u.action), code(&u.version), code(&u.sha)]
    };
    w.actions.iter().map(row).collect()
}

fn secrets(w: &Workflow) -> Vec<Vec<String>> {
    let row = |n: &Need| vec![code(&w.id), code(&n.name), n.purpose.to_owned()];
    w.secrets.iter().map(row).collect()
}

fn manual(m: &Manual) -> Vec<String> {
    vec![code(&m.path), m.reason.clone()]
}

fn app(a: &App) -> Vec<String> {
    vec![code(&a.bot), a.name.to_owned(), a.note.to_owned()]
}

#[cfg(test)]
mod tests;
