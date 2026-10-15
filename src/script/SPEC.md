# SPEC

## §G GOAL

`src/script` — seeded shell scripts shipped in the crate, written by `gen` with header.

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: registry, runners, pins, YAML writer, kinds, render, verbs, CLI
self|src/script|seeded shell scripts shipped in the crate (`update-pins.sh`, `open-pr.sh`) & their bats
sib|src/registry|parse `.actlith`: format line, `## workflows`, `## actions`, `## hand-written`; row errors
sib|src/system|nix system → runner label table
sib|src/yaml|workflow model types & own YAML writer: key order, quoting, comments, header
sib|src/pin|pin table: built-in pins + `## actions` overrides; `uses:` lines
sib|src/kind|kind trait, arg schemas, common parts: permissions, concurrency, timeouts, checkout, install-nix, cachix, secrets
sib|src/gate|kind `gate`: hk gate per system
sib|src/build|kind `build`: build, flake check, verify-cache
sib|src/bump|kinds `update-pins` & `upstream`: scheduled bump PRs
sib|src/render|`.actlith` rows → ordered set of files (path, bytes)
sib|src/gen|`gen` & `gen --check`: write or compare rendered files
sib|src/check|`check`: totality over `.github/workflows/`
sib|src/import|`import`: workflow file → row by render-and-compare
sib|src/detect|`detect` & `seed --init`: rows from tracked files
sib|src/report|`report` text, json, md: workflows × systems × pins, secrets
sib|src/fetch|GitHub API tag → SHA for `pin`; ONLY network code
sib|src/cli|clap verbs, flags, exit codes, completions, man page

## §C CONSTRAINTS

- scripts live as real files `src/script/*.sh` (shellcheck, shfmt, bats see them), embedded w/ `include_str!`. ⊥ shell text in Rust strings.

## §I INTERFACES

- fns: `scripts() -> &[(&'static str, &'static str)]` (published path `scripts/actlith/<name>.sh`, bytes w/ header).

## §V INVARIANTS

V13: ∀ emitted `run:` = 1 line. logic > 1 cmd → seeded script `scripts/actlith/<name>.sh` (shellcheck, bats, xenolith see it)

## §T TASKS

id|status|task|cites
T21|.|`update-pins.sh` (update input, build, branch `pins/nixpkgs-<short>`, PR; idempotent) & `open-pr.sh`; bats tests FIRST (tdd-order mirror); embedded w/ header|V13,`src/yaml:V8`,`src/bump:V26`

## §B BUGS

id|date|cause|fix
