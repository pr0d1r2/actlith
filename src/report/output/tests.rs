//! `report` end to end (T28, V33): the render fixture `.actlith` (one row
//! of every v1 kind, a hand-written file, every v1 bot) reports the
//! golden text (below), JSON and Markdown (under
//! `tests/fixtures/report/`); a refused row is the error.

use crate::pin::PinTable;
use crate::registry::parse;
use crate::report::{Format, report};

const ALL: &str = include_str!("../../../tests/fixtures/render/all.actlith");
/// The golden text. Inline, not a fixture file: `.pklith` has no `txt`
/// type, and `pklith check` refuses a tracked file type without one.
const TEXT: &str = concat!(
    "workflows\n",
    "  build: kind build, cache pr0d1r2\n",
    "    system x86_64-linux on ubuntu-latest\n",
    "    system aarch64-linux on ubuntu-24.04-arm\n",
    "    system aarch64-darwin on macos-latest\n",
    "    system x86_64-darwin on macos-15-intel\n",
    "    action actions/checkout@\
     3d3c42e5aac5ba805825da76410c181273ba90b1 v7.0.1\n",
    "    action cachix/cachix-action@\
     5f2d7c5294214f71b873db4b969586b980625e71 v17\n",
    "    action cachix/install-nix-action@\
     630ae543ea3a38a9a4166f03376c02c50f408342 v31.11.0\n",
    "    secret CACHIX_AUTH_TOKEN: cachix push\n",
    "  ci: kind gate, cache pr0d1r2\n",
    "    system x86_64-linux on ubuntu-latest\n",
    "    system aarch64-linux on ubuntu-24.04-arm\n",
    "    system aarch64-darwin on macos-latest\n",
    "    system x86_64-darwin on macos-15-intel\n",
    "    action actions/checkout@\
     3d3c42e5aac5ba805825da76410c181273ba90b1 v7.0.1\n",
    "    action cachix/cachix-action@\
     5f2d7c5294214f71b873db4b969586b980625e71 v17\n",
    "    action cachix/install-nix-action@\
     630ae543ea3a38a9a4166f03376c02c50f408342 v31.11.0\n",
    "    secret CACHIX_AUTH_TOKEN: cachix push\n",
    "  update-pins: kind update-pins, cache -\n",
    "    system x86_64-linux on ubuntu-latest\n",
    "    action actions/checkout@\
     3d3c42e5aac5ba805825da76410c181273ba90b1 v7.0.1\n",
    "    action cachix/install-nix-action@\
     630ae543ea3a38a9a4166f03376c02c50f408342 v31.11.0\n",
    "    secret GITHUB_TOKEN: push the bump branch and open its PR\n",
    "  upstream: kind upstream, cache -\n",
    "    system x86_64-linux on ubuntu-latest\n",
    "    action actions/checkout@\
     3d3c42e5aac5ba805825da76410c181273ba90b1 v7.0.1\n",
    "    action cachix/install-nix-action@\
     630ae543ea3a38a9a4166f03376c02c50f408342 v31.11.0\n",
    "    secret GITHUB_TOKEN: push the bump branch and open its PR\n",
    "hand-written\n",
    "  .github/workflows/release.yml: not a v1 kind yet\n",
    "bots\n",
    "  coderabbit: CodeRabbit; install the app; free tier per \
     docs/SERVICES.md\n",
    "  greptile: Greptile; install the app; free tier per \
     docs/SERVICES.md\n",
    "  socket: Socket; install the app; free tier per docs/SERVICES.md\n",
);
const JSON: &str = include_str!("../../../tests/fixtures/report/all.json");
const MD: &str = include_str!("../../../tests/fixtures/report/all.md");

fn reported(text: &str, format: Format) -> Result<String, String> {
    let registry = parse(text).map_err(|e| e.to_string())?;
    let pins = PinTable::builtin()
        .with_overrides(&registry.actions)
        .map_err(|e| e.to_string())?;
    report(&registry, &pins, format).map_err(|e| e.to_string())
}

#[test]
fn the_fixture_reports_the_golden_text() -> Result<(), String> {
    assert_eq!(reported(ALL, Format::Text)?, TEXT);
    Ok(())
}

#[test]
fn the_fixture_reports_the_golden_json() -> Result<(), String> {
    assert_eq!(reported(ALL, Format::Json)?, JSON);
    Ok(())
}

#[test]
fn the_fixture_reports_the_golden_markdown() -> Result<(), String> {
    assert_eq!(reported(ALL, Format::Md)?, MD);
    Ok(())
}

#[test]
fn a_refused_row_is_the_error() {
    let text = "format 1\n\n## workflows\nid|kind|systems|cache|args\n\
                ci|deploy|-|-|-\n";
    let got = reported(text, Format::Json);
    assert!(got.is_err_and(|e| e.starts_with(".actlith:5: unknown kind")));
}
