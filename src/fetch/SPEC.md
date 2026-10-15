# SPEC

## §G GOAL

`src/fetch` — GitHub API tag → commit SHA for `actlith pin`; the only network code.

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: registry, runners, pins, YAML writer, kinds, render, verbs, CLI
self|src/fetch|GitHub API tag → SHA for `pin`; ONLY network code
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
sib|src/cli|clap verbs, flags, exit codes, completions, man page

## §C CONSTRAINTS

- HTTP via `gh api` subprocess or 1 small client crate, decided in research (§R) before code; behind a trait ∴ tests use a fake.

## §I INTERFACES

- trait `Resolver { fn sha(&self, action, tag) -> Result<String, FetchError> }`; `pin(root, dry_run)` rewrites `## actions` only.

## §V INVARIANTS

V16: `pin` = sole networked path; writes only `## actions` of `.actlith`; `--dry-run` writes ⊥. ⊥ other verb links network code

## §T TASKS

id|status|task|cites
T29|.|research client (`gh api` vs crate) → §R; `Resolver` + fake; `pin` + `--dry-run`|V16

## §B BUGS

id|date|cause|fix
