# Changelog

All notable changes to actlith are recorded here, in the
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format. The
project follows [Semantic Versioning](https://semver.org/): a change to
the bytes actlith emits for the same inputs is at least a minor version
(SPEC.md V18), and its entry names the files consumers will see change.

## [Unreleased]

### Added

- `## caches` in `.actlith`, with `name|public-key` rows parsed by
  `src/registry`; omitted sections continue to use pipelith's built-in
  Cachix cache table.

- `## runners` in `.actlith`, header `system|labels` (pipelith 0.2's
  neutral section, read with `Runners::from_document`): a row such as
  `x86_64-darwin|self-hosted, macOS, X64` moves that system's jobs, in
  every row, onto the repository's own runner. Each such job is a job of
  its own, `runs-on: [self-hosted, macOS, X64]`: `gate-<system>` named
  `gate (<system>)`, `build-<system>` named `build` over a one-entry
  `include`, and the single job of `update-pins` and `upstream`, or
  `check` and `verify-cache` when the row names `x86_64-linux`. Every
  such job carries `if: github.event_name != 'pull_request' ||
  github.event.pull_request.head.repo.full_name == github.repository`,
  so a fork's pull request never runs on the owner's machine; there is
  no opt-out, and hosted jobs never carry it (src/kind V54, pipelith
  V81). `verify-cache` needs every build job. An unknown system, an
  empty `labels` cell, a label other than letters, digits and `._-` or
  a system named twice is refused with its line. `actlith report` names
  the labels, joined by `,`, as the runner (`schema_version` stays 1).
  Files from `.actlith` files without the section are unchanged by it.

- `gate`: the `xcode=<version>` arg pins the Xcode a macOS row builds
  with, for a repository whose toolchain nix cannot supply (T69, issue
  #18). Its workflow sets `DEVELOPER_DIR` to
  `/Applications/Xcode_<version>.app/Contents/Developer` on the job, and
  a first step fails naming that path when the runner image lacks it,
  then prints `xcodebuild -version`. A row with a Linux system or a
  value that is not digits and dots is refused. `xcode` is GitHub-only:
  the portability check names it, and the other kinds refuse it.
  Rows without it render the same bytes as before.
- `report`: a pinned Xcode is listed beside its runner (T70): text
  `system <s> on <runner>, xcode <v>`, md `(Xcode <v>)` after the
  runner, and json an `xcode` field in each system object, present only
  when pinned (`schema_version` stays 1). Reports of rows without it
  are unchanged.

### Changed

- `build`: generated main-branch builds now fail when `nix build` fails,
  even if the Cachix push succeeds.
- `build`: Cachix receives only the package output paths printed by `nix
  build`, so realised checks and fetched release archives are not pushed
  to the cache (issue #32).
- `gate`: invalid `cache-push` values are refused instead of enabling
  Cachix writes; only `true` and `false` are accepted.
- `gate`: `cache-push=false` makes a configured Cachix cache
  substituter-only, so the gate pushes nothing and requires no Cachix
  write token while the existing default remains unchanged (issue #33).
- BREAKING: depends on pipelith 0.2 (its PR #16, T38; paradigm T19,
  V28), where `x86_64-darwin` is a CI system and `-` in `systems` means
  all four. actlith maps it to `macos-15-intel`, GitHub's last Intel
  image, available until August 2027 (src/system R18). After a consumer
  takes this release, every `gate` or `build` row written with `-`
  gains an Intel macOS job, and a row listing `x86_64-darwin`, refused
  before, renders. Files consumers see change:
  - `gate` with `-`: the matrix comment and `os` list gain
    `x86_64-darwin` and `macos-15-intel`, so a new check
    `gate (macos-15-intel)` appears. It is not required until you add it
    to branch protection.
  - `build` with `-`: the `include` list gains
    `{os: macos-15-intel, system: x86_64-darwin}`, `verify-cache.sh`
    is given `x86_64-darwin` too, and `check` no longer runs `nix eval
    --raw .#packages.x86_64-darwin.<package>.drvPath` or says systems
    without a runner are evaluated: every system has one now.
  - this repository's own `ci.yml` gains the Intel job, and the README
    badges list Intel macOS. Its dev shell is pklith's on x86_64-darwin
    too (pklith #103), with microlith 0.7.5, itok 0.3.2, xenolith 0.2.0
    and rekall 1.0.0, whose tags ship an Intel macOS package, and
    characterminator 0.1.1, whose tag does not, built from source. Its
    row sets `timeout=90` until the cache holds those builds.

  A row that lists its systems is unchanged. pipelith 0.2.0 comes from
  crates.io.
- `import`: a `build` file's `package` is read up to the end of its
  attribute name, so the one-line build command (`| cachix push`) still
  names it now that the eval-only step that also named it is gone. A
  system written as an expression (`.#packages.${{ matrix.system }}.hk`)
  no longer hides the package behind its own dot.
- `update-pins.sh` resolves nixpkgs through the root's `nixpkgs-lock`
  input, so renamed lock nodes no longer hide available bumps (issue #22).
- Depends on [`pipelith`](https://github.com/pr0d1r2/pipelith) 0.1.0 from
  crates.io, the forge-neutral core shared with lablith. Nothing uses it
  yet; generated files are unchanged (T49).
- `src/registry`: `.actlith` is parsed by pipelith's grammar (T64). Rows,
  lines and messages are unchanged except a newer `format` line, which
  now reads "`.actlith` is format 2; this build reads format 1" with the
  fix "upgrade to a release that reads format 2".
- `src/kind`: `## workflows` rows are checked by pipelith's `kind` (T65):
  its catalogue gives each kind's args and default systems, and its
  `Row::check` checks systems, cache, args, `timeout` and the row rules
  (cache for `build`, `package`, `cron`, `token`, plain words, one
  system for a bump kind) before a workflow is built. Generated files
  are unchanged. A system without a runner still reads "has no
  GitHub-hosted runner". Two refusals read differently:
  - a bad `token` now has the fix "write `token=<SECRET>`: letters,
    digits and `_`, not starting with a digit" (it lacked the last
    clause);
  - a row with more than one fault names the first in pipelith's order
    (systems, cache, args, `timeout`, then the rules), so it may name a
    different fault first than before.

## [0.1.0] - 2026-10-17

### Added

- nix: the flake's `packages.<system>.default` is actlith
  (`nix run github:pr0d1r2/actlith`), built from the crate alone and
  tested as built on a fixture repository (T32).
- `src/cli`: the `actlith` command: `gen [--check]`, `check`, `detect`,
  `seed --init`, `import FILE...`, `report [--format text|json|md]`,
  `pin [--dry-run]`, `completions <shell>` and `man`, with `--root DIR`
  and `--help` examples; exit 0 ok, 1 finding, 2 usage, unreadable input
  or refused row, every diagnostic naming its fix (T30, V34).
- `src/gen`: `gen` writes each rendered file under the root, only where
  its bytes differ; `gen --check` names each missing or stale file and
  writes nothing; a path leaving the root is refused (T24, V6).
- `src/check`: totality over `.github/workflows/` and the bot config
  files: an undeclared workflow, a hand-written path that is missing,
  also generated or without a reason, and a bot file with no `## bots`
  row each name the path and a fix (T25, V7, `src/bots` V38).
- `src/report`: `report` as text, JSON (`schema_version` 1) or Markdown:
  each workflow's kind, systems with runner labels, cache, pinned
  actions and secrets, the hand-written files and the GitHub Apps the
  `## bots` rows assume, all sorted (T28, V33).
- `src/import`: a workflow file becomes a `## workflows` row only when
  a kind's render of that row matches it after normalising comments
  and whitespace; otherwise the first differing line and key of the
  closest render are named (T26, V17).
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

### Changed

- `src/detect`: `detect` and `seed --init` propose `update-pins` commented
  out, under `# off by default: uncomment to get daily nixpkgs-lock bump
  PRs`, so a seeded repository runs no scheduled bump workflow until it
  opts in (T54, V50). A seeded `.actlith` now holds `ci` only.
