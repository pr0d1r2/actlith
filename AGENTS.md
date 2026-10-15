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
