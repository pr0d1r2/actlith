actlith

Tool that materializes GitHub Actions workflows from a small config.

pklith derives hk git hook guardrails from what a repository's files require.
actlith does the same for CI: a short declaration in the repository (which
systems, which gate, which caches, which schedules) becomes the full
`.github/workflows/*.yml` files, rendered deterministically, with every action
pinned and every hardening default applied.

Goals:
- one small config per repository instead of hand-copied workflow YAML
- the same workflow shape across the fleet, so a fix lands everywhere by a
  regeneration, not by editing many repositories by hand
- check that the committed workflows still match the config (drift gate)

Write in Rust.
Use atomic git commits per topic for human readability.
Keep the closest possible parallel with pklith: same command shape, same
config file style, same guardrails, same conventions. Where pklith already
decided something, reuse the decision unless this file says otherwise.

# Naming

- Project / crate name: `actlith` (act(ions) + lith, stone): fleet family
  `pklith`, `microlith`, `xenolith`.
- Binary name: `actlith`, the same as the crate, as in pklith: `cargo install
  actlith` gives the command it names, and there is no second crates.io name.
- Availability checked 2026-10-15: `actlith` free on crates.io, npm, PyPI,
  RubyGems, Homebrew core; no GitHub user/org `actlith`; no GitHub repository
  with that name.
- Rejected alternatives:
  - `actionlint`: taken by rhysd/actionlint, the workflow linter that actlith's
    own gate and pklith's catalog already run on the generated files.
  - `actionlith`: free, but one letter away from `actionlint`; easy to confuse.
  - `workflith`: free, but longer to type.
  - `ghalith`: GitHub user `ghalith` exists.
  - `runlith`: taken on npm; GitHub user `runlith` exists.
- Repository directory is still `actionlint`; rename it to `actlith` before the
  first push.

# Fleet survey (2026-10-15)

Local checkouts next to this repository, read-only:
- 170 repositories have `.github/workflows/`; 163 of them have a `ci.yml`.
- Other workflow kinds: `update-pins.yml` (11), `release.yml` (3),
  `build.yml` (3), `upstream.yml` (2), then one-offs.
- Many `ci.yml` files are byte-identical copies (groups of 28, 22, 19, 12 and
  12), yet copies of the "same" workflow drift: pklith's and congestimancer's
  `ci.yml` differ in comment, job name, `permissions` place, `timeout-minutes`
  and comment spacing after SHA pins.
- Older copies pin actions by tag (`actions/checkout@v4`,
  `cachix/install-nix-action@v27`), which zizmor flags; current ones pin by
  SHA with a version comment. Upgrading means editing every copy.
- Current reference shape (pklith `ci.yml`): push to `main`, `pull_request`,
  `workflow_dispatch`; `concurrency` per ref with cancel; `permissions:
  contents: read`; matrix `ubuntu-latest`, `ubuntu-24.04-arm`, `macos-latest`;
  checkout with `persist-credentials: false`; install-nix with the
  `pr0d1r2.cachix.org` substituter; cachix push except for fork PRs; one step
  `nix develop --command hk check --all --check`.

Conclusion: the variable part of a fleet workflow is a handful of values. The
rest is shape, and shape should come from one place.

# Workflow kinds

Read `docs/KINDS.md` together with this file. It is the authoritative
catalogue of workflow kinds, their triggers, permissions, jobs, steps and the
row values each accepts. Where the config sketch below differs from it, the
catalogue wins.

# Config: `.actlith`

The one file a repository writes. Same style as `.pklith`: a `format 1`
header, `## <section>` headings, pipe tables, `#` comments, `-` for an empty
cell, `\|` for a literal pipe. Sketch, not final:

```
format 1

## workflows
id|kind|systems|gate|cache
ci|gate|x86_64-linux, aarch64-linux, aarch64-darwin|hk check --all --check|pr0d1r2
update-pins|update-pins|x86_64-linux|-|pr0d1r2
```

- One row = one `.github/workflows/<id>.yml`.
- `kind` names a workflow template from actlith's built-in catalog
  (`gate`, `update-pins`, `release`, ...). As in pklith, a `.actlith` row with
  the same id as a built-in one replaces it; a new id adds one.
- `systems` are nix system names, not runner labels. actlith maps them to
  runners in one table: `x86_64-linux` = `ubuntu-latest`, `aarch64-linux` =
  `ubuntu-24.04-arm`, `aarch64-darwin` = `macos-latest`. `x86_64-darwin` has no
  runner and is refused with a clear message (verified by hand on the Intel
  Mac, as in the other repos).
- Triggers, permissions, concurrency, timeouts and step lists belong to the
  kind, not to the row. A repository that needs something else adds a kind,
  it does not grow per-row knobs.
