//! Value types of the model: an expression, a text made of literal and
//! expression parts, and a comment (V5). None of them can carry a broken
//! `${{ }}`: an expression is checked when built, and a literal that holds
//! `${{` is written so that GitHub reads it back as those characters.

/// A GitHub Actions expression: the part between `${{` and `}}`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Expr(String);

impl Expr {
    /// The expression `inner`, trimmed. `None` when it is empty, holds a
    /// control character, or holds `${{` or `}}` (which would end or nest
    /// the expression early: an injected expression, V5).
    pub(crate) fn new(inner: &str) -> Option<Self> {
        let inner = inner.trim();
        let broken = inner.is_empty()
            || inner.contains("${{")
            || inner.contains("}}")
            || inner.chars().any(char::is_control);
        (!broken).then(|| Self(inner.to_owned()))
    }

    /// The expression as it is written in a workflow, `${{ inner }}`.
    pub(crate) fn render(&self) -> String {
        format!("${{{{ {} }}}}", self.0)
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

/// The literal `${{`, written as an expression that yields it.
const OPEN_ESCAPED: &str = "${{ '${{' }}";

impl Part {
    /// The part as written in a workflow. A literal `${{` becomes an
    /// expression that yields `${{`, so literal text never opens one.
    pub(crate) fn render(&self) -> String {
        match self {
            Self::Lit(s) => s.replace("${{", OPEN_ESCAPED),
            Self::Expr(e) => e.render(),
        }
    }
}

/// A string value: literal parts and expression parts, in order.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Text(Vec<Part>);

impl Text {
    /// A text of one literal part.
    pub(crate) fn lit(s: &str) -> Self {
        Self(vec![Part::Lit(s.to_owned())])
    }

    /// A text of one expression.
    pub(crate) fn expr(e: Expr) -> Self {
        Self(vec![Part::Expr(e)])
    }

    /// This text followed by the literal `s`.
    #[must_use]
    pub(crate) fn then_lit(mut self, s: &str) -> Self {
        self.0.push(Part::Lit(s.to_owned()));
        self
    }

    /// This text followed by the expression `e`.
    #[must_use]
    pub(crate) fn then_expr(mut self, e: Expr) -> Self {
        self.0.push(Part::Expr(e));
        self
    }

    /// The parts, in order.
    pub(crate) fn parts(&self) -> &[Part] {
        &self.0
    }

    /// The text as written in a workflow, before any YAML quoting.
    pub(crate) fn render(&self) -> String {
        self.0.iter().map(Part::render).collect()
    }
}

impl From<&str> for Text {
    fn from(s: &str) -> Self {
        Self::lit(s)
    }
}

/// A comment of one or more lines. A control character other than a line
/// break becomes a space, and trailing spaces are dropped, so a comment
/// never ends its line early or leaves trailing whitespace.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Comment(Vec<String>);

impl Comment {
    /// A comment of the lines of `text`.
    pub(crate) fn new(text: &str) -> Self {
        Self(text.split('\n').map(clean).collect())
    }

    /// The comment's lines, each as written: `# text`, or `#` when empty.
    pub(crate) fn lines(&self) -> Vec<String> {
        self.0.iter().map(|l| hash(l)).collect()
    }

    /// The comment on one line, its lines joined by a space: the form a
    /// comment takes after a value.
    pub(crate) fn inline(&self) -> String {
        hash(self.0.join(" ").trim())
    }
}

fn clean(line: &str) -> String {
    let spaced: String = line
        .chars()
        .map(|c| if c.is_control() { ' ' } else { c })
        .collect();
    spaced.trim_end().to_owned()
}

fn hash(line: &str) -> String {
    if line.is_empty() {
        "#".to_owned()
    } else {
        format!("# {line}")
    }
}

#[cfg(test)]
mod tests;
