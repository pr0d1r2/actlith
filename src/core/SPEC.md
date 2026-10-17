# SPEC

## §G GOAL

`src/core` — facade: the ONE node naming `pipelith`; re-exports its neutral modules for the other nodes.

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: registry, runners, pins, YAML writer, kinds, render, verbs, CLI
self|src/core|facade: ONE node naming `pipelith`; re-exports its neutral modules
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
sib|src/gen|`gen` & `gen --check`: write or compare rendered files
sib|src/check|`check`: totality over `.github/workflows/`
sib|src/import|`import`: workflow file → row by render-and-compare
sib|src/detect|`detect` & `seed --init`: rows from tracked files
sib|src/report|`report` text, json, md: workflows × systems × pins, secrets
sib|src/bots|reviewer-bot config files (`.coderabbit.yaml`, `greptile.json`, `socket.yml`) from `## bots`
sib|src/fetch|GitHub API tag → SHA for `pin`; ONLY network code
sib|src/cli|clap verbs, flags, exit codes, completions, man page

## §C CONSTRAINTS

- re-export only, ⊥ logic, ⊥ adapter (adapters live in the node that needs one).
- surface = `pipelith::{registry, kind, system, yaml, files, detect}`, added module by module as each node switches (`tests:T50`).

## §I INTERFACES

- mods: `core::{registry, kind, system, yaml, files, detect}` = `pipelith` modules of same name.

## §V INVARIANTS

V51: ⊥ `pipelith::` path outside `src/core`; ∀ other node reaches pipelith through `crate::core`

## §T TASKS

id|status|task|cites
T59|.|node + `system` re-export (1st switch), then 1 module per switch PR|V51,`tests:V48`

## §B BUGS

id|date|cause|fix
