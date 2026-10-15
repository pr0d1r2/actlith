//! The YAML tree the model lowers to, and the writer that prints it (V5):
//! two-space indentation, block style, `[a, b]` lists, comments on their
//! own lines or two spaces after a value, blank lines only between
//! top-level keys.

use super::scalar::{self, WIDTH};
use super::text::{Comment, Text};

/// A scalar value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Value {
    /// A string, styled by `scalar`.
    Text(Text),
    /// A whole number.
    Int(u64),
    /// `true` or `false`.
    Bool(bool),
}

/// A mapping key.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Key {
    /// A key of the workflow schema, written as it is.
    Fixed(&'static str),
    /// A key from the caller (an input, a variable, a job id), quoted when
    /// it could not be read back plain.
    User(String),
}

/// A node of the tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Node {
    /// A scalar.
    Value(Value),
    /// A list of strings, `[a, b]` when it fits on its line.
    List(Vec<Text>),
    /// A mapping, in order.
    Map(Vec<Entry>),
    /// A block sequence of mappings.
    Seq(Vec<Vec<Entry>>),
    /// No value: `key:`.
    Null,
}

/// A mapping entry with the comments above it and after it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Entry {
    pub(crate) comments: Vec<Comment>,
    pub(crate) key: Key,
    pub(crate) node: Node,
    pub(crate) trailing: Option<Comment>,
}

impl Entry {
    /// An entry with a schema key and no comments.
    pub(crate) fn fixed(key: &'static str, node: Node) -> Self {
        Self::new(Key::Fixed(key), node)
    }

    /// An entry with a caller's key and no comments.
    pub(crate) fn user(key: &str, node: Node) -> Self {
        Self::new(Key::User(key.to_owned()), node)
    }

    fn new(key: Key, node: Node) -> Self {
        Self {
            comments: Vec::new(),
            key,
            node,
            trailing: None,
        }
    }
}

/// The document: `---`, the leading comments, then the top-level entries
/// with a blank line between them.
pub(crate) fn document(comments: &[Comment], entries: &[Entry]) -> String {
    let mut out = String::from("---\n");
    remarks(&mut out, comments, 0);
    for (i, entry) in entries.iter().enumerate() {
        if i > 0 {
            out.push('\n');
        }
        member(&mut out, entry, 0);
    }
    out
}

fn remarks(out: &mut String, comments: &[Comment], indent: usize) {
    for line in comments.iter().flat_map(Comment::lines) {
        push(out, &format!("{}{line}", pad(indent)), None);
    }
}

fn member(out: &mut String, entry: &Entry, indent: usize) {
    remarks(out, &entry.comments, indent);
    write_entry(out, entry, &pad(indent), indent);
}

fn map(out: &mut String, entries: &[Entry], indent: usize) {
    for entry in entries {
        member(out, entry, indent);
    }
}

fn item(out: &mut String, entries: &[Entry], indent: usize) {
    let Some((first, rest)) = entries.split_first() else {
        return push(out, &format!("{}- {{}}", pad(indent)), None);
    };
    remarks(out, &first.comments, indent);
    let inner = indent.saturating_add(2);
    write_entry(out, first, &format!("{}- ", pad(indent)), inner);
    map(out, rest, inner);
}

/// The start of an entry's first line, the comment that ends it, and the
/// column its block lines (nested keys, items, block scalars) start at.
struct Head<'a> {
    text: String,
    tail: Option<&'a Comment>,
    indent: usize,
}

impl Head<'_> {
    /// The first line with `rest` after the head, then the tail.
    fn push(&self, out: &mut String, rest: &str) {
        push(out, &format!("{}{rest}", self.text), self.tail);
    }

    fn fits(&self, rest: &str) -> bool {
        let extra = self.tail.map_or(0, |c| c.inline().len().saturating_add(2));
        let len = self
            .text
            .chars()
            .count()
            .saturating_add(rest.chars().count());
        len.saturating_add(extra) <= WIDTH
    }
}

/// `entry` after `lead` (its indentation, and `- ` for a sequence item),
/// its key at column `indent`.
fn write_entry(out: &mut String, entry: &Entry, lead: &str, indent: usize) {
    let head = Head {
        text: format!("{lead}{}:", key(&entry.key)),
        tail: entry.trailing.as_ref(),
        indent: indent.saturating_add(2),
    };
    match &entry.node {
        Node::Value(v) => value(out, &head, v),
        Node::List(items) => list(out, &head, items),
        Node::Map(m) => nested(out, &head, m),
        Node::Seq(items) => seq(out, &head, items),
        Node::Null => head.push(out, ""),
    }
}

fn key(key: &Key) -> String {
    match key {
        Key::Fixed(k) => (*k).to_owned(),
        Key::User(k) => scalar::key(k),
    }
}

/// A nested mapping; `{}` when empty.
fn nested(out: &mut String, head: &Head, entries: &[Entry]) {
    if entries.is_empty() {
        return head.push(out, " {}");
    }
    head.push(out, "");
    map(out, entries, head.indent);
}

/// A block sequence; `[]` when empty.
fn seq(out: &mut String, head: &Head, items: &[Vec<Entry>]) {
    if items.is_empty() {
        return head.push(out, " []");
    }
    head.push(out, "");
    for entries in items {
        item(out, entries, head.indent);
    }
}

/// `[a, b]` when it fits on the line, else one `- item` per line.
fn list(out: &mut String, head: &Head, items: &[Text]) {
    let flow: Vec<String> =
        items.iter().map(|t| scalar::inline(t, true)).collect();
    let one = format!(" [{}]", flow.join(", "));
    if head.fits(&one) {
        return head.push(out, &one);
    }
    head.push(out, "");
    for text in items {
        let line =
            format!("{}- {}", pad(head.indent), scalar::inline(text, false));
        push(out, &line, None);
    }
}

fn value(out: &mut String, head: &Head, value: &Value) {
    match value {
        Value::Int(n) => head.push(out, &format!(" {n}")),
        Value::Bool(b) => head.push(out, &format!(" {b}")),
        Value::Text(t) => text(out, head, t),
    }
}

/// A string: a literal block for several lines, one line when it fits,
/// else a folded block, else one long line.
fn text(out: &mut String, head: &Head, text: &Text) {
    let raw = text.render();
    if let Some((style, lines)) = scalar::literal(&raw) {
        head.push(out, &format!(" {style}"));
        return block(out, &lines, head.indent);
    }
    let one = format!(" {}", scalar::inline(text, false));
    match scalar::fold(&raw, head.indent) {
        Some(lines) if !head.fits(&one) => {
            head.push(out, " >-");
            block(out, &lines, head.indent);
        }
        _ => head.push(out, &one),
    }
}

/// The lines of a block scalar at `indent`; an empty line stays bare.
fn block<S: AsRef<str>>(out: &mut String, lines: &[S], indent: usize) {
    for line in lines.iter().map(AsRef::as_ref) {
        if line.is_empty() {
            out.push('\n');
        } else {
            push(out, &format!("{}{line}", pad(indent)), None);
        }
    }
}

/// One line: `line`, then `tail` two spaces after it.
fn push(out: &mut String, line: &str, tail: Option<&Comment>) {
    out.push_str(line);
    if let Some(comment) = tail {
        out.push_str("  ");
        out.push_str(&comment.inline());
    }
    out.push('\n');
}

fn pad(indent: usize) -> String {
    " ".repeat(indent)
}

#[cfg(test)]
mod tests;
