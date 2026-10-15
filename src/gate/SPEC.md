# SPEC

## §G GOAL

`src/gate` — kind `gate`: the hk gate on every CI system (fleet `ci.yml`).

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: registry, runners, pins, YAML writer, kinds, render, verbs, CLI
self|src/gate|kind `gate`: hk gate per system
sib|src/registry|parse `.actlith`: format line, `## workflows`, `## actions`, `## hand-written`; row errors
sib|src/system|nix system → runner label table
sib|src/yaml|workflow model types & own YAML writer: key order, quoting, comments, header
sib|src/script|seeded shell scripts shipped in the crate (`update-pins.sh`, `open-pr.sh`) & their bats
sib|src/pin|pin table: built-in pins + `## actions` overrides; `uses:` lines
sib|src/kind|kind trait, arg schemas, common parts: permissions, concurrency, timeouts, checkout, install-nix, cachix, secrets
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

- shape = pklith `ci.yml` (`.:R3`). triggers push `main`, `pull_request`, `workflow_dispatch`.

## §I INTERFACES

- kind name `gate`; job `gate` (stable); matrix over systems, `fail-fast: false`; checkout `fetch-depth: 0`; args `command` (default `nix develop --command hk check --all --check`), `timeout` (min, default 30); default systems `x86_64-linux aarch64-linux aarch64-darwin`; secret `CACHIX_AUTH_TOKEN` when `cache` set.

## §V INVARIANTS

V29: `gate` render of actlith's own row == this repo's hand-written `.github/workflows/ci.yml` modulo the generated header (golden; dogfood `nix:T31`)

## §T TASKS

id|status|task|cites
T19|.|kind `gate` + golden vs own `ci.yml`|V29,`src/kind:V3`,`src/kind:V9`

## §B BUGS

id|date|cause|fix
