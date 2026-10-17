//! Row typing and key rules, through `parse` (registry T13, T64).

use crate::registry::{
    ActionPin, Arg, BotRow, HandWritten, Registry, RegistryError, WorkflowRow,
    parse,
};

const SHA: &str = "11bd71901bbe5b1630ceea73d27597364c9af683";

/// A section: its name and its header row.
type Table = (&'static str, &'static str);

const WORKFLOWS: Table = ("workflows", "id|kind|systems|cache|args");
const ACTIONS: Table = ("actions", "action|version|sha");
const HAND_WRITTEN: Table = ("hand-written", "path|reason");
const BOTS: Table = ("bots", "bot|args");

/// `row` alone in its section, on line 9.
fn one(table: Table, row: &str) -> Result<Registry, RegistryError> {
    let (name, header) = table;
    parse(&format!("format 1\n\n\n\n\n\n## {name}\n{header}\n{row}\n"))
}

fn refused(table: Table, row: &str) -> String {
    one(table, row).err().unwrap_or_default().what
}

#[test]
fn a_workflow_row_is_typed() {
    let reg = one(WORKFLOWS, "ci|gate|x86_64-linux|pr0d1r2|a=b");
    let want = WorkflowRow {
        line: 9,
        id: "ci".into(),
        kind: "gate".into(),
        systems: vec!["x86_64-linux".into()],
        cache: Some("pr0d1r2".into()),
        args: vec![Arg {
            key: "a".into(),
            value: "b".into(),
        }],
    };
    assert_eq!(reg.unwrap_or_default().workflows, [want]);
}

#[test]
fn a_workflow_row_takes_dashes() {
    let reg = one(WORKFLOWS, "ci|gate|-|-|-").unwrap_or_default();
    let row = reg.workflows.first().cloned();
    let row = row.map(|r| (r.systems.len(), r.cache, r.args.len()));
    assert_eq!(row, Some((0, None, 0)));
}

#[test]
fn a_workflow_needs_id_and_kind() {
    assert_eq!(refused(WORKFLOWS, "-|gate|-|-|-"), "`id` is empty");
    assert_eq!(refused(WORKFLOWS, "ci|-|-|-|-"), "`kind` is empty");
}

#[test]
fn a_workflow_id_is_a_file_name() {
    for bad in ["a/b", "a b", ".hidden", "ci:yml"] {
        let row = format!("{bad}|gate|-|-|-");
        let what = refused(WORKFLOWS, &row);
        assert!(what.contains("is not a file name"), "{bad}: {what}");
    }
    assert!(one(WORKFLOWS, "Up_pins.2-x|gate|-|-|-").is_ok());
}

#[test]
fn a_workflow_row_has_five_cells() {
    let what = refused(WORKFLOWS, "ci|gate|-|-");
    assert_eq!(what, "4 cells where `## workflows` rows have 5");
}

#[test]
fn an_action_row_is_typed() {
    let row = format!("actions/checkout|v4.2.2|{SHA}");
    let want = ActionPin {
        line: 9,
        action: "actions/checkout".into(),
        version: "v4.2.2".into(),
        sha: SHA.into(),
    };
    let reg = one(ACTIONS, &row).unwrap_or_default();
    assert_eq!(reg.actions, [want]);
}

#[test]
fn an_action_sha_is_40_lowercase_hex() {
    for bad in ["abc", &SHA.to_uppercase(), &format!("{SHA}0")] {
        let what = refused(ACTIONS, &format!("a/b|v1|{bad}"));
        assert!(what.contains("is not a full commit SHA"), "{what}");
    }
    assert_eq!(refused(ACTIONS, "a/b|v1|-"), "`sha` is empty");
    let row = format!("-|v1|{SHA}");
    assert_eq!(refused(ACTIONS, &row), "`action` is empty");
    let row = format!("a/b|-|{SHA}");
    assert_eq!(refused(ACTIONS, &row), "`version` is empty");
}

#[test]
fn a_hand_written_row_needs_a_reason() {
    let reg = one(HAND_WRITTEN, ".github/workflows/x.yml|why");
    let want = HandWritten {
        line: 9,
        path: ".github/workflows/x.yml".into(),
        reason: "why".into(),
    };
    assert_eq!(reg.unwrap_or_default().hand_written, [want]);
    let what = refused(HAND_WRITTEN, ".github/workflows/x.yml|-");
    assert_eq!(what, "`reason` is empty");
    assert_eq!(refused(HAND_WRITTEN, "-|why"), "`path` is empty");
}

#[test]
fn a_bot_row_is_typed() {
    let reg = one(BOTS, "coderabbit|central=pr0d1r2/coderabbit");
    let want = BotRow {
        line: 9,
        bot: "coderabbit".into(),
        args: vec![Arg {
            key: "central".into(),
            value: "pr0d1r2/coderabbit".into(),
        }],
    };
    assert_eq!(reg.unwrap_or_default().bots, [want]);
    assert_eq!(refused(BOTS, "-|-"), "`bot` is empty");
    assert_eq!(refused(BOTS, "socket|x"), "arg `x` is not `key=value`");
}

#[test]
fn distinct_keys_pass() {
    let text = "format 1\n## workflows\nid|kind|systems|cache|args\n\
                ci|gate|-|-|-\nbuild|build|-|-|package=x\n\
                ## hand-written\npath|reason\na.yml|r\n";
    assert!(parse(text).is_ok());
}

#[test]
fn a_duplicate_key_names_both_lines() {
    let sha = format!("a/b|v1|{SHA}");
    let cases = [
        (WORKFLOWS, "ci|gate|-|-|-", "id `ci`"),
        (ACTIONS, sha.as_str(), "action `a/b`"),
        (HAND_WRITTEN, "a.yml|r", "path `a.yml`"),
        (BOTS, "socket|-", "bot `socket`"),
    ];
    for ((name, header), row, key) in cases {
        let text = format!("format 1\n## {name}\n{header}\n{row}\n{row}\n");
        let e = parse(&text).err().unwrap_or_default();
        assert_eq!(e.line, 5);
        assert_eq!(e.what, format!("{key} given twice, first on line 4"));
        assert_eq!(e.fix, "keep one row per key");
    }
}
