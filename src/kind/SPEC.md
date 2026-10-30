# SPEC

## §G GOAL

`src/kind` — what every kind shares: arg schema, hardening, common steps, secrets.

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: registry, runners, pins, YAML writer, kinds, render, verbs, CLI
self|src/kind|kind trait, arg schemas, common parts: permissions, concurrency, timeouts, checkout, install-nix, cachix, secrets
sib|src/core|facade: ONE node naming `pipelith`; re-exports its neutral modules
sib|src/registry|parse `.actlith`: format line, `## workflows`, `## actions`, `## hand-written`; row errors
sib|src/system|nix system → runner label table
sib|src/yaml|workflow model types on pipelith YAML writer: key order, quoting, comments, header
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
sib|src/bots|reviewer-bot config files (`.coderabbit.yaml`, `greptile.json`, `socket.yml`) from `## bots`
sib|src/fetch|GitHub API tag → SHA for `pin`; ONLY network code
sib|src/cli|clap verbs, flags, exit codes, completions, man page

## §C CONSTRAINTS

- a kind = Rust type implementing `Kind`: `spec()` (pipelith catalogue entry), secrets, `build(&Row, &PinTable) -> Workflow`. ⊥ text templates.
- row, args, cache = pipelith `kind`; here: runner wording + `EXTRAS` (kinds ∅, keys `xcode`, `cache-push`); `check` passes a kind only its `Kind::forge_args` (default ∅) ∴ other kinds refuse these keys.
- common: permissions, concurrency, timeout, checkout, install-nix/cache, cachix (fork-PR skip, `pushFilter`); gate `cache-push=false`: read-only.

## §I INTERFACES

- types: `Kind` trait (+ `forge_args`), `Secret`, `KindError` (`Row` ← pipelith's | `Pin`); `Row`, `Args`, `Cache` = pipelith's.
- fns: `check(&dyn Kind, &WorkflowRow) -> Result<Row, KindError>`: systems known (∀ has a host, `src/system:V53` ∴ ⊥ refused at render), then pipelith `Row::check` (keys, cache, timeout 1..=360, rules → error naming line); builders flat (⊥ `common` module).
- runners: `Kind::build(&Row, &PinTable, &Runners)`; `hosts(&Row, &Runners) -> Hosts` splits systems hosted | own; `own(Job, labels)` → `runs-on: [labels]` + guard `if:` (V54).
- cache: public keys from pipelith's built-in table (v1: `pr0d1r2` only); other cache names refused w/ fix hint, ⊥ guessed key. `## caches` (`name|public-key`) = T44.

## §R RESEARCH

id|topic|finding|src
R19|own runner shape|job `if:` ⊥ sees `matrix` (evaluated before expansion; actionlint refuses) ∴ 1 matrix job ⊥ guards 1 entry; `include` list `os` hides labels behind `${{ matrix.os }}` ∴ own runner = own job `<job>-<system>`, literal `runs-on`|GitHub docs contexts availability; actionlint
R42|adoption|wf: CodeQL, Scorecard, release-plz, CodSpeed, Bencher, Codecov, Semantic PR, stale, lychee, trusted-publishing; config-only: Dependabot, Renovate, Mergify; API: Secret scanning/push protection; Harden-Runner, attestations, cache-nix-action, flake-checker-action; merge queue; cache:C,T17|`docs/SERVICES.md` Adopt

## §V INVARIANTS

V3: hardening ⊥ optional, ∀ kind: top `permissions: contents: read` (job widens only declared need) · checkout `persist-credentials: false` unless job pushes · `concurrency` group `<id>-${{ github.ref }}` (cancel for push/PR kinds, ⊥ for scheduled) · `timeout-minutes` ∀ job · ⊥ `pull_request_target` · ⊥ `${{ }}` inside `run:` (values pass via `env:`)
V9: job names stable across versions (`pklith protect` reads them); rename = breaking change + CHANGELOG entry
V14: ∀ kind declares ∀ secret it reads; `report` lists them per workflow. fork PR needs ⊥ secret (cachix push skipped, read-only cache works)
V49: kind ∈ pipelith catalogue only if it renders on both forges, else forge-only & named by portability check. refines `.:V24`
V54: job on `Host::Own` → `runs-on: [labels]` & `if: github.event_name != 'pull_request' || github.event.pull_request.head.repo.full_name == github.repository` (`&&` job's own `if`); hosted job ⊥ guard; ⊥ opt-out (pipelith `runner` `trusted_only`). golden has both, passes actionlint & zizmor
V58: adopted integrations are typed: workflows stay hand-written until ≥2 fleet repos share a shape (`.:V24`); steps use pinned `Step`, triggers extend `Trigger`, and kinds declare permissions/secrets (`V3,V14`)
V59: secret scanning/push protection: settings/API; Cachix: C,T17

## §T TASKS

id|status|task|cites
T17|x|`Kind` trait, arg schema & parse, common parts (permissions, concurrency, timeouts, checkout, install-nix, cachix, env-only expressions); tests per part|V3,V14,`src/registry:V10`
T42|x|adopted contracts; build after `.:V24`|`.:V24`,`.:V41`,V58,V59,R42
T44|x|`.actlith` `## caches` section (columns name, public-key; `src/registry`) so a repo names its own cachix cache; built-in table stays the default|`src/registry:V10`
T51|x|rows, args, contract → `pipelith` `kind`; mark forge-only arg keys|`tests:V48`,V49
T53|.|split `docs/KINDS.md`: neutral → pipelith, GitHub syntax → `docs/KINDS-GITHUB.md`|`.:V25`,V49
T65|x|rows, args, cache, catalogue → pipelith `kind`; `Kind` = spec + secrets + build; runner wording kept; empty `Extras`|V49,`tests:V48`,`src/core:V51`
T74|x|`build` takes `Runners`; `hosts`, `own` + guard; ∀ kind renders own-runner jobs (R19)|V54,`src/system:V53`,`tests:V4`

## §B BUGS

id|date|cause|fix
