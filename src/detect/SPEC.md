# SPEC

## §G GOAL

`src/detect` — `detect` & `seed --init`: proposed rows from tracked files.

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: registry, runners, pins, YAML writer, kinds, render, verbs, CLI
self|src/detect|`detect` & `seed --init`: rows from tracked files
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
sib|src/report|`report` text, json, md: workflows × systems × pins, secrets
sib|src/bots|reviewer-bot config files (`.coderabbit.yaml`, `greptile.json`, `socket.yml`) from `## bots`
sib|src/fetch|GitHub API tag → SHA for `pin`; ONLY network code
sib|src/cli|clap verbs, flags, exit codes, completions, man page

## §C CONSTRAINTS

- input = tracked file list + `flake.nix` / `flake.lock` text. ⊥ nix eval. rows printed in fixed order.

## §I INTERFACES

- fns: `detect(&[path], flake_lock: Option<&str>) -> Vec<WorkflowRow>` (ids `ci` for gate, `update-pins`); `seed_init(root, rows)` writes `.actlith` ⊥ over existing (`src/cli` lists files & reads `flake.lock` ∴ node stays pure); `SeedError::code()` 1 exists | 2 I/O.

## §V INVARIANTS

V32: `seed --init` ⊥ overwrites an existing `.actlith`; exit 1 naming it

## §T TASKS

id|status|task|cites
T27|x|`detect` (`flake.nix` → `gate`, `nixpkgs-lock` input → `update-pins`) + `seed --init`|V32,`.:V1`

## §B BUGS

id|date|cause|fix
