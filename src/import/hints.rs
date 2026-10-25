//! The values a workflow text shows for a row's cells: the workflow
//! name (the id), cachix cache names, and each arg value a kind writes
//! where the text can be read back (T26). Hints only: a candidate built
//! from them is accepted only when its render matches (V17).

use super::normalise::Line;

/// Lifts an arg value out of the value of a YAML key.
type Lift = fn(&str) -> Option<String>;

/// Per arg key: the YAML key whose value carries it, and the lift.
const LIFTS: [(&str, &str, Lift); 8] = [
    ("command", "run", whole),
    ("timeout", "timeout-minutes", whole),
    ("package", "run", package),
    ("cron", "cron", whole),
    ("build", "BUILD", whole),
    ("token", "GH_TOKEN", secret),
    ("script", "SCRIPT", whole),
    ("branch-prefix", "BRANCH", prefix),
];

/// The top-level `name:`, which a rendered workflow takes from its id.
pub(crate) fn id(lines: &[Line]) -> Option<String> {
    lines
        .iter()
        .filter(|l| !l.block)
        .find_map(|l| l.text.strip_prefix("name: "))
        .map(unquote)
}

/// Every cachix cache the text substitutes from, in text order.
pub(crate) fn caches(lines: &[Line]) -> Vec<String> {
    let names = lines.iter().filter(|l| l.block).filter_map(|l| {
        let url = l.text.trim().strip_prefix("extra-substituters = https://");
        url?.strip_suffix(".cachix.org").map(str::to_owned)
    });
    distinct(names.collect())
}

/// Every value the text shows for arg `key`, in text order.
pub(crate) fn arg_values(key: &str, lines: &[Line]) -> Vec<String> {
    let found = LIFTS.iter().filter(|(arg, _, _)| *arg == key).flat_map(
        |&(_, yaml, lift)| {
            values(lines, yaml)
                .into_iter()
                .filter_map(move |v| lift(&v))
        },
    );
    distinct(found.collect())
}

/// The unquoted values of every `yaml:` key, sequence items included.
fn values(lines: &[Line], yaml: &str) -> Vec<String> {
    lines
        .iter()
        .filter(|l| !l.block)
        .filter_map(|l| {
            let text = l.text.trim_start();
            let text = text.strip_prefix("- ").unwrap_or(text);
            text.strip_prefix(yaml)?.strip_prefix(": ").map(unquote)
        })
        .collect()
}

/// The value itself, unless empty.
fn whole(value: &str) -> Option<String> {
    (!value.is_empty()).then(|| value.to_owned())
}

/// `<package>` of `.#packages.<system>.<package>` in a command.
fn package(command: &str) -> Option<String> {
    let (_, attr) = command.split_once(".#packages.")?;
    let after = attr.split_once('.')?.1;
    let word = |c: char| c.is_ascii_alphanumeric() || matches!(c, '_' | '-');
    let name: String = after.chars().take_while(|&c| word(c)).collect();
    (!name.is_empty()).then_some(name)
}

/// `NAME` of `${{ secrets.NAME }}`.
fn secret(value: &str) -> Option<String> {
    let name = value.strip_prefix("${{ secrets.")?.strip_suffix(" }}")?;
    Some(name.to_owned())
}

/// The literal part of a branch before its `${{ ... }}` expression.
fn prefix(value: &str) -> Option<String> {
    let (head, _) = value.split_once("${{")?;
    (!head.is_empty()).then(|| head.to_owned())
}

/// A YAML scalar as written, its quotes undone: `'...'` with `''` for
/// `'`, `"..."` without escapes. Anything else stays as it is.
fn unquote(value: &str) -> String {
    let single = value.strip_prefix('\'').and_then(|v| v.strip_suffix('\''));
    if let Some(inner) = single {
        return inner.replace("''", "'");
    }
    let double = value.strip_prefix('"').and_then(|v| v.strip_suffix('"'));
    match double {
        Some(inner) if !inner.contains('\\') => inner.to_owned(),
        _ => value.to_owned(),
    }
}

/// `items` with repeats dropped, first seen kept.
fn distinct(items: Vec<String>) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for item in items {
        if !out.contains(&item) {
            out.push(item);
        }
    }
    out
}

#[cfg(test)]
mod tests;
