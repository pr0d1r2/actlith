//! A workflow text normalised for comparison (V17): comment lines,
//! trailing comments (the `# vX` pin comment among them), trailing
//! whitespace, blank lines and the `---` marker dropped; a `>-` folded
//! block joined onto its key's line. Block scalar content is kept as
//! written, `#` lines included: there a `#` is content, not a comment.

/// One kept line of a workflow text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Line {
    /// The 1-based line it starts on in the text.
    pub(crate) number: usize,
    /// The normalised text, indentation kept.
    pub(crate) text: String,
    /// Content of a literal block scalar: no key on it.
    pub(crate) block: bool,
}

/// An open block scalar: the column of its key, and how it reads.
#[derive(Clone, Copy, Debug)]
struct Open {
    column: usize,
    folded: bool,
}

/// `text` normalised, line by line.
pub(crate) fn normalise(text: &str) -> Vec<Line> {
    let mut out = Vec::new();
    let mut open: Option<Open> = None;
    for (number, raw) in (1..).zip(text.lines()) {
        let raw = raw.trim_end();
        match open.filter(|o| inside(*o, raw)) {
            Some(o) => content(&mut out, o, number, raw),
            None => open = plain(&mut out, number, raw),
        }
    }
    out
}

/// Whether `raw` belongs to the block `open`: blank or indented past
/// its key.
fn inside(open: Open, raw: &str) -> bool {
    raw.is_empty() || indent(raw) > open.column
}

/// A line of block content: a folded line joins its key's line, a
/// literal one is kept; blank lines go.
fn content(out: &mut Vec<Line>, open: Open, number: usize, raw: &str) {
    if raw.is_empty() {
        return;
    }
    match out.last_mut() {
        Some(last) if open.folded => {
            last.text.push(' ');
            last.text.push_str(raw.trim_start());
        }
        _ => out.push(Line {
            number,
            text: raw.to_owned(),
            block: true,
        }),
    }
}

/// A line outside any block: dropped when it is blank, a comment or
/// `---`, else kept without its trailing comment. The block it opens,
/// if any.
fn plain(out: &mut Vec<Line>, number: usize, raw: &str) -> Option<Open> {
    let trimmed = raw.trim_start();
    if trimmed.is_empty() || trimmed.starts_with('#') || raw == "---" {
        return None;
    }
    let kept = strip_comment(raw).trim_end();
    let (text, open) = header(kept);
    out.push(Line {
        number,
        text: text.to_owned(),
        block: false,
    });
    open
}

/// `kept` without a block scalar indicator, and the block it opens. A
/// `>-` header loses its indicator: its lines join it.
fn header(kept: &str) -> (&str, Option<Open>) {
    let Some((head, indicator)) = kept.rsplit_once(' ') else {
        return (kept, None);
    };
    let keyed = head.ends_with(':');
    if !keyed || !["|", "|-", "|+", ">", ">-", ">+"].contains(&indicator) {
        return (kept, None);
    }
    let folded = indicator == ">-";
    let open = Open {
        column: column(kept),
        folded,
    };
    (if folded { head } else { kept }, Some(open))
}

/// Leading spaces of `raw`.
fn indent(raw: &str) -> usize {
    raw.len().saturating_sub(raw.trim_start_matches(' ').len())
}

/// The column of the key on `raw`: past a `- ` sequence marker.
pub(crate) fn column(raw: &str) -> usize {
    let start = indent(raw);
    let rest = raw.get(start..).unwrap_or_default();
    if rest.starts_with("- ") {
        start.saturating_add(2)
    } else {
        start
    }
}

/// `raw` up to a ` #` comment outside quotes.
fn strip_comment(raw: &str) -> &str {
    let mut quote: Option<char> = None;
    let mut previous = ' ';
    for (at, c) in raw.char_indices() {
        quote = match (quote, c) {
            (None, '#') if previous == ' ' => {
                return raw.get(..at).unwrap_or(raw);
            }
            (None, '\'' | '"') => Some(c),
            (Some(q), _) if q == c && !escaped(q, previous) => None,
            (state, _) => state,
        };
        previous = c;
    }
    raw
}

/// Whether a `"` follows a backslash: escaped, still quoted.
fn escaped(quote: char, previous: char) -> bool {
    quote == '"' && previous == '\\'
}

#[cfg(test)]
mod tests;
