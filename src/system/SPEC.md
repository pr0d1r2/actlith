# SPEC

## §G GOAL

`src/system` — nix system name → GitHub-hosted runner label, in one table.

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: registry, runners, pins, YAML writer, kinds, render, verbs, CLI
self|src/system|nix system → runner label table
sib|src/registry|parse `.actlith`: format line, `## workflows`, `## actions`, `## hand-written`; row errors
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

- zero deps. 1 `const` table. ⊥ runner label literal anywhere else in the crate.

## §I INTERFACES

- types: `System` (enum of 4 nix systems), `SystemError`.
- fns: `parse_system(&str) -> Result<System, SystemError>`, `runner(System) -> Result<&'static str, SystemError>` (`x86_64-darwin` → refused, ⊥ runner), `DEFAULT_SYSTEMS`.

## §R RESEARCH

id|topic|finding|src
R8|runners|`ubuntu-latest` x86_64, `ubuntu-24.04-arm` aarch64 linux, `macos-latest` aarch64 darwin; ⊥ hosted x86_64-darwin going forward (macos-13 retiring)|pklith `ci.yml`, nix-hk `build.yml` comment

## §V INVARIANTS

V15: system → runner mapping in ONE table; ⊥ runner label literal elsewhere in code

## §T TASKS

id|status|task|cites
T14|x|`System` enum, parse, runner table; refuse `x86_64-darwin` naming hand verification on Intel Mac|V15,`src/registry:V10`

## §B BUGS

id|date|cause|fix
