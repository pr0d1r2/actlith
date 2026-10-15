# Built by an LLM, deliberately and in the open

This repository (code, spec, tests and prose) was written by
[Claude Code](https://claude.com/claude-code) running Anthropic's **Claude
Opus 5.5**. Every commit carries a `Co-Authored-By: Claude Opus 5.5`
trailer, which `git log --format=%B | grep -c Co-Authored-By` will
confirm against `git rev-list --count HEAD`. A human owns every decision,
reviews every diff, and is accountable for what ships.

That is the disclaimer. The rest of this file is why it is a design note
rather than an apology, and what you can check for yourself.

## Why say it at all

An LLM writes plausible code, and plausible is not correct. A reader who
does not know how a repository was produced cannot calibrate how hard to
look at it.

actlith makes the point sharper. Its whole job is to replace hand-copied
CI workflows, which drift, with files rendered from one place and checked
against it. A repository that did that for others while its own rules
lived in prose would be arguing against itself. So the standard here is:
**a claim in prose is a defect until something executes it.**

## The method is spec-driven development

[`SPEC.md`](../SPEC.md) and one `SPEC.md` per node below it are the law,
not a description written afterwards. Together they carry
**44 invariants** (the `V` rows: what must stay true), `T` tasks (what is
decided and what is not), **5 bugs** in their `B` tables (each paired
with the rule that now catches it), and `R` research (the
measurements the constraints rest on). A rule and its checker land in the
same commit, because a rule with no runner gates nothing.

The spec's format is checked mechanically by
[`microlith`](https://github.com/pr0d1r2/microlith) and its federation by
[`sherd`](https://github.com/pr0d1r2/sherd), so "the spec says so" is a
statement about a file that was parsed, not one that was skimmed.

## The guardrails are git hooks that also run on CI

Entering the dev shell (`nix develop`, or `direnv allow`) points git at
the tracked hooks, which run [hk](https://github.com/jdx/hk) against one
definition of the gate in [`hk.pkl`](../hk.pkl); the README badge counts
its steps. Each check was laid in its own commit, and the commit that
brings a rule records the planted violation it refused.
[`ci.yml`](../.github/workflows/ci.yml) runs that same
definition on three platforms, so a laptop and a runner cannot disagree.
Line coverage has a floor of 99.09% that may only rise, `unsafe` is
forbidden, and the only command meant to touch the network is
`actlith pin`.

## The record is deliberately unflattering

Every entry in the bug table was green before it was found.

**B1: the task table was reordered to change run order.** The spec
format requires task ids in order; the reorder passed every check present
at the time and was refused only once `pklith lay` laid `mth-check` and
`hk check --all` ran it.

**B2: the crate's first source files came before the rule that orders
tests first.** They were bodiless stubs, so there was no failing test to
write first; they are listed by full commit id in
`nix/guard/tdd-order.exempt` with the reason.

**B3: the first commit-message and changelog hooks were shell scripts
copied without tests.** They predated the rule that every script has a
test; the tested Rust port in `actlith-dev` replaced them.

**B4: the test suite forked without bound inside a commit hook.** Tests
ran git in scratch repositories but inherited the hook's `GIT_*`
variables, so each test's commit re-entered the real repository's hooks,
which ran the tests again. Every test had passed when run by hand.

**B5: that recursion damaged every checkout at once.** A scratch
`git init` aimed at the real repository wrote `core.worktree` into the
configuration all worktrees share, so each parallel worker's git acted
on one worker's files.

## What a reader should check

1. **Does the gate run for you?** `nix develop`, then `hk check --all`. If
   a claim here is false, that is where it shows.
2. **Does the bug table look like a real log or a curated one?**
3. **Do the invariants have runners?** An invariant that nothing executes
   is exactly the defect this project exists to remove.

## Accountability

The human named in [`LICENSE`](../LICENSE) is responsible for this code,
including the parts a model wrote and the parts nobody caught. "The LLM
wrote it" explains provenance; it never transfers responsibility.
