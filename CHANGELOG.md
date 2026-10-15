# Changelog

All notable changes to actlith are recorded here, in the
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format. The
project follows [Semantic Versioning](https://semver.org/): a change to
the bytes actlith emits for the same inputs is at least a minor version
(SPEC.md V18), and its entry names the files consumers will see change.

## [Unreleased]

### Added

- `src/gen`: `gen` writes each rendered file under the root, only where
  its bytes differ; `gen --check` names each missing or stale file and
  writes nothing; a path leaving the root is refused (T24, V6).
- `src/render`: `.actlith` to the sorted file set `gen` writes: one
  `.github/workflows/<id>.yml` per row, each seeded script a rendered
  kind needs once, the `## bots` files; an unknown kind or system names
  its row (T38, V31).
- `src/fetch`: `pin` resolves each pinned action to its latest release
  tag and commit SHA through `gh api` and rewrites only `## actions` of
  `.actlith`; a dry run writes nothing (T29).
- Kinds `update-pins` and `upstream` (`src/bump` T22, T23): scheduled
  bump PRs with job-level `contents`/`pull-requests` writes, a `token`
  arg naming the secret, no `cancel-in-progress`, the R5 limit in the
  header comment.
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
- `src/bots`: `.coderabbit.yaml` (remote config from a central repo,
  inheritance on), `greptile.json` (skips release PRs) and `socket.yml`
  from `## bots` rows; an unknown bot or arg names its line (T39).
- `src/kind`: the `Kind` trait, arg schema check naming the row line,
  and the hardened common parts every kind builds from (permissions,
  concurrency, timeouts, checkout, install-nix, cachix, env-only `run:`);
  no file is emitted yet (T17).
- Kind `gate` (`src/gate`): the hk gate per system, job `gate`, args
  `command` and `timeout`; actlith's own row renders this repository's
  `ci.yml` byte for byte; no file is emitted yet (T19, V29).
- `src/build`: kind `build` with jobs `build` (matrix per system, push
  only from `main`), `check` (`nix flake check --all-systems` plus
  eval-only systems) and `verify-cache` (runs the seeded
  `verify-cache.sh`); no file is emitted yet (T20, V30).