- Missing `.actlith`: `actlith detect` proposes rows from tracked files
  (`flake.nix` gives `gate`, a `nixpkgs-lock` input gives `update-pins`).

# actlith CLI

Verbs mirror pklith's, so muscle memory carries over:

actlith gen # render .actlith into .github/workflows/*.yml
actlith gen --check # write nothing; exit 1 if any generated workflow is stale
actlith check # every workflow file is generated by actlith or listed as hand-written; no row is stale
actlith detect # propose .actlith rows from tracked files, one per line
actlith import .github/workflows/ci.yml # turn an existing workflow into a .actlith row, refuse what has no kind
actlith report [--format text|json|md] # workflows x systems x actions with pinned versions
actlith pin [--dry-run] # resolve action tags to commit SHAs (network, opt-in)

- Exit codes as in pklith: 0 ok, 1 finding, 2 usage or unreadable input.
- Generated files start with a `# generated by actlith gen from .actlith; do
  not edit` header, as `hk.pklith.pkl` does. Hand-written workflows stay
  allowed but must be listed, so `check` sees every file under
  `.github/workflows/`.

# Decisions (2026-10-15)

1. Deterministic core, as in pklith: same `.actlith` and same actlith version
   give byte-identical workflows. No LLM, no heuristic guess, no clock, no
   hostname in output. No network, except `actlith pin`, which is opt-in.
2. Action pins: every `uses:` is pinned to a full commit SHA with the version
   as a trailing comment (`@<sha> # v7.0.1`), the zizmor-clean form the newest
   fleet workflows use. Pins live in a pin table (built-in, overridable in
   `.actlith` `## actions`), never in templates, so one bump of the table
   updates every kind. `gen` never resolves tags; `pin` does, and writes the
   table.
3. Hardening defaults in every kind, not optional: top-level `permissions:
   contents: read` (jobs widen only what they need); checkout with
   `persist-credentials: false`; `concurrency` group per workflow and ref;
   `timeout-minutes` on every job; no `pull_request_target`; no untrusted
   expression interpolated into `run:`. Kinds that need write access (for
   example `update-pins`) declare it per job.
4. Output gate: generated workflows must pass `actionlint` and `zizmor` with
   no finding. A template change that fails either is a test failure in
   actlith, not something consumers discover.
5. Output layout: one fixed key order and quoting style, comments taken from
   the kind, so diffs between two generations show only real changes.
