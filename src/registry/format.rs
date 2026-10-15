//! The `format N` line every `.actlith` starts with (registry V11).

use super::error::RegistryError;

/// The newest format this actlith reads.
pub(crate) const SUPPORTED: u32 = 1;

/// The first content line, `(line, text)`, is `format 1`.
///
/// # Errors
///
/// No format line, a malformed one, or a version newer than
/// [`SUPPORTED`], named next to it (registry V11).
pub(crate) fn check(first: Option<(usize, &str)>) -> Result<(), RegistryError> {
    let (line, text) = first.unwrap_or((1, ""));
    let mut words = text.split_whitespace();
    if words.next() != Some("format") {
        let fix = format!("start the file with `format {SUPPORTED}`");
        return Err(RegistryError::new(line, "no `format` line", fix));
    }
    version(line, &words.collect::<Vec<_>>().join(" "))
}

/// The version after `format` is the supported one.
fn version(line: usize, version: &str) -> Result<(), RegistryError> {
    match version.parse::<u32>() {
        Ok(SUPPORTED) => Ok(()),
        Ok(newer) if newer > SUPPORTED => Err(too_new(line, newer)),
        _ => {
            let what = format!("unknown format `{version}`");
            let fix = format!("write `format {SUPPORTED}`");
            Err(RegistryError::new(line, what, fix))
        }
    }
}

/// A newer format names both versions; no best-effort parse (V11).
fn too_new(line: usize, newer: u32) -> RegistryError {
    let what = format!(
        "`.actlith` is format {newer}; this actlith reads format {SUPPORTED}"
    );
    let fix = format!("upgrade actlith to one that reads format {newer}");
    RegistryError::new(line, what, fix)
}

#[cfg(test)]
mod tests;
