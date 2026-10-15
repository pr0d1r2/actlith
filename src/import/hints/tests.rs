//! Hints (T26): the id, caches and arg values a text shows, read back
//! from where the kinds write them.

use crate::import::hints::{arg_values, caches, id};
use crate::import::normalise::normalise;

const NIX: &str = "      - uses: cachix/install-nix-action@x
        with:
          extra_nix_config: |
            extra-substituters = https://demo.cachix.org
            extra-substituters = https://demo.cachix.org
            extra-substituters = https://other.example
";

fn values(key: &str, text: &str) -> Vec<String> {
    arg_values(key, &normalise(text))
}

#[test]
fn the_id_is_the_top_level_name() {
    let text = "name: 'ci'\njobs:\n  gate:\n    name: gate\n";
    assert_eq!(id(&normalise(text)), Some("ci".to_owned()));
    assert_eq!(id(&normalise("jobs:\n  a:\n    name: b\n")), None);
}

#[test]
fn caches_come_from_the_substituters_once_each() {
    assert_eq!(caches(&normalise(NIX)), ["demo"]);
    assert!(caches(&normalise("name: ci\n")).is_empty());
}

#[test]
fn command_and_timeout_are_read_whole() {
    let text = "    timeout-minutes: 45\n      - run: nix flake check\n";
    assert_eq!(values("command", text), ["nix flake check"]);
    assert_eq!(values("timeout", text), ["45"]);
}

#[test]
fn the_package_is_read_from_a_packages_attribute() {
    let text = "      - run: 'nix build \".#packages.$SYSTEM.hk\"'
      - run: 'nix eval --raw .#packages.x86_64-darwin.hk.drvPath'
      - run: nix build .#packages
";
    assert_eq!(values("package", text), ["hk"]);
}

#[test]
fn bump_args_are_read_from_their_env() {
    let text = "    - cron: '0  5 * * 1'
          BUILD: '.#hk'
          GH_TOKEN: ${{ secrets.PINS_TOKEN }}
          GH_TOKEN: plain
          SCRIPT: scripts/bump.sh
          BRANCH: bump/hk-${{ steps.bump.outputs.NEW_VERSION }}
          BRANCH: ${{ x }}
";
    assert_eq!(values("cron", text), ["0  5 * * 1"]);
    assert_eq!(values("build", text), [".#hk"]);
    assert_eq!(values("token", text), ["PINS_TOKEN"]);
    assert_eq!(values("script", text), ["scripts/bump.sh"]);
    assert_eq!(values("branch-prefix", text), ["bump/hk-"]);
}

#[test]
fn quotes_are_undone_only_when_whole() {
    let text = "run: 'it''s'\nrun: \"a b\"\nrun: \"a\\tb\"\nrun: 'open\n";
    assert_eq!(
        values("command", text),
        ["it's", "a b", "\"a\\tb\"", "'open"]
    );
}

#[test]
fn an_unknown_key_or_block_content_shows_nothing() {
    assert!(values("nope", "run: x\n").is_empty());
    assert!(values("command", "k: |\n  run: x\n").is_empty());
}
