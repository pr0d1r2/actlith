use super::{add, unique};
use crate::registry::table::Table;
use crate::registry::{
    ActionPin, Arg, BotRow, HandWritten, Registry, RegistryError, WorkflowRow,
};

const SHA: &str = "11bd71901bbe5b1630ceea73d27597364c9af683";

fn one(table: Table, row: &str) -> Result<Registry, RegistryError> {
    let mut reg = Registry::default();
    add(&mut reg, table, 9, row)?;
    Ok(reg)
}

fn refused(table: Table, row: &str) -> String {
    one(table, row).err().unwrap_or_default().what
}

#[test]
fn a_workflow_row_is_typed() {
    let reg = one(Table::Workflows, "ci|gate|x86_64-linux|pr0d1r2|a=b");
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
    let reg = one(Table::Workflows, "ci|gate|-|-|-").unwrap_or_default();
    let row = reg.workflows.first().cloned();
    let row = row.map(|r| (r.systems.len(), r.cache, r.args.len()));
    assert_eq!(row, Some((0, None, 0)));
}

#[test]
fn a_workflow_needs_id_and_kind() {
    assert_eq!(refused(Table::Workflows, "-|gate|-|-|-"), "`id` is empty");
    assert_eq!(refused(Table::Workflows, "ci|-|-|-|-"), "`kind` is empty");
}

#[test]
fn a_workflow_id_is_a_file_name() {
    for bad in ["a/b", "a b", ".hidden", "ci:yml"] {
        let row = format!("{bad}|gate|-|-|-");
        let what = refused(Table::Workflows, &row);
        assert!(what.contains("is not a file name"), "{bad}: {what}");
    }
    assert!(one(Table::Workflows, "Up_pins.2-x|gate|-|-|-").is_ok());
}

#[test]
fn a_workflow_row_has_five_cells() {
    let what = refused(Table::Workflows, "ci|gate|-|-");
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
    let reg = one(Table::Actions, &row).unwrap_or_default();
    assert_eq!(reg.actions, [want]);
}

#[test]
fn an_action_sha_is_40_lowercase_hex() {
    for bad in ["abc", &SHA.to_uppercase(), &format!("{SHA}0")] {
        let what = refused(Table::Actions, &format!("a/b|v1|{bad}"));
        assert!(what.contains("is not a full commit SHA"), "{what}");
    }
    assert_eq!(refused(Table::Actions, "a/b|v1|-"), "`sha` is empty");
    let row = format!("-|v1|{SHA}");
    assert_eq!(refused(Table::Actions, &row), "`action` is empty");
    let row = format!("a/b|-|{SHA}");
    assert_eq!(refused(Table::Actions, &row), "`version` is empty");
}

#[test]
fn a_hand_written_row_needs_a_reason() {
    let reg = one(Table::HandWritten, ".github/workflows/x.yml|why");
    let want = HandWritten {
        line: 9,
        path: ".github/workflows/x.yml".into(),
        reason: "why".into(),
    };
    assert_eq!(reg.unwrap_or_default().hand_written, [want]);
    let what = refused(Table::HandWritten, ".github/workflows/x.yml|-");
    assert_eq!(what, "`reason` is empty");
    assert_eq!(refused(Table::HandWritten, "-|why"), "`path` is empty");
}

#[test]
fn a_bot_row_is_typed() {
    let reg = one(Table::Bots, "coderabbit|central=pr0d1r2/coderabbit");
    let want = BotRow {
        line: 9,
        bot: "coderabbit".into(),
        args: vec![Arg {
            key: "central".into(),
            value: "pr0d1r2/coderabbit".into(),
        }],
    };
    assert_eq!(reg.unwrap_or_default().bots, [want]);
    assert_eq!(refused(Table::Bots, "-|-"), "`bot` is empty");
    assert_eq!(
        refused(Table::Bots, "socket|x"),
        "arg `x` is not `key=value`"
    );
}

fn registry(rows: &[(Table, usize, &str)]) -> Registry {
    let mut reg = Registry::default();
    for (table, line, row) in rows {
        let _ = add(&mut reg, *table, *line, row);
    }
    reg
}

#[test]
fn distinct_keys_pass() {
    let reg = registry(&[
        (Table::Workflows, 3, "ci|gate|-|-|-"),
        (Table::Workflows, 4, "build|build|-|-|package=x"),
        (Table::HandWritten, 7, "a.yml|r"),
    ]);
    assert_eq!(unique(&reg), Ok(()));
}

#[test]
fn a_duplicate_key_names_both_lines() {
    let sha = format!("a/b|v1|{SHA}");
    let cases = [
        (Table::Workflows, "ci|gate|-|-|-", "id `ci`"),
        (Table::Actions, sha.as_str(), "action `a/b`"),
        (Table::HandWritten, "a.yml|r", "path `a.yml`"),
        (Table::Bots, "socket|-", "bot `socket`"),
    ];
    for (table, row, key) in cases {
        let reg = registry(&[(table, 3, row), (table, 8, row)]);
        let e = unique(&reg).err().unwrap_or_default();
        assert_eq!(e.line, 8);
        assert_eq!(e.what, format!("{key} given twice, first on line 3"));
        assert_eq!(e.fix, "keep one row per key");
    }
}
