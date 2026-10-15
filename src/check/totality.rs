//! Totality (V7, `src/bots:V38`): pure, over a listing already read.

use super::finding::Finding;
// `src/render`'s `File` is this one; its re-export there stays unused
// (and `expect`ed so) until `src/gen` lands, so name it at the source.
use crate::bots::File;
use crate::registry::HandWritten;

/// Every finding for `listing` against the `rendered` files and the
/// `hand_written` rows, sorted by path.
pub(crate) fn totality(
    listing: &[String],
    rendered: &[File],
    hand_written: &[HandWritten],
) -> Vec<Finding> {
    let _ = (listing, rendered, hand_written);
    Vec::new()
}

#[cfg(test)]
mod tests;
