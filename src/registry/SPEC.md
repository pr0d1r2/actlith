# SPEC

## §G GOAL

`src/registry` — parse `.actlith` into typed rows; every row error named.

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: registry, runners, pins, YAML writer, kinds, render, verbs, CLI
self|src/registry|parse `.actlith`: format line, `## workflows`, `## actions`, `## hand-written`; row errors
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
sib|src/bots|reviewer-bot config files (`.coderabbit.yaml`, `greptile.json`, `socket.yml`) from `## bots`
sib|src/fetch|GitHub API tag → SHA for `pin`; ONLY network code
sib|src/cli|clap verbs, flags, exit codes, completions, man page

## §C CONSTRAINTS

- zero deps. pure fns over text; ⊥ I/O (caller reads file).
- format = pklith registry style: `format N` line, `## <section>`, pipe tables, `#` comments, `-` empty, `\|` literal pipe.

## §I INTERFACES

- types: `Registry { workflows, actions, hand_written }`, `WorkflowRow { id, kind, systems, cache, args }`, `ActionPin { action, version, sha }`, `HandWritten { path, reason }`, `RegistryError` (line, what, fix).
- fns: `parse(&str) -> Result<Registry, RegistryError>`.

## §V INVARIANTS

V10: unknown kind | args key | system, missing required arg, `x86_64-darwin` in systems → exit 2 naming it. ⊥ silent default. registry checks row SHAPE; kind & system names validated against `src/kind` & `src/system` by `src/render`
V11: `.actlith` ! carry `format` line; newer than supported → exit 2 naming both versions, ⊥ best-effort parse
V12: missing `.actlith` → `gen`|`check` exit 2, ⊥ pass, ⊥ empty render. missing optional section (`## actions`, `## hand-written`) = empty

## §T TASKS

id|status|task|cites
T13|.|parse `format`, `## workflows`, `## actions`, `## hand-written`; duplicate id, empty reason, bad cell count → error w/ line; fixtures 1-to-1|V10,V11,V12

## §B BUGS

id|date|cause|fix
