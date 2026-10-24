# SPEC

## §G GOAL

`src/system` — nix system → GitHub-hosted runner label (1 table) | `## runners` labels.

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: registry, runners, pins, YAML writer, kinds, render, verbs, CLI
self|src/system|nix system → runner label table
sib|src/core|facade: ONE node naming `pipelith`; re-exports its neutral modules
sib|src/registry|parse `.actlith`: format line, `## workflows`, `## actions`, `## hand-written`; row errors
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
sib|src/cli|clap verbs, flags, exit codes, completions, man page

## §C CONSTRAINTS

- `System`, parse & defaults from `src/core` (pipelith); this node = GitHub runner table (`const fn` match ∴ total), `host`, name error. 1 table. ⊥ runner label literal anywhere else in the crate.

## §I INTERFACES

- types: `System` (enum of 4 nix systems), `SystemError` (`Unknown` only), `Host::{Hosted(label), Own(labels)}`; `Runners` = pipelith's.
- fns: `parse_system(&str) -> Result<System, SystemError>`, `runner(System) -> &'static str` (∀ 4), `host(&Runners, System) -> Host` = `Runners::pick(system, true)` (∀ has default ∴ ⊥ refusal), `DEFAULT_SYSTEMS`.

## §R RESEARCH

id|topic|finding|src
R8|runners|`ubuntu-latest` x86_64, `ubuntu-24.04-arm` aarch64 linux, `macos-latest` aarch64 darwin, `macos-15-intel` x86_64 darwin|pklith `ci.yml`, nix-hk `build.yml` comment
R18|Intel macOS|`macos-15-intel` = GitHub's LAST x86_64 image, until 2027-08. RISK: then ⊥ hosted x86_64-darwin → `## runners` row \| drop system|runner-images #13045

## §V INVARIANTS

V15: system → runner mapping in ONE table; ⊥ runner label literal elsewhere in code
V53: ∀ `System` ∈ table; `## runners` row → `Own` over label; `Own` ⇒ fork-PR guard (`src/kind:V54`)

## §T TASKS

id|status|task|cites
T14|x|`System` enum, parse, runner table; refuse `x86_64-darwin` naming hand verification on Intel Mac|V15,`src/registry:V10`
T60|x|`System`, `parse_system`, `DEFAULT_SYSTEMS` → `src/core` (pipelith); keep `runner` & "GitHub-hosted runner" error; goldens unchanged|V15,`tests:V48`,`src/core:V51`
T72|.|`x86_64-darwin` → `macos-15-intel`; `host` via `Runners::pick` (pipelith 0.2, T38)|V15,V53,R18

## §B BUGS

id|date|cause|fix
