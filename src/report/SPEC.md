# SPEC

## §G GOAL

`src/report` — `report`: workflows × systems × pins and the secrets each needs.

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: registry, runners, pins, YAML writer, kinds, render, verbs, CLI
self|src/report|`report` text, json, md: workflows × systems × pins, secrets
sib|src/core|facade: ONE node naming `pipelith`; re-exports its neutral modules
sib|src/registry|parse `.actlith`: format line, `## workflows`, `## actions`, `## hand-written`; row errors
sib|src/system|nix system → runner label table
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
sib|src/bots|reviewer-bot config files (`.coderabbit.yaml`, `greptile.json`, `socket.yml`) from `## bots`
sib|src/fetch|GitHub API tag → SHA for `pin`; ONLY network code
sib|src/cli|clap verbs, flags, exit codes, completions, man page

## §C CONSTRAINTS

- formats `text`, `json` (`schema_version` field), `md`. sorted output.

## §I INTERFACES

- fns: `report(&Registry, &PinTable, Format) -> Result<String, ReportError>` (refused row → error naming line, exit 2). actions = `uses:` of the BUILT workflow, sorted.
- runner = hosted label | `## runners` labels joined `,` (json string; `schema_version` unchanged).
- pinned Xcode (`src/gate:V52`) beside runner: text `…, xcode <v>`, md `(Xcode <v>)`, json `xcode` only when pinned (V33: addition). ⊥ pin → bytes unchanged

## §V INVARIANTS

V33: json carries `schema_version`; field rename | removal bumps it

## §T TASKS

id|status|task|cites
T28|x|`report` text/json/md incl. secrets per workflow|V33,`src/kind:V14`
T48|.|mark `GITHUB_TOKEN` "built-in, nothing to configure" apart from repo secrets; take kind & bot lists from `src/render` & `src/bots` exports ∴ drop the copies in `src/report`, `src/check`; text golden → fixture file once `.pklith` types `txt`|`src/kind:V14`,V33
T70|x|`report` lists the pinned Xcode per workflow; rows w/o it unchanged|`src/gate:V52`,V33
T78|x|`report` names `## runners` labels as the runner of their systems|V33,`src/system:V53`

## §B BUGS

id|date|cause|fix
