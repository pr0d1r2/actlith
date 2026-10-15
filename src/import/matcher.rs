//! `import` (T26, V17). RED: signature only.

use super::mismatch::Mismatch;
use crate::registry::WorkflowRow;

/// The row whose render is `text`, after normalising both.
///
/// # Errors
///
/// No candidate renders to `text`.
pub(crate) fn import(_text: &str) -> Result<WorkflowRow, Mismatch> {
    Err(Mismatch::default())
}

#[cfg(test)]
mod tests;
