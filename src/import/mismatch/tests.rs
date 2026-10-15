//! A refusal (V17) names the first differing line and its dotted key
//! path, against the closest render.

use crate::import::mismatch::{Mismatch, common, path};
use crate::import::normalise::normalise;

const RENDER: &str = "name: ci
on:
  push:
    branches: [main]
jobs:
  gate:
    steps:
      - uses: a
        with:
          extra_nix_config: |
            x: y
";

#[test]
fn the_path_walks_up_by_indentation() {
    let lines = normalise(RENDER);
    assert_eq!(path(&lines, 3), "on.push.branches");
    assert_eq!(path(&lines, 7), "jobs.gate.steps.uses");
    assert_eq!(path(&lines, 10), "jobs.gate.steps.with.extra_nix_config");
}

#[test]
fn the_first_differing_line_is_named() {
    let text = normalise(&RENDER.replace("[main]", "[dev]"));
    let render = normalise(RENDER);
    assert_eq!(common(&text, &render), 3);
    let mismatch = Mismatch::at(&text, &render, "gate");
    assert_eq!(
        mismatch,
        Mismatch {
            line: 4,
            key: "on.push.branches".to_owned(),
            kind: Some("gate".to_owned()),
            expected: Some("branches: [main]".to_owned()),
            found: Some("branches: [dev]".to_owned()),
        }
    );
}

#[test]
fn a_text_that_stops_short_is_named_past_its_end() {
    let text = normalise("# c\nname: ci\non:\n");
    let render = normalise(RENDER);
    let mismatch = Mismatch::at(&text, &render, "gate");
    assert_eq!(mismatch.line, 4);
    assert_eq!(mismatch.key, "on.push");
    assert_eq!(mismatch.found, None);
    assert_eq!(Mismatch::at(&[], &render, "gate").line, 1);
}

#[test]
fn a_text_that_runs_on_is_named_at_its_extra_line() {
    let text = normalise(&format!("{RENDER}extra: 1\n"));
    let mismatch = Mismatch::at(&text, &normalise(RENDER), "build");
    assert_eq!((mismatch.line, mismatch.key.as_str()), (12, "extra"));
    assert_eq!(mismatch.expected, None);
}

#[test]
fn an_unnamed_text_is_named_at_its_name_or_first_line() {
    let named = Mismatch::unnamed(&normalise("on: push\nname: Two words\n"));
    assert_eq!(named.line, 2);
    assert_eq!(named.found.as_deref(), Some("name: Two words"));
    let bare = Mismatch::unnamed(&normalise("\non: push\n"));
    assert_eq!((bare.line, bare.kind, bare.found), (2, None, None));
    assert_eq!(Mismatch::unnamed(&[]).line, 1);
}

#[test]
fn the_message_names_line_key_both_sides_and_a_fix() {
    let text = normalise(&RENDER.replace("[main]", "[dev]"));
    let shown = Mismatch::at(&text, &normalise(RENDER), "gate").to_string();
    assert_eq!(
        shown,
        "line 4: `on.push.branches` differs (closest kind `gate`): expected \
         `branches: [main]`, found `branches: [dev]`\n  fix: list the file \
         under `## hand-written`"
    );
    let none = Mismatch::unnamed(&normalise("on: push\n")).to_string();
    assert!(none.contains("(no kind renders)"), "{none}");
    assert!(none.contains("found nothing"), "{none}");
}
