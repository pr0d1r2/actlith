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
pub(crate) fn write(_value: &Json) -> String {
    String::new()
}

#[cfg(test)]
mod tests;
