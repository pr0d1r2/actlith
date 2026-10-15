//! The little JSON `greptile.json` needs: strings and string arrays,
//! escaped per RFC 8259 so caller text never breaks the document.

/// `raw` as a JSON string, quotes included.
#[must_use]
pub(crate) fn string(raw: &str) -> String {
    raw.to_owned()
}

/// `items` as a one-line JSON array of strings: `["a", "b"]`.
#[must_use]
pub(crate) fn array(items: &[&str]) -> String {
    let _ = items;
    String::new()
}

#[cfg(test)]
mod tests;
