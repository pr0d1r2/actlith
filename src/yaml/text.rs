//! Value types of the model: an expression, a text made of literal and
//! expression parts, and a comment (V5). RED: signatures only.

/// A GitHub Actions expression: the part between `${{` and `}}`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Expr(String);

impl Expr {
    /// The expression `inner`, trimmed; `None` when broken.
    pub(crate) fn new(_inner: &str) -> Option<Self> {
        None
    }

    /// The expression as it is written in a workflow, `${{ inner }}`.
    pub(crate) fn render(&self) -> String {
        String::new()
    }

    /// The expression without its `${{ }}` delimiters.
    pub(crate) fn inner(&self) -> &str {
        &self.0
    }
}

/// One part of a [`Text`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Part {
    /// Characters taken as they are.
    Lit(String),
    /// An expression GitHub evaluates.
    Expr(Expr),
}

/// A string value: literal parts and expression parts, in order.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Text(Vec<Part>);

impl Text {
    /// A text of one literal part.
    pub(crate) fn lit(_s: &str) -> Self {
        Self::default()
    }

    /// A text of one expression.
    pub(crate) fn expr(_e: Expr) -> Self {
        Self::default()
    }

    /// This text followed by the literal `s`.
    #[must_use]
    pub(crate) fn then_lit(self, _s: &str) -> Self {
        self
    }

    /// This text followed by the expression `e`.
    #[must_use]
    pub(crate) fn then_expr(self, _e: Expr) -> Self {
        self
    }

    /// The parts, in order.
    pub(crate) fn parts(&self) -> &[Part] {
        &self.0
    }

    /// The text as written in a workflow, before any YAML quoting.
    pub(crate) fn render(&self) -> String {
        String::new()
    }
}

impl From<&str> for Text {
    fn from(s: &str) -> Self {
        Self::lit(s)
    }
}

/// A comment of one or more lines.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Comment(Vec<String>);

impl Comment {
    /// A comment of the lines of `text`.
    pub(crate) fn new(_text: &str) -> Self {
        Self(Vec::new())
    }

    /// The comment's lines, each as written.
    pub(crate) fn lines(&self) -> Vec<String> {
        self.0.clone()
    }

    /// The comment on one line.
    pub(crate) fn inline(&self) -> String {
        String::new()
    }
}

#[cfg(test)]
mod tests;
