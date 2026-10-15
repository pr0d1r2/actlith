//! `render` (T38, V31): every `## workflows` row checked against its v1
//! kind and built into `.github/workflows/<id>.yml`, every seeded script
//! a rendered kind needs once, every `## bots` file; sorted bytewise by
//! path (C). Pure: no I/O (`.:V1`).

use super::error::RenderError;
use crate::bots::File;
use crate::registry::Registry;

/// Every file `.actlith` asks for, sorted bytewise by path.
///
/// # Errors
///
/// The first row naming no v1 kind or one its kind refuses (naming the
/// row's line), a refused pin or bot row, or two files at one path.
pub(crate) fn render(_registry: &Registry) -> Result<Vec<File>, RenderError> {
    Err(RenderError::Duplicate(String::new()))
}

#[cfg(test)]
mod tests;
