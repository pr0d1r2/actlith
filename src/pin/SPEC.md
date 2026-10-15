# SPEC

## §G GOAL

`src/pin` — pin table: every `uses:` resolves to a full commit SHA with its version.

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: registry, runners, pins, YAML writer, kinds, render, verbs, CLI
self|src/pin|pin table: built-in pins + `## actions` overrides; `uses:` lines
sib|src/registry|parse `.actlith`: format line, `## workflows`, `## actions`, `## hand-written`; row errors
sib|src/system|nix system → runner label table
sib|src/yaml|workflow model types & own YAML writer: key order, quoting, comments, header
sib|src/script|seeded shell scripts shipped in the crate (`update-pins.sh`, `open-pr.sh`) & their bats
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

- built-in table = `const` in this node, bumped by `actlith pin` on actlith's own repo; `.actlith` `## actions` rows override per repo. ⊥ network here (`src/fetch` resolves).

## §I INTERFACES

- types: `PinTable`, `Uses` (`owner/repo@sha # version`), `PinError`.
- fns: `PinTable::builtin()`, `with_overrides(&[ActionPin])`, `uses(&self, action) -> Result<Uses, PinError>`.

## §R RESEARCH

id|topic|finding|src
R2|pin drift|older copies pin by tag (`actions/checkout@v4`, `cachix/install-nix-action@v27`); nix-hk `update-pins`/`build`/`upstream` still `@v5`/`@v31`/`@v16`; newest pin SHA + `# vX.Y.Z`. zizmor flags tag pins|`.:R1` reading

## §V INVARIANTS

V2: ∀ emitted `uses:` = `owner/repo@<40-hex sha>  # <version>`; sha from pin table (built-in, overridden by `## actions`) ONLY. tag | branch ref in output ⊥. action w/o pin → exit 2

## §T TASKS

id|status|task|cites
T15|.|built-in pins (checkout v7.0.1, install-nix-action v31.11.0, cachix-action v17), override merge, 40-hex check; missing pin → error naming action|V2

## §B BUGS

id|date|cause|fix
