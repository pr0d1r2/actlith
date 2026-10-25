//! One test per fixture in `tests/fixtures/registry/` (registry T13).

use crate::registry::{Registry, RegistryError, parse};

macro_rules! fixture {
    ($name:literal) => {
        include_str!(concat!(
            "../../../tests/fixtures/registry/",
            $name,
            ".actlith"
        ))
    };
}

fn refused(text: &str) -> RegistryError {
    parse(text).err().unwrap_or_default()
}

fn ids(reg: &Registry) -> Vec<&str> {
    reg.workflows.iter().map(|w| w.id.as_str()).collect()
}

#[test]
fn full_fills_the_workflows() {
    let reg = parse(fixture!("full")).unwrap_or_default();
    assert_eq!(ids(&reg), ["ci", "update-pins", "check"]);
    let lines: Vec<usize> = reg.workflows.iter().map(|w| w.line).collect();
    assert_eq!(lines, [8, 10, 11]);
    let ci = reg.workflows.first().map(|w| w.systems.len());
    assert_eq!(ci, Some(3));
    let check = reg.workflows.last().and_then(|w| w.args.first());
    let check = check.map(|a| a.value.as_str());
    assert_eq!(check, Some("hk check --all | tee log"));
}

#[test]
fn full_fills_the_other_sections() {
    let reg = parse(fixture!("full")).unwrap_or_default();
    let pin = reg.actions.first().map(|a| (a.line, a.version.as_str()));
    assert_eq!(pin, Some((15, "v4.2.2")));
    assert_eq!(reg.hand_written.len(), 1);
    let bots: Vec<&str> = reg.bots.iter().map(|b| b.bot.as_str()).collect();
    assert_eq!(bots, ["coderabbit", "socket"]);
}

#[test]
fn format_only_has_empty_sections() {
    assert_eq!(parse(fixture!("format-only")), Ok(Registry::default()));
}

#[test]
fn no_format_is_refused() {
    let e = refused(fixture!("no-format"));
    assert_eq!((e.line, e.what.as_str()), (1, "no `format` line"));
    assert_eq!(refused("").what, "no `format` line");
    assert_eq!(refused("# only a comment\n\n").what, "no `format` line");
}

#[test]
fn newer_format_is_refused() {
    let e = refused(fixture!("newer-format"));
    assert_eq!(e.line, 1);
    assert!(e.what.contains("format 2"), "{}", e.what);
}

#[test]
fn duplicate_id_names_both_lines() {
    let e = refused(fixture!("duplicate-id"));
    assert_eq!(e.line, 7);
    assert_eq!(e.what, "id `ci` given twice, first on line 5");
}

#[test]
fn empty_reason_is_refused() {
    let e = refused(fixture!("empty-reason"));
    assert_eq!((e.line, e.what.as_str()), (5, "`reason` is empty"));
}

#[test]
fn bad_cell_count_is_refused() {
    let e = refused(fixture!("bad-cell-count"));
    assert_eq!(e.line, 5);
    assert_eq!(e.what, "4 cells where `## workflows` rows have 5");
}

#[test]
fn bad_header_is_refused() {
    let e = refused(fixture!("bad-header"));
    assert_eq!(e.line, 4);
    assert_eq!(
        e.what,
        "`## workflows` header is `id|kind|systems|gate|cache`"
    );
}

#[test]
fn unknown_section_is_refused() {
    let e = refused(fixture!("unknown-section"));
    assert_eq!(
        (e.line, e.what.as_str()),
        (3, "unknown section `## checks`")
    );
    assert!(e.fix.contains("## hand-written"), "{}", e.fix);
}

#[test]
fn duplicate_section_names_both_lines() {
    let e = refused(fixture!("duplicate-section"));
    assert_eq!(e.line, 7);
    assert_eq!(e.what, "`## bots` given twice, first on line 3");
}

#[test]
fn row_outside_section_is_refused() {
    let e = refused(fixture!("row-outside-section"));
    assert_eq!(e.line, 2);
    assert_eq!(e.what, "row `ci|gate|-|-|-` is outside any section");
}

#[test]
fn a_section_may_be_empty_or_header_only() {
    let reg = parse("format 1\n## workflows\n## actions\naction|version|sha\n");
    assert_eq!(reg, Ok(Registry::default()));
}

#[test]
fn a_newer_format_reads_as_pipelith_words_it() {
    let e = refused("format 2\n");
    assert_eq!(e.line, 1);
    assert_eq!(e.what, "`.actlith` is format 2; this build reads format 1");
    assert_eq!(e.fix, "upgrade to a release that reads format 2");
}

#[test]
fn a_workflow_row_is_pipelith_raw_row() {
    use std::any::TypeId;
    let ours = TypeId::of::<crate::registry::WorkflowRow>();
    assert_eq!(ours, TypeId::of::<crate::core::kind::RawRow>());
}

#[test]
fn runners_name_a_system_s_own_labels() {
    use crate::core::runner::Runner;
    use crate::core::system::System;
    let reg = parse(fixture!("runners")).unwrap_or_default();
    let own = ["self-hosted", "macOS", "X64"].map(str::to_owned);
    let darwin = reg.runners.pick(System::X86_64Darwin, true);
    assert_eq!(darwin, Ok(Runner::Labels(own.as_slice())));
    let linux = reg.runners.pick(System::X86_64Linux, true);
    assert_eq!(linux, Ok(Runner::Default));
    assert_eq!(ids(&reg), ["ci"]);
}

#[test]
fn a_file_without_runners_has_none() {
    let reg = parse(fixture!("full")).unwrap_or_default();
    assert_eq!(reg.runners, crate::core::runner::Runners::default());
}

#[test]
fn a_runners_row_for_an_unknown_system_is_refused() {
    let e = refused(fixture!("runners-unknown-system"));
    assert_eq!(e.line, 5);
    assert!(e.what.starts_with("unknown system `riscv64-linux`"), "{e}");
}

#[test]
fn a_runners_label_must_be_a_plain_word() {
    let e = refused(fixture!("runners-bad-label"));
    assert_eq!(e.line, 5);
    assert_eq!(e.what, "label `-macOS` is not a runner label");
}

#[test]
fn a_runners_row_needs_labels() {
    let e = refused(fixture!("runners-empty"));
    assert_eq!(e.line, 5);
    assert!(e.what.contains("labels"), "{e}");
}

#[test]
fn a_system_named_twice_in_runners_is_refused() {
    let e = refused(fixture!("runners-repeated"));
    assert_eq!(e.line, 6);
    assert!(e.what.contains("x86_64-darwin"), "{e}");
}
