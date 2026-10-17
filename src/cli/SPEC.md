# SPEC

## §G GOAL

`src/cli` — clap verbs, flags and exit codes over the verb nodes.

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: registry, runners, pins, YAML writer, kinds, render, verbs, CLI
self|src/cli|clap verbs, flags, exit codes, completions, man page
sib|src/core|facade: ONE node naming `pipelith`; re-exports its neutral modules
sib|src/registry|parse `.actlith`: format line, `## workflows`, `## actions`, `## hand-written`; row errors
sib|src/system|nix system → runner label table
sib|src/yaml|workflow model types on pipelith YAML writer: key order, quoting, comments, header
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

## §C CONSTRAINTS

- `clap` (derive) here only; `clap_complete`, `clap_mangen`. verbs dispatch to nodes, ⊥ logic.

## §I INTERFACES

- verbs & exit codes → `.:I`.

## §V INVARIANTS

V34: exit 0 ok · 1 finding · 2 usage | unreadable | unknown kind/key/system | newer format; ∀ error line names the fix

## §T TASKS

id|status|task|cites
T30|x|clap verbs `gen check detect seed import report pin completions man`; exit codes; `--help` w/ examples|V34,`.:I`

## §B BUGS

id|date|cause|fix
