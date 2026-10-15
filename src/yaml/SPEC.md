# SPEC

## §G GOAL

`src/yaml` — workflow model & own YAML writer; malformed YAML ⊥ representable.

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: registry, runners, pins, YAML writer, kinds, render, verbs, CLI
self|src/yaml|workflow model types & own YAML writer: key order, quoting, comments, header
sib|src/registry|parse `.actlith`: format line, `## workflows`, `## actions`, `## hand-written`; row errors
sib|src/system|nix system → runner label table
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

- zero deps. writer only, ⊥ reader. output style = yamllint-clean under repo `.yamllint.yaml`: `---` start, 2 spaces before comments, ≤120 cols (fold long expressions w/ `>-`).

## §I INTERFACES

- types: `Workflow`, `Trigger`, `Job`, `Step`, `Matrix`, `Permissions`, `Expr` (a `${{ }}` value, only in `with:`/`env:`/`if:`/`runs-on`), `Comment`.
- fns: `render(&Workflow) -> String` incl. generated header.

## §V INVARIANTS

V5: workflows built as typed model → own YAML writer: fixed key order, fixed quoting, comments from kind. ⊥ string substitution into YAML text ∴ malformed YAML | injected expression ⊥ representable
V8: ∀ generated file (workflow, seeded script) starts w/ generated header; hand edit = drift caught by `src/gen:V6`

## §T TASKS

id|status|task|cites
T16|.|model types + writer: fixed order & quoting, header, comments, folded scalars; golden bytes tests incl. pklith `ci.yml` shape|V5,V8,`.:V1`

## §B BUGS

id|date|cause|fix
