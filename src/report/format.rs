//! The formats `report` prints (C, `.:I` `--format`).

/// How the report is printed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Format {
    /// Indented plain text, for a terminal.
    Text,
    /// JSON with `schema_version` (V33), for tools.
    Json,
    /// Markdown tables, for a PR or an issue.
    Md,
}
