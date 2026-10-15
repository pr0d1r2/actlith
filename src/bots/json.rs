//! The little JSON `greptile.json` needs: strings and string arrays,
//! escaped per RFC 8259 so caller text never breaks the document.

/// `raw` as a JSON string, quotes included.
#[must_use]
pub(crate) fn string(raw: &str) -> String {
    let body: String = raw.chars().map(escape).collect();
    format!("\"{body}\"")
}

/// One character as it sits inside a JSON string.
fn escape(c: char) -> String {
    match c {
        '"' => "\\\"".to_owned(),
        '\\' => "\\\\".to_owned(),
        '\n' => "\\n".to_owned(),
        '\t' => "\\t".to_owned(),
        '\r' => "\\r".to_owned(),
        c if c.is_control() => format!("\\u{:04x}", u32::from(c)),
        c => c.to_string(),
    }
}

/// `items` as a one-line JSON array of strings: `["a", "b"]`.
#[must_use]
pub(crate) fn array(items: &[&str]) -> String {
    let items: Vec<String> = items.iter().map(|i| string(i)).collect();
    format!("[{}]", items.join(", "))
}

#[cfg(test)]
mod tests;
