//! One table row into cells: `|` splits, `\|` is a literal pipe, `-` is
//! an empty cell, `,` splits a list (registry C, `.:I` file).

use super::error::RegistryError;
use super::model::Arg;
use super::table::Table;

/// The fix hint for a malformed row.
const ROW_FIX: &str = "one cell per column, `-` if empty, `\\|` for a pipe";

/// The cells of a row, trimmed, `\|` unescaped, `-` made empty.
#[must_use]
pub(crate) fn split(row: &str) -> Vec<String> {
    let mut cells = Vec::new();
    let mut cell = String::new();
    let mut chars = row.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\\' if chars.next_if_eq(&'|').is_some() => cell.push('|'),
            '|' => cells.push(std::mem::take(&mut cell)),
            _ => cell.push(c),
        }
    }
    cells.push(cell);
    cells.iter().map(|c| tidy(c)).collect()
}

/// A raw cell trimmed, `-` made empty.
fn tidy(cell: &str) -> String {
    match cell.trim() {
        "-" => String::new(),
        c => c.to_owned(),
    }
}

/// Exactly the section's cell count, or the row is refused.
///
/// # Errors
///
/// The row has more or fewer cells than the section's header.
pub(crate) fn fixed<const N: usize>(
    line: usize,
    table: Table,
    cells: Vec<String>,
) -> Result<[String; N], RegistryError> {
    let count = cells.len();
    cells.try_into().map_err(|_| {
        let s = if count == 1 { "" } else { "s" };
        let what = format!(
            "{count} cell{s} where `## {}` rows have {N}",
            table.name()
        );
        RegistryError::new(line, what, format!("{}: {ROW_FIX}", table.header()))
    })
}

/// The header row is the section's own, cell by cell.
///
/// # Errors
///
/// Any other header.
pub(crate) fn header(
    line: usize,
    table: Table,
    row: &str,
) -> Result<(), RegistryError> {
    if split(row) == table.header().split('|').collect::<Vec<_>>() {
        return Ok(());
    }
    let what = format!("`## {}` header is `{}`", table.name(), row.trim());
    let fix = format!("start the section with `{}`", table.header());
    Err(RegistryError::new(line, what, fix))
}

/// A cell that may not be empty.
///
/// # Errors
///
/// The cell is empty (or `-`).
pub(crate) fn required(
    line: usize,
    column: &str,
    value: String,
) -> Result<String, RegistryError> {
    if value.is_empty() {
        let fix = format!("fill `{column}`; `-` is only for optional cells");
        return Err(RegistryError::new(
            line,
            format!("`{column}` is empty"),
            fix,
        ));
    }
    Ok(value)
}

/// A cell that may be empty: `None` then.
#[must_use]
pub(crate) fn optional(value: &str) -> Option<String> {
    (!value.is_empty()).then(|| value.to_owned())
}

/// A comma-separated list; an empty cell is an empty list.
///
/// # Errors
///
/// An empty item (`a,,b`).
pub(crate) fn list(
    line: usize,
    column: &str,
    cell: &str,
) -> Result<Vec<String>, RegistryError> {
    if cell.is_empty() {
        return Ok(Vec::new());
    }
    let items: Vec<String> =
        cell.split(',').map(|i| i.trim().to_owned()).collect();
    if items.iter().any(String::is_empty) {
        let what = format!("`{column}` has an empty item");
        let fix = "separate items with one `,`; `-` for none";
        return Err(RegistryError::new(line, what, fix));
    }
    Ok(items)
}

/// Comma-separated `key=value` pairs, each key once.
///
/// # Errors
///
/// An item that is not `key=value`, or a key given twice.
pub(crate) fn args(line: usize, cell: &str) -> Result<Vec<Arg>, RegistryError> {
    let mut out: Vec<Arg> = Vec::new();
    for item in list(line, "args", cell)? {
        let arg = pair(line, &item)?;
        if out.iter().any(|a| a.key == arg.key) {
            let what = format!("arg `{}` given twice", arg.key);
            return Err(RegistryError::new(line, what, "give each key once"));
        }
        out.push(arg);
    }
    Ok(out)
}

/// One `key=value` item, both sides non-empty.
fn pair(line: usize, item: &str) -> Result<Arg, RegistryError> {
    let kv = item.split_once('=').map(|(k, v)| (k.trim(), v.trim()));
    match kv {
        Some((key, value)) if !key.is_empty() && !value.is_empty() => Ok(Arg {
            key: key.to_owned(),
            value: value.to_owned(),
        }),
        _ => Err(RegistryError::new(
            line,
            format!("arg `{item}` is not `key=value`"),
            "write `key=value`; leave the key out instead of an empty value",
        )),
    }
}

#[cfg(test)]
mod tests;
