//! The files `check` looks at, read from disk under `--root` (check C).

use std::fs;
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
    let mut paths = workflows(root)?;
    let bots = BOTS.iter().filter(|bot| root.join(bot).is_file());
    paths.extend(bots.map(|bot| (*bot).to_owned()));
    paths.sort();
    Ok(paths)
}

/// The files directly under `root/.github/workflows/`; none when the
/// directory does not exist.
fn workflows(root: &Path) -> io::Result<Vec<String>> {
    let entries = match fs::read_dir(root.join(WORKFLOWS)) {
        Err(err) if err.kind() == io::ErrorKind::NotFound => {
            return Ok(Vec::new());
        }
        other => other?,
    };
    let mut paths = Vec::new();
    for entry in entries {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            let name = entry.file_name();
            paths.push(format!("{WORKFLOWS}{}", name.to_string_lossy()));
        }
    }
    Ok(paths)
}

#[cfg(test)]
mod tests;
