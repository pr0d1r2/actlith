# SPEC

## §G GOAL

`src/gen` — `gen` writes rendered files; `gen --check` proves they are current.

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: registry, runners, pins, YAML writer, kinds, render, verbs, CLI
self|src/gen|`gen` & `gen --check`: write or compare rendered files
sib|src/core|facade: ONE node naming `pipelith`; re-exports its neutral modules
sib|src/registry|parse `.actlith`: format line, `## workflows`, `## actions`, `## hand-written`; row errors
sib|src/system|nix system → runner label table
sib|src/yaml|workflow model types & own YAML writer: key order, quoting, comments, header
sib|src/script|seeded shell scripts shipped in the crate (`update-pins.sh`, `open-pr.sh`) & their bats
sib|src/pin|pin table: built-in pins + `## actions` overrides; `uses:` lines
sib|src/kind|kind trait, arg schemas, common parts: permissions, concurrency, timeouts, checkout, install-nix, cachix, secrets
sib|src/gate|kind `gate`: hk gate per system
sib|src/build|kind `build`: build, flake check, verify-cache
sib|src/bump|kinds `update-pins` & `upstream`: scheduled bump PRs
sib|src/render|`.actlith` rows → ordered set of files (path, bytes)
sib|src/check|`check`: totality over `.github/workflows/`
sib|src/import|`import`: workflow file → row by render-and-compare
sib|src/detect|`detect` & `seed --init`: rows from tracked files
sib|src/report|`report` text, json, md: workflows × systems × pins, secrets
sib|src/bots|reviewer-bot config files (`.coderabbit.yaml`, `greptile.json`, `socket.yml`) from `## bots`
sib|src/fetch|GitHub API tag → SHA for `pin`; ONLY network code
sib|src/cli|clap verbs, flags, exit codes, completions, man page

## §C CONSTRAINTS

- only node besides `src/fetch`-driven `pin` that writes files: rendered paths ONLY, under `--root`.
- write, compare, path check = pipelith `files` via `src/core`; `File` = `core::files::File` (render & bots too).

## §I INTERFACES

- fns: `r#gen(root, &[File]) -> io::Result<Vec<path>>`, `check(root, &[File]) -> io::Result<Vec<Stale>>` (`Stale::{Missing, Changed}`; `Err` = path escapes root | unreadable → exit 2). path check textual ∴ a symlinked dir inside root may still point outside.

## §V INVARIANTS

V6: `gen --check` writes ⊥; ∃ stale | missing generated file → exit 1 naming each

## §T TASKS

id|status|task|cites
T24|x|`gen` + `gen --check` over a temp root fixture|V6,`.:V1`
T61|x|`gen`, `check`, `target`, `File` → `src/core` files; goldens unchanged|V6,`tests:V48`,`src/core:V51`

## §B BUGS

id|date|cause|fix
