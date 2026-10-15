# SPEC

## §G GOAL

`src/bump` — kinds `update-pins` & `upstream`: scheduled pull-model bump PRs.

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: registry, runners, pins, YAML writer, kinds, render, verbs, CLI
self|src/bump|kinds `update-pins` & `upstream`: scheduled bump PRs
sib|src/registry|parse `.actlith`: format line, `## workflows`, `## actions`, `## hand-written`; row errors
sib|src/system|nix system → runner label table
sib|src/yaml|workflow model types & own YAML writer: key order, quoting, comments, header
sib|src/script|seeded shell scripts shipped in the crate (`update-pins.sh`, `open-pr.sh`) & their bats
sib|src/pin|pin table: built-in pins + `## actions` overrides; `uses:` lines
sib|src/kind|kind trait, arg schemas, common parts: permissions, concurrency, timeouts, checkout, install-nix, cachix, secrets
sib|src/gate|kind `gate`: hk gate per system
sib|src/build|kind `build`: build, flake check, verify-cache
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

- pull model: consumer polls; ⊥ cross-repo token. shapes = nix-hk `update-pins.yml`, `upstream.yml` (`.:R3`).

## §I INTERFACES

- kind `update-pins`: triggers `schedule` (arg `cron`, default `50 6 * * *`), `workflow_dispatch`; job perms `contents: write`, `pull-requests: write`; runs seeded `update-pins.sh` (also `actlith pin` & `actlith gen` in branch); args `cron` `build` `token`.
- kind `upstream`: args `script` (required), `cron` (default `20 6 * * *`), `branch-prefix` (default `upstream/`), `token`; cachix read-only; seeded `open-pr.sh`.

## §R RESEARCH

id|topic|finding|src
R5|token limit|PR opened w/ `GITHUB_TOKEN` ⊥ triggers other workflows ∴ gate ⊥ runs on bump PR until a push \| rerun. lift = App \| fine-grained token secret|GitHub docs "Triggering a workflow from a workflow"

## §V INVARIANTS

V26: bump kinds idempotent: branch exists → exit 0, ⊥ 2nd PR. ⊥ `cancel-in-progress` (half-pushed branch)

## §T TASKS

id|status|task|cites
T22|.|kind `update-pins` (cron, per-job write perms, `actlith pin` + `gen` in branch, `token` arg; R5 limit in header comment)|V26,R5,`src/kind:V3`,`src/kind:V14`
T23|.|kind `upstream` (`script` `cron` `branch-prefix` `token`)|V26,`src/kind:V3`,`src/kind:V14`

## §B BUGS

id|date|cause|fix
