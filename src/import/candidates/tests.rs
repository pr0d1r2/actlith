//! Candidates (T26): every kind x systems x cache x args the text
//! shows, absent first, so the first match is the shortest row.

use crate::import::candidates::candidates;
use crate::import::normalise::normalise;

fn rows(text: &str) -> Vec<String> {
    candidates(&normalise(text))
}

#[test]
fn a_bare_text_gives_each_kind_on_each_systems_choice() {
    let all = rows("name: ci\n");
    assert_eq!(all.len(), 4 * 16);
    assert_eq!(all.first().map(String::as_str), Some("ci|gate|-|-|-"));
    let kinds: Vec<&str> = all
        .iter()
        .filter(|r| r.contains("|-|-|-"))
        .filter_map(|r| r.split('|').nth(1))
        .collect();
    assert_eq!(kinds, ["gate", "build", "update-pins", "upstream"]);
}

#[test]
fn systems_are_every_ordered_pick_shortest_first() {
    let all = rows("name: ci\n");
    let systems: Vec<&str> = all
        .iter()
        .take(16)
        .filter_map(|r| r.split('|').nth(2))
        .collect();
    assert_eq!(systems.get(1), Some(&"x86_64-linux"));
    assert_eq!(systems.get(4), Some(&"x86_64-linux, aarch64-linux"));
    assert_eq!(
        systems.last(),
        Some(&"aarch64-darwin, aarch64-linux, x86_64-linux")
    );
}

#[test]
fn a_cache_and_arg_values_from_the_text_widen_the_set() {
    let text = "name: ci
    timeout-minutes: 45
          extra_nix_config: |
            extra-substituters = https://demo.cachix.org
      - run: nix flake check
";
    let all = rows(text);
    let gate = 16 * 2 * 2 * 2;
    assert!(all.len() > gate, "{}", all.len());
    assert!(all.contains(&"ci|gate|-|demo|-".to_owned()));
    let full = "ci|gate|-|demo|command=nix flake check, timeout=45";
    assert!(all.contains(&full.to_owned()));
}

#[test]
fn a_pipe_is_escaped_and_a_comma_value_is_skipped() {
    let text = "name: a|b\n      - run: x | y\n      - run: a, b\n";
    let all = rows(text);
    assert!(all.contains(&"a\\|b|gate|-|-|command=x \\| y".to_owned()));
    assert!(!all.iter().any(|r| r.contains("a, b")));
}

#[test]
fn no_name_leaves_the_id_empty() {
    assert_eq!(
        rows("on: push\n").first().map(String::as_str),
        Some("|gate|-|-|-")
    );
}
