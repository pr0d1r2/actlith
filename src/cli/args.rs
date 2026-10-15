//! The argv table: verbs, flags and `--help` text with examples (`.:I`,
//! T30). One table feeds parsing, completions and the man page.

use std::path::PathBuf;

use clap::{Args, CommandFactory as _, Parser, Subcommand, ValueEnum};
use clap_complete::Shell;

use crate::report::Format;

/// Every verb with an example, after the top-level help.
const EXAMPLES: &str = "\
Examples:
  actlith gen                    write the workflows the .actlith rows render
  actlith gen --check            exit 1 naming each stale or missing file
  actlith check                  exit 1 naming each undeclared workflow file
  actlith detect                 print rows proposed from the tracked files
  actlith seed --init            write a first .actlith from detect
  actlith import .github/workflows/ci.yml
                                 print its row, or where no kind matches
  actlith report --format json   workflows x systems x pins, and secrets
  actlith pin --dry-run          print fresh `## actions` rows (network)
  actlith completions zsh        print a shell completion script
  actlith man                    print the man page (roff)

Exit status: 0 ok, 1 finding, 2 usage, unreadable input or refused row.";

/// `actlith`: materialize GitHub Actions workflows from `.actlith`.
#[derive(Debug, Parser)]
#[command(name = "actlith", version, after_help = EXAMPLES)]
pub(crate) struct Cli {
    /// The verb.
    #[command(subcommand)]
    pub(crate) verb: Verb,
}

/// The repository root, shared by the verbs that read it.
#[derive(Debug, Clone, PartialEq, Eq, Args)]
pub(crate) struct Root {
    /// The repository root holding `.actlith`.
    #[arg(long, value_name = "DIR", default_value = ".")]
    pub(crate) root: PathBuf,
}

/// The verbs (`.:I` cmd).
#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub(crate) enum Verb {
    /// Write `.github/workflows/<id>.yml` for each row and the scripts
    /// the kinds need.
    #[command(after_help = "Example:\n  actlith gen --check --root ../repo")]
    Gen {
        #[command(flatten)]
        root: Root,
        /// Write nothing; exit 1 naming each stale or missing file.
        #[arg(long)]
        check: bool,
    },
    /// Every file in `.github/workflows/` is generated or hand-written.
    #[command(after_help = "Example:\n  actlith check --root ../repo")]
    Check(Root),
    /// Print `## workflows` rows proposed from the tracked files.
    #[command(after_help = "Example:\n  actlith detect")]
    Detect(Root),
    /// Write the first `.actlith` from `detect`; never over one.
    #[command(after_help = "Example:\n  actlith seed --init")]
    Seed {
        #[command(flatten)]
        root: Root,
        /// Write `.actlith` (required: `seed` does nothing without it).
        #[arg(long)]
        init: bool,
    },
    /// Print the row of each workflow file a kind renders exactly.
    #[command(
        after_help = "Example:\n  actlith import .github/workflows/ci.yml"
    )]
    Import {
        /// Workflow files, each named `<id>.yml`.
        #[arg(required = true, value_name = "FILE")]
        files: Vec<PathBuf>,
    },
    /// Report workflows x systems x action pins, and secrets.
    #[command(after_help = "Example:\n  actlith report --format md")]
    Report {
        #[command(flatten)]
        root: Root,
        /// How to print the report.
        #[arg(long, value_enum, default_value_t = ReportFormat::Text)]
        format: ReportFormat,
    },
    /// Resolve every action tag to a commit SHA via the GitHub API and
    /// write `## actions` (the only networked verb).
    #[command(after_help = "Example:\n  actlith pin --dry-run")]
    Pin {
        #[command(flatten)]
        root: Root,
        /// Print the rows; write nothing.
        #[arg(long)]
        dry_run: bool,
    },
    /// Print a shell completion script.
    #[command(after_help = "Example:\n  actlith completions zsh > _actlith")]
    Completions {
        /// The shell.
        #[arg(value_enum)]
        shell: Shell,
    },
    /// Print the man page (roff).
    #[command(after_help = "Example:\n  actlith man > actlith.1")]
    Man,
}

/// `report --format`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub(crate) enum ReportFormat {
    /// Indented plain text.
    Text,
    /// JSON with `schema_version`.
    Json,
    /// Markdown tables.
    Md,
}

impl From<ReportFormat> for Format {
    fn from(format: ReportFormat) -> Self {
        match format {
            ReportFormat::Text => Self::Text,
            ReportFormat::Json => Self::Json,
            ReportFormat::Md => Self::Md,
        }
    }
}

/// The clap command the argv table builds.
pub(crate) fn command() -> clap::Command {
    Cli::command()
}

#[cfg(test)]
mod tests;
