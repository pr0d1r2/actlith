//! The `.actlith` sections and the header row each one carries (`.:I`
//! file, registry C).

/// A known `## <section>`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Table {
    /// `## workflows`.
    Workflows,
    /// `## actions`.
    Actions,
    /// `## hand-written`.
    HandWritten,
    /// `## bots` (`src/bots` I).
    Bots,
}

/// Every section, in the order the spec lists them.
pub(crate) const ALL: [Table; 4] = [
    Table::Workflows,
    Table::Actions,
    Table::HandWritten,
    Table::Bots,
];

impl Table {
    /// The name after `## `.
    #[must_use]
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Workflows => "workflows",
            Self::Actions => "actions",
            Self::HandWritten => "hand-written",
            Self::Bots => "bots",
        }
    }

    /// The header row the section starts with.
    #[must_use]
    pub(crate) fn header(self) -> &'static str {
        match self {
            Self::Workflows => "id|kind|systems|cache|args",
            Self::Actions => "action|version|sha",
            Self::HandWritten => "path|reason",
            Self::Bots => "bot|args",
        }
    }

    /// The section a heading names, if any.
    #[must_use]
    pub(crate) fn named(name: &str) -> Option<Self> {
        ALL.into_iter().find(|t| t.name() == name)
    }

    /// The section names, as a fix hint lists them.
    #[must_use]
    pub(crate) fn names() -> String {
        let names: Vec<String> =
            ALL.iter().map(|t| format!("## {}", t.name())).collect();
        names.join(", ")
    }
}

#[cfg(test)]
mod tests;
