# SPEC

## §G GOAL

`src/bots` — reviewer-bot config files from `.actlith` `## bots`: one central config, a stub per repo.

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: registry, runners, pins, YAML writer, kinds, render, verbs, CLI
self|src/bots|reviewer-bot config files (`.coderabbit.yaml`, `greptile.json`, `socket.yml`) from `## bots`
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
sib|src/fetch|GitHub API tag → SHA for `pin`; ONLY network code
sib|src/cli|clap verbs, flags, exit codes, completions, man page

## §C CONSTRAINTS

- writes CONFIG FILES only. installing a bot's GitHub App = account action by the user, ⊥ actlith; `report` lists which apps a repo's rows assume.
- cost rule (user, 2026-10-15): free tier wherever one exists; paid only when ⊥ free option, & then the SMALLEST plan w/ small amounts. ∴ coderabbit, socket, greptile on free tiers (greptile Starter 1 dev, + OSS application); bugbot only via Cursor Premium $30/mo = smallest paid ∴ ⊥ v1 unless user opts in (R12).
- v1 bots: `coderabbit` (`.coderabbit.yaml` stub w/ `remote_config` → central repo, `inheritance: true`), `greptile` (`greptile.json`: `disabledLabels`, `ignoreKeywords` skip release PRs; free Starter = 1 dev), `socket` (`socket.yml`, optional). app-only bots (Cursor Bugbot) & unpriced ones (Entire) ⊥ v1 (R12).
- same rules as workflows: generated header (where the format has comments), drift via `gen --check`, totality via `check` (bot file present w/o row → finding).

## §I INTERFACES

- file: `.actlith` `## bots` → `bot|args` (e.g. `coderabbit|central=pr0d1r2/coderabbit`).
- fns: `bot_files(&[BotRow]) -> Result<Vec<File>, BotError>`.

## §R RESEARCH

id|topic|finding|src
R9|review bots|hk #1705: greptile, cursor bugbot, coderabbit each caught distinct real bug (GIT_DIR ignored, shared-repo 0700, basename patch collision); author answers each inline w/ fix sha \| reasoned "No change"; coderabbit then stores a repo Learning|<https://github.com/jdx/hk/pull/1705>
R10|coderabbit cfg|`.coderabbit.yaml` = `remote_config` → central repo (`jdx/coderabbit`) + `inheritance: true` ∴ fleet 1-line stub; free forever for public repos|<https://www.coderabbit.ai/pricing>, jdx/hk `.coderabbit.yaml`
R11|bot cfg files|greptile `greptile.json` (`disabledLabels`, `ignoreKeywords` skip release PRs); socket `socket.yml` optional; bugbot app-only here; entire `.entire/settings.json` + `.entire/runners/trail-*.json` haiku scoring prompts|jdx/hk tree @ main
R12|bot pricing (verified 2026-10-15)|coderabbit free public repos, paid Essentials $30 \| Team $60 \| Advanced $72/user/mo (annual $24/$48/$72); socket free plan 1,000 scans/mo + free OSS Team on request, paid Team $25/dev/mo (min 5); greptile Starter free 1 dev 50 credits/mo, Pro $30/seat/mo + $1/credit, free non-commercial OSI OSS by application; bugbot only in Cursor Premium $30 \| Super $100 \| Ultra $200/mo, Teams $30+/user/mo, ⊥ free; entire ⊥ published pricing|coderabbit.ai/pricing, socket.dev/pricing, greptile.com/pricing, cursor.com/pricing, docs.entire.io
R13|jdx wf shapes|hk PR checks: conventional title, link-discussion, perf instruction-count, cargo-deny, pr-closer, draft-limit via reusable `jdx/workflows/...@sha # v1.1.0` = candidate kinds once ≥2 fleet repos need them (`.:V24`)|<https://github.com/jdx/hk/tree/main/.github/workflows>

## §V INVARIANTS

V38: bot files rendered only for `## bots` rows; ∀ bot file in the repo ⊥ backed by a row → `check` finding; unknown bot | arg → exit 2

## §T TASKS

id|status|task|cites
T39|.|`## bots` rows (`src/registry`), `coderabbit`, `greptile`, `socket` files; render adds them; golden files|V38,`.:V1`
T40|.|central `coderabbit` config repo for the fleet; install CodeRabbit, Socket, Greptile apps on FREE tiers; Greptile OSS application; any paid plan = smallest one, ! user action & approval; actlith only writes stubs|V38,R10,R12

## §B BUGS

id|date|cause|fix
