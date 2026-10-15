//! The seeded scripts, embedded from the real files beside this module
//! (src/script C) and prefixed with the generated header (`src/yaml:V8`).

/// Every seeded script: published path `scripts/actlith/<name>.sh` and
/// its bytes, header first, sorted by path.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "used by src/render and the kind nodes in a later wave"
    )
)]
pub(crate) fn scripts() -> &'static [(&'static str, &'static str)] {
    &[]
}

#[cfg(test)]
mod tests;
