# Agents

Instructions for coding agents working in this repository.

## Read first

- `SPEC.md` is the law. `docs/KINDS.md` describes every workflow kind
  for humans; `SPEC-PROMPT.md` is the input the spec was made from.
- actlith is the GitHub Actions counterpart of pklith: when a choice is
  open, do what pklith does.

## Working rules

- Work in TDD: a failing test first, then the code, then refactor.
- One topic per commit; commit through the hooks (never `--no-verify`).
- Commit subjects are `type(scope): claim` (at most 72 characters, scope
  in lower case) with a body that says why.
- Enter the dev shell (`direnv allow` or `nix develop`) before committing:
  the hooks refuse without `hk`.
- Spec edits go through `/spec`; `/build` only flips task status.
- A new check is added to `.pklith` and laid with `pklith lay`, one commit
  per check, after proving it rejects a planted violation.
- When seeding from pklith shows a gap, file an issue on pklith, without
  naming this repository, after checking that none exists.

## The gate

Generated from `hk.pkl`; the `agents-table` step fails when it is stale.

<!-- BEGIN steps: generated from hk.pkl by `cargo run -p actlith-dev -- agents`; do not edit -->
46 steps run on every commit and 51 on push and `hk check`; the commit-msg hook checks the message.

| step | runs on | files | fixes |
| --- | --- | --- | --- |
| `trailing-whitespace` | commit | `**/*` | yes |
| `final-newline` | commit | `**/*` | yes |
| `line-endings` | commit | `**/*` | yes |
| `no-bom` | commit | `**/*` | - |
| `no-merge-conflict` | commit | `**/*` | - |
| `no-case-conflict` | commit | `**/*` | - |
| `no-broken-symlinks` | commit | whole tree | - |
| `no-private-key` | commit | `**/*` | - |
| `ripsecrets` | commit | `**/*` | - |
| `no-local-paths` | commit | `**/*` | - |
| `nixfmt` | commit | `**/*.nix` | yes |
| `shfmt` | commit | `**/*.sh` `**/*.bash` | yes |
| `taplo` | commit | `**/*.toml` | yes |
| `rustfmt` | commit | `**/*.rs` | yes |
| `typos` | commit | `**/*` | - |
| `shellcheck` | commit | `.githooks/*` `.envrc` `**/*.sh` | - |
| `clippy` | commit | `**/*.rs` `Cargo.toml` `clippy.toml` | - |
| `actionlint` | commit | `.github/workflows/*.yml` `.github/workflows/*.yaml` | - |
| `zizmor` | commit | `.github/workflows/*.yml` `.github/workflows/*.yaml` | - |
| `statix` | commit | `**/*.nix` | yes |
| `deadnix` | commit | `**/*.nix` | yes |
| `markdownlint` | commit | `**/*.md` | yes |
| `yamllint` | commit | `**/*.yml` `**/*.yaml` | - |
| `editorconfig-checker` | commit | `**/*` | - |
| `ctrm` | commit | `**/*` | yes |
| `xnl-check` | commit | `**/*.nix` `**/*.pkl` `.envrc` `.githooks/*` | - |
| `xnl-lint` | commit | `**/*.nix` `**/*.pkl` `.envrc` `.githooks/*` | yes |
| `rust-mirror` | commit | `**/*.rs` `nix/guard/rust-mirror.sh` | - |
| `rustdoc` | commit | `**/*.rs` `Cargo.toml` | - |
| `json` | commit | `**/*.json` | - |
| `rekall` | commit | `AGENTS.md` `CLAUDE.md` `rekall.toml` | - |
| `mth-fmt` | commit | `**/SPEC.md` | yes |
| `mth-check` | commit | `**/SPEC.md` | - |
| `sherd-check` | commit | `**/SPEC.md` `**/*.rs` | - |
| `sherd-nav` | commit | `**/SPEC.md` | yes |
| `sherd-budget` | commit | `**/SPEC.md` `.context-limits` | - |
| `itok` | commit | `**/*` `.context-limits` | - |
| `agents-table` | commit | `hk.pkl` `hk.pklith.pkl` `AGENTS.md` `dev/**` | yes |
| `readme-badges` | commit | `README.md` `Cargo.toml` `.coverage` `hk.pkl` `hk.pklith.pkl` `flake.lock` `**/SPEC.md` `docs/LLM-DISCLAIMER.md` `.github/workflows/ci.yml` `dev/**` | yes |
| `notices` | commit | `Cargo.toml` `Cargo.lock` `docs/THIRD-PARTY-NOTICES.md` `dev/**` | yes |
| `doctest` | commit | `**/*.rs` `Cargo.toml` | - |
| `deny` | commit | `Cargo.toml` `Cargo.lock` `deny.toml` | - |
| `gen-check` | commit | `.pklith` `hk.pklith.pkl` `nix/pklith.nix` | yes |
| `pklith-check` | commit | whole tree | - |
| `actlith-gen-check` | commit | `.actlith` `.github/workflows/**` `scripts/actlith/**` `src/**` | yes |
| `actlith-check` | commit | `.actlith` `.github/workflows/**` | - |
| `test` | push | `**/*.rs` `Cargo.toml` `Cargo.lock` | - |
| `coverage` | push | `**/*.rs` `Cargo.toml` `.coverage` | - |
| `tdd-order` | push | `**/*.rs` `nix/guard/tdd-order.sh` `nix/guard/tdd-order.exempt` | - |
| `package` | push | `Cargo.toml` `Cargo.lock` `src/**` | - |
| `deny-advisories` | push | `Cargo.toml` `Cargo.lock` `deny.toml` | - |
<!-- END steps -->
