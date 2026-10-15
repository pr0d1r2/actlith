# SPEC

## §G GOAL

`src/kind` — what every kind shares: arg schema, hardening, common steps, secrets.

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: registry, runners, pins, YAML writer, kinds, render, verbs, CLI
self|src/kind|kind trait, arg schemas, common parts: permissions, concurrency, timeouts, checkout, install-nix, cachix, secrets
sib|src/registry|parse `.actlith`: format line, `## workflows`, `## actions`, `## hand-written`; row errors
sib|src/system|nix system → runner label table
sib|src/yaml|workflow model types & own YAML writer: key order, quoting, comments, header
sib|src/script|seeded shell scripts shipped in the crate (`update-pins.sh`, `open-pr.sh`) & their bats
sib|src/pin|pin table: built-in pins + `## actions` overrides; `uses:` lines
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

- a kind = Rust type implementing `Kind`: name, arg schema, default systems, secrets, `build(&Row, &PinTable) -> Workflow`. ⊥ text templates.
- common parts here, used by every kind: top permissions, concurrency, job timeout, checkout, install-nix w/ cache substituter, cachix (fork-PR skip, `pushFilter`).

## §I INTERFACES

- types: `Kind` trait, `ArgSchema` (key, required, default), `Args`, `Secret`, `KindError`.
- fns: `parse_args(&ArgSchema, &str) -> Result<Args, KindError>` (unknown key → error), `common::*` builders.

## §V INVARIANTS

V3: hardening ⊥ optional, ∀ kind: top `permissions: contents: read` (job widens only declared need) · checkout `persist-credentials: false` unless job pushes · `concurrency` group `<id>-${{ github.ref }}` (cancel for push/PR kinds, ⊥ for scheduled) · `timeout-minutes` ∀ job · ⊥ `pull_request_target` · ⊥ `${{ }}` inside `run:` (values pass via `env:`)
V9: job names stable across versions (`pklith protect` reads them); rename = breaking change + CHANGELOG entry
V14: ∀ kind declares ∀ secret it reads; `report` lists them per workflow. fork PR needs ⊥ secret (cachix push skipped, read-only cache works)

## §T TASKS

id|status|task|cites
T17|.|`Kind` trait, arg schema & parse, common parts (permissions, concurrency, timeouts, checkout, install-nix, cachix, env-only expressions); tests per part|V3,V14,`src/registry:V10`

## §B BUGS

id|date|cause|fix
