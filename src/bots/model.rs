//! A rendered bot config file (`src/bots` I). Shape only.

/// One file to write: its repository-relative path and its contents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct File {
    /// Where the bot reads it, relative to the repository root.
    pub(crate) path: String,
    /// The whole file, ending in a newline.
    pub(crate) bytes: String,
}
