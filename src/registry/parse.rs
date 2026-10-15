//! The whole file: `format` line, then `## <section>` tables, `#`
//! comments and blank lines skipped (registry C, V11, V12).

use super::error::RegistryError;
use super::format;
use super::model::Registry;
use super::rows::{add, unique};
use super::table::Table;

/// A `## <section>` heading and the content lines under it.
struct Section<'a> {
    /// The heading's line.
    line: usize,
    /// The name after `##`.
    name: &'a str,
    /// `(line, text)` of each row, header first.
    rows: Vec<(usize, &'a str)>,
}

/// Parse `.actlith` text. Pure: the caller reads the file (registry C).
///
/// # Errors
///
/// The first problem found, with its line and a fix: a missing or newer
/// `format`, an unknown or repeated section, a bad header, a malformed
/// row, or a key given twice.
pub(crate) fn parse(text: &str) -> Result<Registry, RegistryError> {
    let mut lines = content(text);
    format::check(lines.next())?;
    let mut reg = Registry::default();
    let mut seen: Vec<(Table, usize)> = Vec::new();
    for section in sections(lines)? {
        let table = known(&section, &seen)?;
        seen.push((table, section.line));
        fill(&mut reg, table, &section.rows)?;
    }
    unique(&reg)?;
    Ok(reg)
}

/// `(line, text)` of each line that is not blank or a `#` comment.
fn content(text: &str) -> impl Iterator<Item = (usize, &str)> {
    let comment = |l: &str| l.starts_with('#') && !l.starts_with("##");
    (1..)
        .zip(text.lines())
        .map(|(no, l)| (no, l.trim()))
        .filter(move |(_, l)| !l.is_empty() && !comment(l))
}

/// Group content lines under their `##` heading.
fn sections<'a>(
    lines: impl Iterator<Item = (usize, &'a str)>,
) -> Result<Vec<Section<'a>>, RegistryError> {
    let mut out: Vec<Section<'a>> = Vec::new();
    for (line, text) in lines {
        if let Some(name) = text.strip_prefix("##") {
            out.push(Section::new(line, name.trim()));
        } else if let Some(last) = out.last_mut() {
            last.rows.push((line, text));
        } else {
            return Err(outside(line, text));
        }
    }
    Ok(out)
}

impl<'a> Section<'a> {
    /// A heading with no rows yet.
    fn new(line: usize, name: &'a str) -> Self {
        let rows = Vec::new();
        Self { line, name, rows }
    }
}

/// A row above the first heading.
fn outside(line: usize, text: &str) -> RegistryError {
    let what = format!("row `{text}` is outside any section");
    let fix = "put it under a `## <section>` heading";
    RegistryError::new(line, what, fix)
}

/// The section a heading names, given once.
fn known(
    section: &Section<'_>,
    seen: &[(Table, usize)],
) -> Result<Table, RegistryError> {
    let line = section.line;
    let Some(table) = Table::named(section.name) else {
        let what = format!("unknown section `## {}`", section.name);
        let fix = format!("sections are {}", Table::names());
        return Err(RegistryError::new(line, what, fix));
    };
    match seen.iter().find(|(t, _)| *t == table) {
        None => Ok(table),
        Some((_, first)) => Err(twice(line, table, *first)),
    }
}

/// A section heading given a second time.
fn twice(line: usize, table: Table, first: usize) -> RegistryError {
    let name = table.name();
    let what = format!("`## {name}` given twice, first on line {first}");
    RegistryError::new(line, what, "merge them into one section")
}

/// Check the header row, then type each data row into `reg`.
fn fill(
    reg: &mut Registry,
    table: Table,
    rows: &[(usize, &str)],
) -> Result<(), RegistryError> {
    let Some(((line, head), body)) = rows.split_first() else {
        return Ok(());
    };
    super::cells::header(*line, table, head)?;
    for (line, row) in body {
        add(reg, table, *line, row)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests;
