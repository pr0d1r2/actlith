//! The closed kind list (C), the scripts each kind needs (V31), and
//! parity with `docs/KINDS.md` and each kind's node spec (V25, T33).

use super::{entries, lookup, names};
use crate::core::kind::{BUILD, GATE, UPDATE_PINS, UPSTREAM};
use crate::script::scripts;

const KINDS_MD: &str = include_str!("../../../docs/KINDS.md");

/// The node whose spec owns each kind.
const NODES: [(&str, &str); 4] = [
    ("gate", include_str!("../../gate/SPEC.md")),
    ("build", include_str!("../../build/SPEC.md")),
    ("update-pins", include_str!("../../bump/SPEC.md")),
    ("upstream", include_str!("../../bump/SPEC.md")),
];

/// One arg as written: key, whether required, the default it names.
type Arg<'a> = (&'a str, bool, Option<&'a str>);

#[test]
fn every_v1_kind_is_found_by_name() {
    for name in ["gate", "build", "update-pins", "upstream"] {
        let found = lookup(name).map(|e| e.kind.name());
        assert_eq!(found, Some(name));
    }
}

#[test]
fn each_kind_is_its_catalogue_entry() {
    let want = [&GATE, &BUILD, &UPDATE_PINS, &UPSTREAM];
    for (entry, spec) in entries().into_iter().zip(want) {
        assert_eq!(entry.kind.spec(), spec, "{}", spec.name);
    }
}

#[test]
fn any_other_name_is_no_kind() {
    for name in ["", "Gate", "lint", "update_pins"] {
        assert!(lookup(name).is_none(), "{name}");
    }
}

#[test]
fn the_names_list_every_kind_in_order() {
    assert_eq!(names(), "`gate`, `build`, `update-pins`, `upstream`");
}

#[test]
fn each_kind_needs_its_scripts() {
    let needs = |name: &str| lookup(name).map(|e| e.scripts.to_vec());
    assert_eq!(needs("gate"), Some(vec![]));
    assert_eq!(
        needs("build"),
        Some(vec!["scripts/actlith/verify-cache.sh"])
    );
    assert_eq!(
        needs("update-pins"),
        Some(vec![
            "scripts/actlith/update-pins.sh",
            "scripts/actlith/open-pr.sh"
        ])
    );
    assert_eq!(needs("upstream"), Some(vec!["scripts/actlith/open-pr.sh"]));
}

#[test]
fn every_needed_script_is_seeded() {
    for entry in entries() {
        for path in entry.scripts {
            assert!(scripts().iter().any(|(p, _)| p == path), "{path}");
        }
    }
}

/// The ``## `kind` `` sections of `docs/KINDS.md`, in order.
fn doc_kinds() -> Vec<(&'static str, &'static str)> {
    KINDS_MD
        .split("\n## ")
        .skip(1)
        .filter_map(|section| {
            let (head, body) = section.split_once('\n')?;
            let name = head.strip_prefix('`')?.strip_suffix('`')?;
            Some((name, body))
        })
        .collect()
}

/// The `- args:` bullet of a kind section, continuation lines included.
fn args_bullet(body: &str) -> &str {
    let rest = body.split_once("\n- args:").map_or("", |(_, r)| r);
    let bullet = rest.split("\n- ").next().unwrap_or(rest);
    bullet.split("\n\n").next().unwrap_or(bullet)
}

/// Parenthesis depth and backtick state after one more char.
fn scan((depth, tick): (u8, bool), c: char) -> (u8, bool) {
    match c {
        '`' => (depth, !tick),
        '(' if !tick => (depth.saturating_add(1), tick),
        ')' if !tick => (depth.saturating_sub(1), tick),
        _ => (depth, tick),
    }
}

/// The bullet split on commas outside parentheses and backticks.
fn items(bullet: &str) -> Vec<&str> {
    let (mut out, mut state, mut start) = (Vec::new(), (0, false), 0);
    for (i, c) in bullet.char_indices() {
        state = scan(state, c);
        if c == ',' && state == (0, false) {
            out.push(bullet.get(start..i).unwrap_or(""));
            start = i.saturating_add(1);
        }
    }
    out.push(bullet.get(start..).unwrap_or(""));
    out
}

/// One item: `` `key` (required | default `value` | ...) ``.
fn doc_arg(item: &str) -> Option<Arg<'_>> {
    let (_, rest) = item.split_once('`')?;
    let (key, rest) = rest.split_once('`')?;
    let paren = rest.split_once('(').map_or("", |(_, p)| p);
    let default = paren
        .split_once("default `")
        .and_then(|(_, d)| d.split_once('`'))
        .map(|(d, _)| d);
    Some((key, paren.starts_with("required"), default))
}

/// A spec's `## <head>` section, up to the next one.
fn section<'a>(spec: &'a str, head: &str) -> &'a str {
    let rest = spec.split_once(head).map_or("", |(_, r)| r);
    rest.split("\n## ").next().unwrap_or(rest)
}

#[test]
fn the_doc_lists_every_kind_in_code_order() {
    let doc: Vec<&str> = doc_kinds().into_iter().map(|(n, _)| n).collect();
    let code: Vec<&str> = entries().iter().map(|e| e.kind.name()).collect();
    assert_eq!(doc, code);
}

/// The args `docs/KINDS.md` gives the kind `name`.
fn doc_args(name: &str) -> Vec<Arg<'static>> {
    let docs = doc_kinds();
    let body = docs.iter().find(|(n, _)| *n == name).map_or("", |d| d.1);
    items(args_bullet(body))
        .into_iter()
        .filter_map(doc_arg)
        .collect()
}

#[test]
fn the_doc_args_are_each_schema_in_order() {
    for entry in entries() {
        let name = entry.kind.name();
        let code: Vec<Arg<'_>> = entry
            .kind
            .schema()
            .iter()
            .map(|a| (a.key, a.required, a.default))
            .collect();
        assert_eq!(doc_args(name), code, "docs/KINDS.md `{name}` args");
    }
}

#[test]
fn every_kind_has_its_node_rules() {
    for (name, spec) in NODES {
        assert!(lookup(name).is_some(), "{name}");
        let named = format!("`{name}`");
        assert!(
            section(spec, "## \u{a7}I").contains(&named),
            "{name} interfaces"
        );
        let rules = section(spec, "## \u{a7}V").lines();
        assert!(
            rules.filter(|l| l.starts_with('V')).count() > 0,
            "{name} invariants"
        );
        let tasks = section(spec, "## \u{a7}T");
        assert!(tasks.contains(&format!("kind {named}")), "{name} tasks");
    }
}
