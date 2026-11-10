# Changelog

All notable changes to actlith are recorded here, in the
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format. The
project follows [Semantic Versioning](https://semver.org/): a change to
the bytes actlith emits for the same inputs is at least a minor version
(SPEC.md V18), and its entry names the files consumers will see change.

## [Unreleased]

## [0.2.0] - 2026-11-10

What a consumer sees change: actlith now reads pipelith 0.2's `.actlith`
sections `## caches` (cachix or https caches, public or private) and
`## runners` (self-hosted runners), and pins an Xcode on macOS `gate`
rows. Every `gate` or `build` row written with `-` gains an Intel macOS
(`macos-15-intel`) job, so its rendered files change on the next
`actlith gen` (BREAKING, below). `build` and `gate` push less to Cachix
and fail when `nix build` does, so `build` workflows on a cache change
too. The new sections and the `xcode` arg change nothing for files that
do not use them.

### Added

- `## caches` in `.actlith`, pipelith 0.2.1's neutral section with
  header `name|kind|url|key|push` (T79-T82): a `## workflows` row's
  `cache` cell names a `cachix` or `https` cache there, each of `url`,
  `key` and `push` a literal or `secret:NAME`. It replaces the
  unreleased `name|public-key` header, which is now refused.
  - Reading: install-nix's `extra_nix_config` gains the cache's
    `extra-substituters` and `extra-trusted-public-keys`. A secret value
    is written as `${{ secrets.NAME }}` in an install-nix step that skips
    fork pull requests, beside a cache-less one that runs only for them,
    so a fork never gets an empty substituter.
  - Pushing: a read-only cache (`push` is `-`) gets no cachix step and
    no push step. A cachix row's token secret is the cachix step's
    `authToken`. An https cache pushes with the new seeded
    `scripts/actlith/push-https.sh`, given `NETRC` and `URL` through
    `env:`: it masks the url before any output, skips a run without the
    netrc, writes it mode 600 to `$RUNNER_TEMP/netrc`, removes it on
    exit, and runs `nix copy --option netrc-file ... --to "$URL"`. `gate`
    pushes `devShells.default.inputDerivation` (skipping fork PRs, and
    never with `cache-push=false`), `build` pushes `packages.<package>`
    from `main` only. Each output is checked first, and each
    installable's `drvPath` forced: a flake without it, such as one with
    no `devShells.<system>.default` under a gate, fails before the netrc
    is written, with an `::error::` naming the missing output and the
    fix.
  - Both https scripts hand the netrc to nix or curl only after a
    run-time check that the resolved url, literal or secret, starts with
    `https://` (pipelith V87). Otherwise they stop before writing it,
    and the `::error::` names the cache and the netrc secret, which the
    step passes as `CACHE` and `NETRC_SECRET`, never the url.
  - `build` on an https cache: `verify-cache` runs the new seeded
    `scripts/actlith/verify-https.sh`, which asks `<url>/<hash>.narinfo`,
    with the cache's read netrc when it has one (below), else the push
    netrc when set. A miss names the secret it asked with, from
    `NETRC_SECRET`, or says no netrc was used.
  - `actlith report` lists the cache's url and key secrets as
    `cache read` and its push secret as `cachix push` or
    `cache push (netrc)`.
  - A private https cache, one that answers only with credentials
    (pipelith 0.2.2): `read=secret:R` in the `push` cell, beside
    `push=secret:P` or alone, names the netrc nix reads it with
    (T85). Every kind's workflow then runs the new seeded
    `scripts/actlith/read-netrc.sh` before install-nix, skipping fork
    pull requests: it masks the url, applies the run-time `https://`
    check, and writes the netrc mode 600 to `$RUNNER_TEMP/read-netrc`,
    which stays for the job's later steps. The trusted install-nix
    step's `extra_nix_config` gains
    `netrc-file = ${{ runner.temp }}/read-netrc`; a fork pull request
    runs without the cache. The push keeps its own netrc and file, and
    `verify-https.sh` asks the cache with the read netrc when there is
    one. `actlith report` lists the read secret as `cache read (netrc)`.
  The cache feature alone changes nothing for `.actlith` files without
  `## caches`, including every workflow on the built-in `pr0d1r2` cache.
  Those files still change in this release where a `gate` or `build` row
  is written with `-`: it gains the Intel macOS job (BREAKING, below).

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

- Xcode-pinned `gate` workflows pass `HOST_DEVELOPER_DIR` into the default
  `nix develop` command, preserving the selected Xcode in the dev shell.
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
- Depends on [`pipelith`](https://github.com/pr0d1r2/pipelith) 0.2.2 from
  crates.io, the forge-neutral core shared with lablith; `src/registry`
  parses `.actlith` with it (T49, and the next entry).
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