6. Templates are Rust data (structs rendered by actlith's own YAML writer),
   not text templates with string substitution. Reason: a typed model cannot
   emit malformed YAML or an unquoted expression, and it can be tested 1-to-1.

# Decisions, round 2 (2026-10-15): place next to pklith

7. Division of labour: pklith owns the hk gate, actlith owns CI orchestration.
   The `gate` kind runs the hk gate (`nix develop --command hk check --all
   --check`) and holds no checks of its own, so CI and a laptop cannot
   disagree. actlith never adds lint steps to workflows.
8. pklith integration: pklith's catalog gets an `actlith-gen-check` row
   (`actlith gen --check`) and the `.actlith` file type; actionlint and zizmor
   rows stay where they are. pklith fragment detection activates actlith when
   `.actlith` is tracked.
9. Branch protection stays in `pklith protect`, which reads required
   contexts from CI job names. actlith keeps job names stable across
   versions (a rename is a breaking change) so protection does not break.
10. Nix: `lib.devShell` is pklith's; actlith ships only `packages.<system>
    .default` and is added through pklith's catalog `nix` column, not a
    second dev shell builder.
11. Dependencies minimal, as in pklith: each crate justified in the spec's
    research section. YAML is written by actlith's own writer (decision 6).
    `import` needs to read YAML: pick a maintained parser only after research,
    or compare against rendered candidates if that is enough.
12. Fleet rollout: dogfood on actlith and pklith first, then congestimancer,
    then the repositories with byte-identical copies. Never push to another
    repository without explicit approval.

# Process

Same as congestimancer:
- Nix flake dev shell loaded with direnv. When the basic shell exists, ask me
  to reload and continue the session.
- Guardrails first, before product code, one check per commit, hooks live from
  the commit that brings hk. When guardrails are ready, ask me to reload and
  continue the session.
- Work in TDD: RED, GREEN, REFACTOR cycles.
- 1-to-1 unit coverage with floor and ceiling, aiming at 100%.
- Specs in federated SPEC-DAG form; build in sherd waves with parallel
  subagents once the spec has independent nodes.

# Decisions, round 3 (2026-10-15): guardrails and conventions

13. Guardrails: the same Rust guardrail tools as the other repos, each in its
    role:
    - pklith: materialise hk git hooks; installed when entering the dev shell
      (`lib.devShell`, tracked `.githooks/`).
    - nix-hk: hk in the flake.
    - sherd: SPEC.md federation over a directory DAG, budgets, navigation and
      wave builds.
    - microlith: spec operations (`mth fmt`, `mth check`, `mth archive`).
    - itok: context limits (`.context-limits`).
    - characterminator: allowed characters per file type (`.ctrm`).
    - rekall: rules and skills from agent memory, with its honesty gate.
    - xenolith: one language per file, `rust-mirror` test layout, `tdd-order`.
    - actionlint and zizmor: on this repository's workflows and on every
      fixture workflow actlith renders in tests.
14. Dogfood from day one: this repository's own `.github/workflows/ci.yml` is
    hand-written until `actlith gen` can render it, then generated, and
    `actlith gen --check` joins the gate in the same commit.
15. Argument parsing: `clap` (derive), as congestimancer decided; pklith#36
    tracks the move for pklith.
16. `unsafe_code = "forbid"`: actlith needs no FFI.
17. Conventions copied from pklith and congestimancer:
    - Cargo: edition 2024, `rust-version = "1.95"`, MIT, shared clippy deny
      list and `clippy.toml` thresholds, `rustfmt.toml` (`max_width = 80`).
    - Workspace: one published crate `actlith` (library + binary) plus `dev/`
      (`actlith-dev`, `publish = false`) for README badges, third-party
      notices and the AGENTS step table.
    - Flake: Rust from `nixpkgs-lock`, fleet tools pinned by tag, every input
      follows `nixpkgs-lock`, `pr0d1r2.cachix.org` substituter, 4 systems
      (`aarch64-darwin`, `x86_64-darwin`, `x86_64-linux`, `aarch64-linux`),
      `.envrc` with `use flake`.
    - hk.pkl: `fast` steps pre-commit, `all` steps pre-push; commit-msg lint
      (`type(scope): claim`, at most 72 characters); changelog-touched check.
    - Coverage: hard floor plus `.coverage` ratchet ceiling.
    - Size limits per module, ASCII-only via `.ctrm`.
    - Spec: sections G/F/N/C/I/R/V/T/B, caveman encoding, start with one
      SPEC.md and sherdify when it exceeds its budget.
    - cargo-deny with the shared `deny.toml`.
    - CI on 3 runners (`x86_64-linux`, `aarch64-linux`, `aarch64-darwin`);
      `x86_64-darwin` verified by hand on the Intel Mac.
    - Release: `cargo-release`, Keep a Changelog, version ladder (even minor =
      stable). Claim the crate with the first real pre-release `0.1.0-rc.1`;
      publishing needs explicit approval.
    - Tests: fixtures in `tests/fixtures/<topic>/`, CLI tests in
      `tests/cli.rs`, golden workflow files compared byte for byte.
    - After `no-commit-to-branch` lands, work goes through branch + PR; merge
      by rebase or merge commit, never squash.

# Prior art and rejected designs

- Reusable workflows (`workflow_call`) in one central repository: rejected.
  The caller depends on a workflow in another repository, so trust and pinning
  span two repos; zizmor and actionlint only see the one-line caller; job names
  become nested (`gate / gate`), which breaks the required contexts that
  `pklith protect` derives; and a fix in the central repo reaches every
  consumer without review in that consumer.
- Composite actions in a central repository: rejected for the same trust and
  visibility reasons, and they cannot carry triggers, permissions or matrices,
  which is most of what drifts.
- Copy and sync bots (a template repo plus a PR bot that copies files):
  rejected. They copy text, so local edits and the template fight; nothing
  says which values were meant to differ.
- Dependabot / Renovate for action pins: rejected as the pin source, because
  they would edit generated files and fail `actlith gen --check`. Pins are
  bumped in the pin table instead (see decisions).
- Differentiator: generated files are plain, local and readable, every value
  that may differ is named in `.actlith`, and the gate proves the files match.

# Decisions, round 4 (2026-10-15)

18. Pin bumps: the `update-pins` kind runs `actlith pin` and `actlith gen`
    in its PR branch. One reviewed PR per morning carries the nixpkgs bump,
    action pin bumps and template changes together; no Dependabot.
19. actlith version rollout: consumers take actlith as a flake input that
    follows `nixpkgs-lock`. A new actlith release reaches a repository when its
    `update-pins` run bumps the input and regenerates; no fleet-wide push
    tool. Template changes therefore land per repository, reviewed, and a
    repository can hold back by not merging.
20. Secrets are part of the template contract: `CACHIX_AUTH_TOKEN` for cache
    pushes; push skipped for fork PRs (no secret there, read-only cache still
    works). A kind names every secret it reads; `actlith report` lists them so
    a repository owner knows what to configure.
