//! `import` (T26, V17): a workflow text becomes a `## workflows` row
//! only when the render of that row, normalised, equals the text,
//! normalised. Every candidate is rendered by `src/render` itself; no
//! YAML is read (`.:C`).

use super::candidates::candidates;
use super::mismatch::{Mismatch, common};
use super::normalise::{Line, normalise};
use crate::registry::{WorkflowRow, parse};
use crate::render::render;

/// The one-row `.actlith` a candidate row is parsed from.
const HEAD: &str = "format 1\n\n## workflows\nid|kind|systems|cache|args\n";

/// A candidate row and its render, normalised.
struct Rendered {
    row: WorkflowRow,
    lines: Vec<Line>,
}

/// The row whose render is `text`, after normalising both.
///
/// # Errors
///
/// No candidate renders to `text`: the first line and key where it
/// parts from the closest render. Never a partial row.
pub(crate) fn import(text: &str) -> Result<WorkflowRow, Mismatch> {
    let lines = normalise(text);
    let mut closest: Option<(usize, Rendered)> = None;
    for render in candidates(&lines).iter().filter_map(|c| rendered(c)) {
        let same = common(&lines, &render.lines);
        if same == lines.len() && same == render.lines.len() {
            return Ok(render.row);
        }
        if closest.as_ref().is_none_or(|(best, _)| same > *best) {
            closest = Some((same, render));
        }
    }
    Err(closest.map_or_else(
        || Mismatch::unnamed(&lines),
        |(_, r)| Mismatch::at(&lines, &r.lines, &r.row.kind),
    ))
}

/// `cells` parsed as a row and rendered; `None` when the registry or
/// the kind refuses it.
fn rendered(cells: &str) -> Option<Rendered> {
    let registry = parse(&format!("{HEAD}{cells}\n")).ok()?;
    let row = registry.workflows.first()?.clone();
    let path = format!(".github/workflows/{}.yml", row.id);
    let file = render(&registry)
        .ok()?
        .into_iter()
        .find(|f| f.path == path)?;
    Some(Rendered {
        lines: normalise(&file.bytes),
        row,
    })
}

#[cfg(test)]
mod tests;
