//! Generated blocks inside hand-written files, between
//! `<!-- BEGIN name ... -->` and `<!-- END name -->`. RED: signatures only.

/// `doc` with block `name` replaced by `block` (markers included); `None`
/// when `doc` has no such block.
#[must_use]
pub(crate) fn splice(_doc: &str, _name: &str, _block: &str) -> Option<String> {
    None
}

#[cfg(test)]
mod tests;
