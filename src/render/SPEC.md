# SPEC

## §G GOAL

`src/render` — `.actlith` rows → ordered set of files to write.

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: registry, runners, pins, YAML writer, kinds, render, verbs, CLI
self|src/render|`.actlith` rows → ordered set of files (path, bytes)
sib|src/registry|parse `.actlith`: format line, `## workflows`, `## actions`, `## hand-written`; row errors
sib|src/system|nix system → runner label table
sib|src/yaml|workflow model types & own YAML writer: key order, quoting, comments, header
sib|src/script|seeded shell scripts shipped in the crate (`update-pins.sh`, `open-pr.sh`) & their bats
sib|src/pin|pin table: built-in pins + `## actions` overrides; `uses:` lines
sib|src/kind|kind trait, arg schemas, common parts: permissions, concurrency, timeouts, checkout, install-nix, cachix, secrets
sib|src/gate|kind `gate`: hk gate per system
sib|src/build|kind `build`: build, flake check, verify-cache
sib|src/bump|kinds `update-pins` & `upstream`: scheduled bump PRs
sib|src/gen|`gen` & `gen --check`: write or compare rendered files
sib|src/check|`check`: totality over `.github/workflows/`
sib|src/import|`import`: workflow file → row by render-and-compare
sib|src/detect|`detect` & `seed --init`: rows from tracked files
sib|src/report|`report` text, json, md: workflows × systems × pins, secrets
sib|src/bots|reviewer-bot config files (`.coderabbit.yaml`, `greptile.json`, `socket.yml`) from `## bots`
sib|src/fetch|GitHub API tag → SHA for `pin`; ONLY network code
sib|src/cli|clap verbs, flags, exit codes, completions, man page

## §C CONSTRAINTS

- pure: `Registry` in, `Vec<(path, bytes)>` out, sorted bytewise by path. ⊥ I/O.
- kind lookup = closed list of v1 kinds (`gate`, `build`, `update-pins`, `upstream`).

## §I INTERFACES

- fns: `render(&Registry) -> Result<Vec<File>, RenderError>`; `File { path, bytes }`.

## §V INVARIANTS

V31: render output = workflows of ∀ row + ∀ seeded script ≥1 rendered kind needs, sorted, each once; unknown kind | system → error naming row (`src/registry:V10`)

## §T TASKS

id|status|task|cites
T38|x|`render`: validate rows vs kinds & systems, build workflows, add needed scripts, sort|V31,`.:V1`,`src/registry:V10`

## §B BUGS

id|date|cause|fix
