# SPEC

## §G GOAL

`src/` — hub. code nodes of `actlith`. ⊥ own logic itself.

## §F FEDERATION

dir|owns|⊥owns|tokens
registry|parse `.actlith`: format line, `## workflows`, `## actions`, `## hand-written`; row errors|what a kind accepts|-
system|nix system → runner label table|kind shapes|-
yaml|workflow model types & own YAML writer: key order, quoting, comments, header|what any workflow says|-
script|seeded shell scripts shipped in the crate (`update-pins.sh`, `open-pr.sh`) & their bats|when a kind needs them|-
pin|pin table: built-in pins + `## actions` overrides; `uses:` lines|resolving tags (network)|-
kind|kind trait, arg schemas, common parts: permissions, concurrency, timeouts, checkout, install-nix, cachix, secrets|any one kind's jobs|-
gate|kind `gate`: hk gate per system|other kinds|-
build|kind `build`: build, flake check, verify-cache|other kinds|-
bump|kinds `update-pins` & `upstream`: scheduled bump PRs|seeded script text|-
render|`.actlith` rows → ordered set of files (path, bytes)|writing files|-
gen|`gen` & `gen --check`: write or compare rendered files|rendering|-
check|`check`: totality over `.github/workflows/`|rendering|-
import|`import`: workflow file → row by render-and-compare|writing files|-
detect|`detect` & `seed --init`: rows from tracked files|rendering|-
report|`report` text, json, md: workflows × systems × pins, secrets|rendering|-
bots|reviewer-bot config files (`.coderabbit.yaml`, `greptile.json`, `socket.yml`) from `## bots`|installing bot apps|-
fetch|GitHub API tag → SHA for `pin`; ONLY network code|pin table merge|-
cli|clap verbs, flags, exit codes, completions, man page|every verb's logic|-

## §N NAV

rel|path|lens
up|.|-
self|src|the tool: registry, runners, pins, YAML writer, kinds, render, verbs, CLI
sib|tests|integration tests driving `actlith` binary, actionlint & zizmor harness, golden workflows, fixtures
sib|dev|`actlith-dev`, `publish = false`: commit rules, README badges, notices, AGENTS step table & their drift checks
sib|nix|repo gate: dev shell, guardrails, hk steps, CI, release, dogfood

## §C CONSTRAINTS

- module = dir + `mod.rs`. `mod.rs` composes, ⊥ implements.
- ∀ external dep ! ONE facade node: `clap` → `src/cli` · HTTP client (if any) → `src/fetch`.
- dep order (waves): `registry`, `system`, `yaml`, `script` → `pin`, `detect`, `bots` → `kind`, `fetch` → `gate`, `build`, `bump` → `render` → `gen`, `check`, `import`, `report` → `cli`. edges = `use crate::<node> as _;` lines in each `mod.rs`, 1 per line (sherd reads 1st line only).

## §V INVARIANTS

V1: `main.rs` & `lib.rs` dispatch & re-export only, ⊥ logic.
V2: node ⊥ dep sibling node except through its public surface.
V3: lib API = `lib.rs` re-exports ONLY, ∀ node `mod` private.
V4: ∀ node unit-tested 1-to-1 (`<node>/tests.rs`, `rust-mirror`); fixtures in `tests/fixtures/<node>/`; ⊥ network, ⊥ clock in unit tests.

## §T TASKS

id|status|task|cites
T1|.|`lib.rs` re-exports each node's public surface as it lands|V1,V3

## §B BUGS

id|date|cause|fix
