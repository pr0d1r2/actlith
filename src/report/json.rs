//! A tiny JSON writer (no serde, `.:C` deps minimal): a value tree printed
//! with two spaces per level, strings escaped per RFC 8259.

/// A JSON value. Object fields keep their order: names are stable (V33).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Json {
    Null,
    Number(u32),
    String(String),
    Array(Vec<Json>),
    Object(Vec<(&'static str, Json)>),
}

/// `value` as a document ending in a newline.
pub(crate) fn write(value: &Json) -> String {
    format!("{}\n", text(value, ""))
}

/// `value` with its nested lines indented past `indent`.
fn text(value: &Json, indent: &str) -> String {
    let inner = format!("{indent}  ");
    match value {
        Json::Null => "null".to_owned(),
        Json::Number(n) => n.to_string(),
        Json::String(s) => string(s),
        Json::Array(items) => {
            let items = items.iter().map(|v| text(v, &inner));
            block(indent, ('[', ']'), &items.collect::<Vec<_>>())
        }
        Json::Object(fields) => {
            block(indent, ('{', '}'), &object(fields, &inner))
        }
    }
}

/// Each field as `"key": value`, nested lines indented past `inner`.
fn object(fields: &[(&str, Json)], inner: &str) -> Vec<String> {
    let field =
        |(k, v): &(&str, Json)| format!("{}: {}", string(k), text(v, inner));
    fields.iter().map(field).collect()
}

/// `entries` one per line between `brackets`; `[]` or `{}` when empty.
fn block(indent: &str, brackets: (char, char), entries: &[String]) -> String {
    let (open, close) = brackets;
    if entries.is_empty() {
        return format!("{open}{close}");
    }
    let lines: Vec<String> =
        entries.iter().map(|e| format!("{indent}  {e}")).collect();
    format!("{open}\n{}\n{indent}{close}", lines.join(",\n"))
}

/// `raw` as a JSON string, quotes included.
fn string(raw: &str) -> String {
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

#[cfg(test)]
mod tests;
