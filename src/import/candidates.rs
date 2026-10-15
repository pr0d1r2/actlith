//! Candidate `## workflows` rows for a workflow text (T26): every v1
//! kind x every systems choice x no cache or a cache the text names x
//! each arg absent or set to a value the text shows. Absent comes
//! first in every choice, so the first match is the shortest row.

use super::hints::{arg_values, caches, id};
use super::normalise::Line;

/// Each v1 kind (`docs/KINDS.md`) and the arg keys import reads back,
/// in schema order.
const KINDS: [(&str, &[&str]); 4] = [
    ("gate", &["command", "timeout"]),
    ("build", &["package", "timeout"]),
    ("update-pins", &["cron", "build", "token", "timeout"]),
    (
        "upstream",
        &["script", "cron", "branch-prefix", "token", "timeout"],
    ),
];

/// The systems with a hosted runner, in `src/system` table order.
const SYSTEMS: [&str; 3] = ["x86_64-linux", "aarch64-linux", "aarch64-darwin"];

/// Every candidate row for `lines`, as `.actlith` row text.
pub(crate) fn candidates(lines: &[Line]) -> Vec<String> {
    let id = escape(&id(lines).unwrap_or_default());
    let mut caches: Vec<String> =
        caches(lines).iter().map(|c| escape(c)).collect();
    caches.insert(0, "-".to_owned());
    let systems = systems();
    let mut out = Vec::new();
    for (kind, keys) in KINDS {
        for args in args(keys, lines) {
            for (system, cache) in pairs(&systems, &caches) {
                out.push(format!("{id}|{kind}|{system}|{cache}|{args}"));
            }
        }
    }
    out
}

/// Every (systems, cache) cell pair, systems outer.
fn pairs<'a>(
    systems: &'a [String],
    caches: &'a [String],
) -> impl Iterator<Item = (&'a String, &'a String)> {
    systems
        .iter()
        .flat_map(move |s| caches.iter().map(move |c| (s, c)))
}

/// `-`, then every ordered pick of distinct runner systems, shortest
/// first.
fn systems() -> Vec<String> {
    let mut all: Vec<Vec<&str>> = vec![Vec::new()];
    let mut frontier = all.clone();
    for _ in SYSTEMS {
        frontier = frontier.iter().flat_map(|pick| grow(pick)).collect();
        all.extend(frontier.iter().cloned());
    }
    all.iter().map(|pick| cell(&pick.join(", "))).collect()
}

/// `pick` extended by each system not yet in it.
fn grow<'a>(pick: &[&'a str]) -> Vec<Vec<&'a str>> {
    SYSTEMS
        .iter()
        .filter(|s| !pick.contains(s))
        .map(|s| [pick, &[*s]].concat())
        .collect()
}

/// Every args cell for `keys`: each key absent or set to a value the
/// text shows.
fn args(keys: &[&str], lines: &[Line]) -> Vec<String> {
    let mut picks: Vec<Vec<String>> = vec![Vec::new()];
    for key in keys {
        let values = arg_values(key, lines);
        picks = picks
            .iter()
            .flat_map(|pick| with(pick, key, &values))
            .collect();
    }
    picks.iter().map(|pick| cell(&pick.join(", "))).collect()
}

/// `pick` as it is, then with `key=value` for each value a cell can
/// hold (args split on `,`, `src/registry` I).
fn with(pick: &[String], key: &str, values: &[String]) -> Vec<Vec<String>> {
    let fits = values.iter().filter(|v| !v.contains(','));
    let set = fits.map(|v| format!("{key}={}", escape(v)));
    let more = set.map(|arg| [pick, &[arg]].concat());
    std::iter::once(pick.to_vec()).chain(more).collect()
}

/// A cell: `-` when empty.
fn cell(text: &str) -> String {
    if text.is_empty() {
        "-".to_owned()
    } else {
        text.to_owned()
    }
}

/// A cell value with `|` escaped as `.actlith` reads it.
fn escape(value: &str) -> String {
    value.replace('|', "\\|")
}

#[cfg(test)]
mod tests;
