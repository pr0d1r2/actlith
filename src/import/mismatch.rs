//! Why a workflow text is not imported (V17): the first line where it
//! parts from the closest kind render, and the YAML key on that line.

use std::fmt;

use super::normalise::{Line, column};

/// A refused import: where the text and the closest render part.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Mismatch {
    /// The 1-based text line; one past the end when the text stops
    /// short.
    pub(crate) line: usize,
    /// Dotted path of the YAML key that differs, e.g.
    /// `jobs.gate.strategy.fail-fast`.
    pub(crate) key: String,
    /// The kind of the closest render; `None` when no candidate row
    /// rendered.
    pub(crate) kind: Option<String>,
    /// The render's line there, normalised; `None` past its end.
    pub(crate) expected: Option<String>,
    /// The text's line there, normalised; `None` past its end.
    pub(crate) found: Option<String>,
}

impl Mismatch {
    /// Where `text` parts from `render` of `kind`, at kept line `at`.
    pub(crate) fn at(text: &[Line], render: &[Line], kind: &str) -> Self {
        let at = common(text, render);
        let found = text.get(at);
        let key = found.map_or_else(|| path(render, at), |_| path(text, at));
        Self {
            line: found.map_or_else(|| end(text), |l| l.number),
            key,
            kind: Some(kind.to_owned()),
            expected: render.get(at).map(|l| l.text.trim().to_owned()),
            found: found.map(|l| l.text.trim().to_owned()),
        }
    }

    /// No candidate rendered: the text has no `name:` a row id can be.
    pub(crate) fn unnamed(text: &[Line]) -> Self {
        let name = text.iter().find(|l| l.text.starts_with("name:"));
        Self {
            line: name.or(text.first()).map_or(1, |l| l.number),
            key: "name".to_owned(),
            kind: None,
            expected: Some("name: <id>".to_owned()),
            found: name.map(|l| l.text.clone()),
        }
    }
}

/// How many leading kept lines `a` and `b` share.
pub(crate) fn common(a: &[Line], b: &[Line]) -> usize {
    a.iter()
        .zip(b)
        .take_while(|(x, y)| x.text == y.text)
        .count()
}

/// One past the last line of `text`.
fn end(text: &[Line]) -> usize {
    text.last().map_or(1, |l| l.number.saturating_add(1))
}

/// The dotted key path of kept line `at`: its key and each enclosing
/// key, found by indentation.
pub(crate) fn path(lines: &[Line], at: usize) -> String {
    let mut keys = Vec::new();
    let mut under = usize::MAX;
    for line in lines.iter().take(at.saturating_add(1)).rev() {
        let col = column(&line.text);
        if let Some(key) = key(line).filter(|_| col < under) {
            keys.push(key);
            under = col;
        }
    }
    keys.reverse();
    keys.join(".")
}

/// The key a line sets: the text before its first `: ` (or a final
/// `:`), past a `- ` marker. Block content sets none.
fn key(line: &Line) -> Option<&str> {
    let text = line.text.trim_start();
    let text = text.strip_prefix("- ").unwrap_or(text);
    let key = text
        .split_once(": ")
        .map_or_else(|| text.strip_suffix(':'), |(k, _)| Some(k));
    key.filter(|_| !line.block)
}

impl fmt::Display for Mismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let closest = self.kind.as_ref().map_or_else(
            || "no kind renders".to_owned(),
            |k| format!("closest kind `{k}`"),
        );
        write!(
            f,
            "line {}: `{}` differs ({closest}): expected {}, found {}\n  \
             fix: list the file under `## hand-written`",
            self.line,
            self.key,
            shown(self.expected.as_deref()),
            shown(self.found.as_deref()),
        )
    }
}

/// A line in backticks, or `nothing`.
fn shown(line: Option<&str>) -> String {
    line.map_or_else(|| "nothing".to_owned(), |t| format!("`{t}`"))
}

impl std::error::Error for Mismatch {}

#[cfg(test)]
mod tests;
