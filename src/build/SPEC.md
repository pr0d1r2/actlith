# SPEC

## §G GOAL

`src/build` — kind `build`: build & cache a flake package per system, prove the cache has it.

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: registry, runners, pins, YAML writer, kinds, render, verbs, CLI
self|src/build|kind `build`: build, flake check, verify-cache
sib|src/registry|parse `.actlith`: format line, `## workflows`, `## actions`, `## hand-written`; row errors
sib|src/system|nix system → runner label table
sib|src/yaml|workflow model types & own YAML writer: key order, quoting, comments, header
sib|src/script|seeded shell scripts shipped in the crate (`update-pins.sh`, `open-pr.sh`) & their bats
sib|src/pin|pin table: built-in pins + `## actions` overrides; `uses:` lines
sib|src/kind|kind trait, arg schemas, common parts: permissions, concurrency, timeouts, checkout, install-nix, cachix, secrets
sib|src/gate|kind `gate`: hk gate per system
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

- shape = nix-hk `build.yml` (`.:R3`), SHA-pinned.

## §I INTERFACES

- kind name `build`; jobs `build` (matrix, `nix build .#packages.<system>.<package>`, push only from `main`), `check` (`nix flake check --all-systems`, eval-only for systems w/o runner), `verify-cache` (needs `build`, `main` push only); args `package` (required), `timeout`.

## §R RESEARCH

id|topic|finding|src
R4|cachix silent fail|cachix push failure logs ⊥ fails job (403 → green over empty cache) ∴ nix-hk `verify-cache` job asks cache per system after build|nix-hk `build.yml` `verify-cache` comment

## §V INVARIANTS

V30: `verify-cache` asks the cache for ∀ built system's output path & fails on ≠ 200; its logic is a seeded script (`src/script`), ⊥ inline shell

## §T TASKS

id|status|task|cites
T20|x|kind `build` (jobs `build` `check` `verify-cache`) + `verify-cache.sh` in `src/script`|V30,R4,`src/kind:V3`,`src/kind:V9`

## §B BUGS

id|date|cause|fix
