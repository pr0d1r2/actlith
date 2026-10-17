# SPEC

## §G GOAL

`src/import` — `import`: an existing workflow becomes a `.actlith` row only on exact match.

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: registry, runners, pins, YAML writer, kinds, render, verbs, CLI
self|src/import|`import`: workflow file → row by render-and-compare
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
sib|src/gen|`gen` & `gen --check`: write or compare rendered files
sib|src/check|`check`: totality over `.github/workflows/`
sib|src/detect|`detect` & `seed --init`: rows from tracked files
sib|src/report|`report` text, json, md: workflows × systems × pins, secrets
sib|src/bots|reviewer-bot config files (`.coderabbit.yaml`, `greptile.json`, `socket.yml`) from `## bots`
sib|src/fetch|GitHub API tag → SHA for `pin`; ONLY network code
sib|src/cli|clap verbs, flags, exit codes, completions, man page

## §C CONSTRAINTS

- render-and-compare first: candidate rows from `detect`-style guesses × kinds, rendered, normalised (comments, whitespace) & compared. YAML reader crate only if research T26 shows render-and-compare insufficient (`.:C`).

## §I INTERFACES

- fns: `import(text) -> Result<WorkflowRow, Mismatch>`; `Mismatch` names first differing key.

## §R RESEARCH

id|topic|finding|src
R16|sweep|214 fleet wf: 3 match; ⊥ order, quotes: 3 ∴ ⊥ YAML crate|local

## §V INVARIANTS

V17: `import` accepts only match of a kind+args render after normalising comments & whitespace; ⊥ match → refuse naming first differing key. ⊥ guess, ⊥ partial row

## §T TASKS

id|status|task|cites
T26|x|research sweep over fleet copies (`.:R1`): does render-and-compare match after normalising? record §R here; then `import`|V17

## §B BUGS

id|date|cause|fix
