//! The YAML tree the model lowers to, and the writer that prints it (V5):
//! two-space indentation, block style, `[a, b]` lists, comments on their
//! own lines or two spaces after a value, blank lines only between
//! top-level keys.

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
/// RED: signature only.
pub(crate) fn document(_comments: &[Comment], _entries: &[Entry]) -> String {
    String::new()
}

#[cfg(test)]
mod tests;
