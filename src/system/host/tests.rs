//! `host` (V53): a `## runners` row wins over the table; without one the
//! table's label is the host.

use super::{Host, Runners, host};
use crate::core::registry::{self, Schema};
use crate::core::runner::RUNNERS;
use crate::system::System;

fn runners(rows: &str) -> Result<Runners, String> {
    let text = format!("format 1\n\n## runners\nsystem|labels\n{rows}");
    let schema = Schema {
        sections: &[RUNNERS],
    };
    let doc = registry::parse(".actlith", &text, &schema)
        .map_err(|e| e.to_string())?;
    Runners::from_document(".actlith", &doc).map_err(|e| e.to_string())
}

#[test]
fn without_a_row_every_system_is_hosted() {
    let none = Runners::default();
    let labels = [
        (System::X86_64Linux, "ubuntu-latest"),
        (System::Aarch64Linux, "ubuntu-24.04-arm"),
        (System::Aarch64Darwin, "macos-latest"),
        (System::X86_64Darwin, "macos-15-intel"),
    ];
    for (system, label) in labels {
        assert_eq!(host(&none, system), Host::Hosted(label), "{system}");
    }
}

#[test]
fn a_row_puts_its_system_on_the_own_labels() -> Result<(), String> {
    let own = runners("x86_64-darwin|self-hosted, macOS, X64\n")?;
    let labels = ["self-hosted", "macOS", "X64"].map(str::to_owned);
    assert_eq!(
        host(&own, System::X86_64Darwin),
        Host::Own(labels.as_slice())
    );
    Ok(())
}

#[test]
fn a_row_leaves_the_other_systems_hosted() -> Result<(), String> {
    let own = runners("x86_64-darwin|self-hosted, macOS, X64\n")?;
    assert_eq!(
        host(&own, System::Aarch64Darwin),
        Host::Hosted("macos-latest")
    );
    Ok(())
}

#[test]
fn a_row_wins_over_a_hosted_label() -> Result<(), String> {
    let own = runners("x86_64-linux|self-hosted, linux, X64\n")?;
    let got = host(&own, System::X86_64Linux);
    assert!(matches!(got, Host::Own(l) if l.len() == 3), "{got:?}");
    Ok(())
}
