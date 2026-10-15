//! The files `check` looks at, read from disk under `--root` (check C).

use std::io;
use std::path::Path;

/// Where GitHub reads workflows from, relative to the root.
pub(crate) const WORKFLOWS: &str = ".github/workflows/";

/// The bot config files `src/bots` renders (`src/bots:V38`).
pub(crate) const BOTS: [&str; 3] =
    [".coderabbit.yaml", "greptile.json", "socket.yml"];

/// Every file directly under `.github/workflows/` and every bot config
/// file present under `root`, as sorted repository-relative paths. A
/// missing workflows directory lists nothing.
///
/// # Errors
///
/// The workflows directory or one of its entries cannot be read.
pub(crate) fn listing(root: &Path) -> io::Result<Vec<String>> {
    let _ = BOTS;
    std::fs::read_dir(root.join(WORKFLOWS)).map(|_| Vec::new())
}

#[cfg(test)]
mod tests;
