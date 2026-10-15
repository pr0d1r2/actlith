//! The argv table (T30): each verb parses, defaults hold, help shows an
//! example of every verb.

use std::path::PathBuf;

use clap::Parser as _;

use super::{Cli, ReportFormat, Root, Verb, command};
use crate::report::Format;

const VERBS: [&str; 9] = [
    "gen",
    "check",
    "detect",
    "seed",
    "import",
    "report",
    "pin",
    "completions",
    "man",
];

/// The verb `args` parse to, if they do.
fn verb(args: &[&str]) -> Option<Verb> {
    let full = ["actlith"].iter().chain(args);
    Cli::try_parse_from(full).ok().map(|cli| cli.verb)
}

fn root(dir: &str) -> Root {
    Root {
        root: PathBuf::from(dir),
    }
}

#[test]
fn the_table_is_consistent() {
    command().debug_assert();
}

#[test]
fn root_defaults_to_the_current_directory() {
    let check = verb(&["check"]);
    assert_eq!(check, Some(Verb::Check(root("."))));
    let gen_check = verb(&["gen", "--check", "--root", "r"]);
    let root = root("r");
    assert_eq!(gen_check, Some(Verb::Gen { root, check: true }));
}

#[test]
fn every_verb_parses() {
    assert!(verb(&["detect", "--root", "r"]).is_some());
    assert!(verb(&["import", "a.yml", "b.yml"]).is_some());
    assert!(verb(&["completions", "zsh"]).is_some());
    assert_eq!(verb(&["man"]), Some(Verb::Man));
}

#[test]
fn seed_and_pin_take_their_flags() {
    let (here, init) = (root("."), true);
    let seed = verb(&["seed", "--init"]);
    assert_eq!(seed, Some(Verb::Seed { root: here, init }));
    let (here, dry_run) = (root("."), true);
    let pin = verb(&["pin", "--dry-run"]);
    assert_eq!(
        pin,
        Some(Verb::Pin {
            root: here,
            dry_run
        })
    );
}

#[test]
fn report_format_defaults_to_text_and_maps_each() {
    let text = verb(&["report"]);
    let format = ReportFormat::Text;
    assert_eq!(
        text,
        Some(Verb::Report {
            root: root("."),
            format
        })
    );
    assert!(verb(&["report", "--format", "yaml"]).is_none());
    assert_eq!(Format::from(ReportFormat::Text), Format::Text);
    assert_eq!(Format::from(ReportFormat::Json), Format::Json);
    assert_eq!(Format::from(ReportFormat::Md), Format::Md);
}

#[test]
fn usage_errors_do_not_parse() {
    assert!(verb(&[]).is_none());
    assert!(verb(&["import"]).is_none());
    assert!(verb(&["completions", "cmd"]).is_none());
    assert!(verb(&["nope"]).is_none());
}

#[test]
fn help_names_every_verb_with_an_example() {
    let help = command().render_long_help().to_string();
    for name in VERBS {
        assert!(help.contains(&format!("  {name} ")), "verb {name}");
        assert!(help.contains(&format!("actlith {name}")), "example {name}");
    }
}
