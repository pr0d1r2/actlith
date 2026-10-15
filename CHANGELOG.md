# Changelog

All notable changes to actlith are recorded here, in the
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format. The
project follows [Semantic Versioning](https://semver.org/): a change to
the bytes actlith emits for the same inputs is at least a minor version
(SPEC.md V18), and its entry names the files consumers will see change.

## [Unreleased]

### Added

- `src/system`: nix system to GitHub-hosted runner table; `x86_64-darwin`
  is refused, naming hand verification on an Intel Mac (T14).
- `.actlith` parser (`src/registry`): the `format` line and the
  `## workflows`, `## actions`, `## hand-written` and `## bots` rows,
  each error naming its line and a fix.
- Seeded scripts `scripts/actlith/update-pins.sh`, `open-pr.sh` and
  `verify-cache.sh`, embedded with the generated header (src/script T21).
- `src/yaml`: typed workflow model and own YAML writer with fixed key
  order and quoting, `|` and `>-` blocks and the generated header; no
  file is emitted yet (T16).
- `src/pin`: built-in pin table (checkout v7.0.1, install-nix-action
  v31.11.0, cachix-action v17) with `## actions` overrides; a non-SHA
  pin or an action without a pin is refused, naming it (T15).
- `src/detect`: proposed `## workflows` rows (`flake.nix` gives `ci|gate`,
  a `nixpkgs-lock` node in `flake.lock` gives `update-pins`) and
  `seed --init` writing the first `.actlith`, never over one (T27, V32).
