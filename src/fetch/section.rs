//! The `## actions` section of `.actlith` text: its rows, and the text
//! with only that section replaced (V16).

use crate::pin::Uses;

/// The `## actions` header row (`.:I` file).
pub(crate) const HEADER: &str = "action|version|sha";

/// The header row and one `action|version|sha` row per pin, each line
/// ending in a newline.
pub(crate) fn rows(pins: &[Uses]) -> String {
    let row = |p: &Uses| format!("{}|{}|{}\n", p.action, p.version, p.sha);
    let body: String = pins.iter().map(row).collect();
    format!("{HEADER}\n{body}")
}

/// `text` with the rows of `## actions` replaced by `rows`; comments in
/// the section and every other line kept. No section -> appended.
pub(crate) fn replace(text: &str, rows: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    match lines.iter().position(|l| is_actions(l)) {
        Some(at) => splice(&lines, at, rows),
        None => format!("{}\n\n## actions\n{rows}", text.trim_end()),
    }
}

/// The lines up to heading `at`, the section's leading comments, `rows`,
/// the section's trailing comments and blanks, then the rest.
fn splice(lines: &[&str], at: usize, rows: &str) -> String {
    let (head, rest) = lines.split_at(at.saturating_add(1));
    let next = rest
        .iter()
        .position(|l| is_heading(l))
        .unwrap_or(rest.len());
    let (body, tail) = rest.split_at(next);
    let (lead, after) = around_rows(body);
    let parts = [joined(head), joined(lead), rows.to_owned()];
    [parts.concat(), joined(after), joined(tail)].concat()
}

/// Each line followed by a newline.
fn joined(lines: &[&str]) -> String {
    lines.iter().fold(String::new(), |mut out, line| {
        out.push_str(line);
        out.push('\n');
        out
    })
}

/// The lines of `body` before its first row (trailing blanks dropped) and
/// after its last row (no rows: the trailing blanks).
fn around_rows<'a>(body: &'a [&'a str]) -> (&'a [&'a str], &'a [&'a str]) {
    let first = body.iter().position(|l| is_row(l)).unwrap_or(body.len());
    let (before, _) = body.split_at(first);
    let keep = before.iter().rposition(|l| !l.trim().is_empty());
    let (lead, _) = before.split_at(keep.map_or(0, |i| i.saturating_add(1)));
    let last = body.iter().rposition(|l| is_row(l));
    let (_, trail) =
        body.split_at(last.map_or(lead.len(), |i| i.saturating_add(1)));
    (lead, trail)
}

/// `## actions`, spaces allowed around the name.
fn is_actions(line: &str) -> bool {
    let name = line.trim().strip_prefix("##").map(str::trim);
    name == Some("actions")
}

/// Any `## <section>` heading.
fn is_heading(line: &str) -> bool {
    line.trim().starts_with("##")
}

/// A header or data row: not blank, not a `#` comment.
fn is_row(line: &str) -> bool {
    let line = line.trim();
    !line.is_empty() && !line.starts_with('#')
}

#[cfg(test)]
mod tests;
