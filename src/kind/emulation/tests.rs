//! The own-runner guard (V67): which systems a row emulates, the guard
//! step, and the refusal of labels that name other hardware.

use super::{emulate_value, emulated, guard, names_other, refuse_silent};
use crate::kind::Hosts;
use crate::registry::parse;
use crate::system::System;
use crate::yaml::Action;

const HEAD: &str = "format 1\n\n## workflows\nid|kind|systems|cache|args\n";

fn row(args: &str) -> Result<crate::core::kind::Row, String> {
    let text = format!("{HEAD}ci|gate|-|-|{args}\n");
    let registry = parse(&text).map_err(|e| e.to_string())?;
    let row = registry.workflows.first().ok_or("no row")?;
    let kind = crate::gate::Gate;
    crate::kind::check(&kind, row, &registry.caches)
        .map(|checked| checked.row)
        .map_err(|e| e.to_string())
}

#[test]
fn emulate_lists_systems_joined_by_plus() -> Result<(), String> {
    let both = row("emulate=x86_64-darwin+aarch64-linux")?;
    assert_eq!(
        emulated(&both),
        [System::X86_64Darwin, System::Aarch64Linux]
    );
    assert_eq!(emulated(&row("-")?), []);
    Ok(())
}

#[test]
fn an_unknown_emulate_system_is_refused() {
    let error = row("emulate=x86_64-darwin+riscv").err().unwrap_or_default();
    assert!(error.contains("unknown system `riscv`"), "{error}");
}

#[test]
fn a_missing_emulate_is_accepted() -> Result<(), String> {
    emulate_value(&row("-")?).map_err(|e| e.to_string())
}

#[test]
fn the_guard_marks_only_an_emulated_system() -> Result<(), String> {
    let row = row("emulate=x86_64-darwin")?;
    let run = |system| match guard(&row, system).map(|s| s.action) {
        Ok(Action::Run(command)) => command,
        other => format!("{other:?}"),
    };
    let script = "bash scripts/actlith/runner-system.sh";
    assert_eq!(
        run(System::X86_64Darwin),
        format!("EMULATE=true {script} x86_64-darwin")
    );
    assert_eq!(run(System::Aarch64Linux), format!("{script} aarch64-linux"));
    Ok(())
}

#[test]
fn labels_name_other_hardware_by_arch_and_os() {
    assert!(names_other(System::Aarch64Linux, "X64"));
    assert!(names_other(System::X86_64Darwin, "arm64"));
    assert!(names_other(System::X86_64Linux, "macOS"));
    assert!(names_other(System::Aarch64Darwin, "Linux"));
    assert!(!names_other(System::X86_64Darwin, "X64"));
    assert!(!names_other(System::X86_64Darwin, "self-hosted"));
}

#[test]
fn a_windows_label_names_another_os() {
    assert!(names_other(System::X86_64Linux, "Windows"));
}

#[test]
fn a_mismatched_row_is_refused_naming_the_fix_unless_emulated()
-> Result<(), String> {
    let labels = ["self-hosted".to_owned(), "X64".to_owned()];
    let hosts = Hosts {
        hosted: Vec::new(),
        own: vec![(System::Aarch64Linux, &labels[..])],
    };
    let plain = refuse_silent(&row("-")?, &hosts)
        .err()
        .map(|e| e.to_string())
        .unwrap_or_default();
    assert!(plain.contains("emulate=aarch64-linux"), "{plain}");
    let chosen = row("emulate=aarch64-linux")?;
    assert!(refuse_silent(&chosen, &hosts).is_ok());
    Ok(())
}

#[test]
fn a_windows_label_on_a_linux_row_is_refused() -> Result<(), String> {
    let labels = ["self-hosted".to_owned(), "Windows".to_owned()];
    let hosts = Hosts {
        hosted: Vec::new(),
        own: vec![(System::X86_64Linux, &labels[..])],
    };
    let refused = refuse_silent(&row("-")?, &hosts)
        .err()
        .map(|e| e.to_string())
        .unwrap_or_default();
    assert!(refused.contains("Windows"), "{refused}");
    assert!(refused.contains("emulate=x86_64-linux"), "{refused}");
    Ok(())
}
