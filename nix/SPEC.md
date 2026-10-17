# SPEC

## §G GOAL

`nix/` — repo gate: dev shell, guardrails, hk steps, CI, release, dogfood. rules & tasks moved here from root keep their numbers.

## §N NAV

rel|path|lens
up|.|-
self|nix|repo gate: dev shell, guardrails, hk steps, CI, release, dogfood
sib|src|the tool: registry, runners, pins, YAML writer, kinds, render, verbs, CLI
sib|tests|integration tests driving `actlith` binary, actionlint & zizmor harness, golden workflows, fixtures
sib|dev|`actlith-dev`, `publish = false`: commit rules, README badges, notices, AGENTS step table & their drift checks

## §C CONSTRAINTS

- flake inputs `nixpkgs-lock` (nixpkgs follows), `nix-hk`, `pklith`, `itok`, `microlith`, `sherd`, `characterminator`, `rekall`, `xenolith`; ∀ follow `nixpkgs-lock`, fleet tools pinned by tag. `pr0d1r2.cachix.org` substituter. 4 systems `aarch64-darwin` `x86_64-darwin` `x86_64-linux` `aarch64-linux`. dev shell = pklith `lib.devShell`; `.envrc` `use flake`.
- gate = hk via pklith-materialized `hk.pklith.pkl` from `.pklith`; `fast` pre-commit, `all` pre-push. guard scripts in `nix/guard/`.
- clippy: fleet deny list + all + pedantic, rustc `unreachable_pub`, fleet `clippy.toml`; `rustfmt.toml` `max_width = 80`.
- commit subject `type(scope): claim` ≤72 chars, lower-case scope, body = WHY; changelog-touched. after `no-commit-to-branch`: branch + PR, merge ⊥ squash.
- CI 3 runners (`x86_64-linux` `aarch64-linux` `aarch64-darwin`); `x86_64-darwin` hand-verified on Intel Mac.
- release: `cargo-release`, Keep a Changelog, even minor = stable. publish ! explicit approval.
- seeding gaps in pklith → anonymous upstream issue after duplicate search.

## §V INVARIANTS

V19: hk hooks installed IN commit that brings hk; ∀ later commit via real hooks; ⊥ `--no-verify`
V20: 1 check = 1 commit = config + hk step + doc row; step adopted only after rejecting a PLANTED violation, recorded in commit body
V21: dogfood: own `ci.yml` hand-written until `gen` renders `gate`; then generated & `actlith gen --check` joins gate in SAME commit
V23: ∀ dep justified in §R; ∀ tool in gate from nix-pinned package, ⊥ fetched at run time
V27: §T rows stay in id order (microlith rule 14); a task that runs out of id order says `AFTER T<n>` in its text. ⊥ reorder rows to change run order
V28: `tdd-order` exemptions ONLY for commits already made w/ ⊥ behaviour to pin (stub, scaffold, copied script predating the rule); full 40-char id + reason in `nix/guard/tdd-order.exempt` & a §B row each
V39: ∀ child git a tool | test aims at a dir ≠ the hooked repo drops `GIT_DIR` `GIT_WORK_TREE` `GIT_INDEX_FILE` `GIT_OBJECT_DIRECTORY` `GIT_ALTERNATE_OBJECT_DIRECTORIES` `GIT_COMMON_DIR` & scratch repos run w/ `core.hooksPath=/dev/null`; ⊥ test spawns git that can reach this repo (as pklith V20)
V40: parallel wave workers share ⊥ git state: pre-commit `stash = "none"` (refs/stash is shared by ∀ worktree); `core.worktree` ⊥ set in `.git/config` (shared) & a gate step refuses it; `test` ⊥ in pre-commit (RED must land; recursion surface)
V42: a change to `hk.pkl` | `hk.pklith.pkl` ! show hk ITSELF (pklr) runs the expected step set; `pkl eval` | `hk validate` passing ⊥ proof (as pklith's hk-parity step)
V43: tests run at pre-push only (V40) ∴ ∀ merge, dogfood | generated-file commit ! pass `hk check --all` BEFORE committing; ⊥ rely on pre-commit for golden | test breakage

## §T TASKS

id|status|task|cites
T2|x|dev shell FIRST: flake on `nixpkgs-lock`, 4 systems, cachix `nixConfig`, fleet guardrail tools pinned by tag, Rust toolchain, `pklith` `lib.devShell` ∴ hooks install on shell entry once `hk.pkl` exists; `.envrc`. ASK user reload|V19,V23
T3|x|hk commit: `.pklith`, `hk.pklith.pkl`, seeded `hk.pkl`, vendored `pkl/`, tracked `.githooks/`; hooks live from this commit|V19,V20
T4|x|AFTER T8: seed crate BY HAND through live hooks: LICENSE, `Cargo.toml`, `clippy.toml`, `rustfmt.toml`, stub `src/`, CHANGELOG + `changelog-touched`; taplo, rustfmt, clippy|V19,V20
T5|x|`commit-msg` step; shell script until `actlith-dev` (`dev:T11`)|V20
T6|x|crate-free pklith catalog checks via `pklith lay`, 1 commit each|V20,V23
T7|x|spec gates: `mth`, `itok` + `.context-limits`, `sherd`|V20
T8|x|`ctrm`, `rekall` (`AGENTS.md`), `xnl check`/`xnl lint`|V20
T9|x|`nix/guard/` scripts; `rust-mirror`, `tdd-order` + exemptions; doctest, rustdoc, `cargo deny`|V20,V23,V28
T10|x|hand-written `.github/workflows/ci.yml` in pklith shape; `actionlint`, `zizmor`, yamllint config|V20,V21
T12|x|`no-commit-to-branch` main; guardrails done → ASK user reload|V19
T31|x|dogfood: generate own `ci.yml`, add `actlith gen --check` step in same commit|V21
T32|x|nix package `packages.<system>.default` built & tested as built; `release.toml` + pre-release hook `hk check --all`|`.:V18`
T41|.|guard step `no-core-worktree`: fail when `git config --local core.worktree` is set; lay via `pklith lay` w/ planted proof|V40
T43|.|port pklith `hk-parity` step: the step set hk sees == the set `pkl eval` sees, per hook; lay w/ planted proof (a `for`/`when` generator)|V42
T49|x|`pipelith` dep = crates.io `0.1.0` (published ∴ ⊥ git pin, ⊥ allow-git, ⊥ `outputHashes`); notices & badges follow; blocks `tests:T50`, `src/kind:T51`|`tests:V48`
T55|x|push step `package`: `cargo package --locked`; packaged crate builds alone (pklith T42)|V20,`.:V22`
T56|x|push step `deny-advisories`: `cargo deny check advisories`; network ∴ push, commit `deny` stays offline|V20,V23
T57|x|commit step `links`: lychee `--offline` over tracked `*.md`; `.pklith` row|V20,V23
T58|x|push step `schema-pin`: `pkl/Config.pkl` == pinned hk's schema; `pkl/VENDORED.md` names version & sha256|V20

## §B BUGS

id|date|cause|fix
B1|2026-10-15|§T rows reordered to run T4 after T8; microlith rule 14 rejects out-of-order ids, caught by `hk check --all` after `pklith lay` laid `mth-check`|V27
B2|2026-10-15|crate seed `980b8db` added `src/lib.rs` & `src/main.rs` before `tdd-order` existed; both bodiless stubs ∴ ⊥ RED to write first|V28
B3|2026-10-15|`scripts/commit-msg.sh` (`4c65434`) & `scripts/changelog-touched.sh` (`41868ca`) copied from pklith before `tdd-order` existed, ⊥ bats mirror; tested Rust port in `actlith-dev` (`dev:T11`) replaced them|V28
B4|2026-10-15|actlith-dev tests ran git in scratch repos w/ the hook's `GIT_*` env under pre-commit `test` ∴ scratch commits re-entered this repo's hooks → 432 hk processes, fork failures|V39
B5|2026-10-15|recursion's scratch `git init` wrote `core.worktree` into shared `.git/config` → ∀ worktree acted on 1 worker's checkout; hk `stash = "git"` popped files across worktrees; commit `ca6a49d` recorded a stale `src/bots/SPEC.md`|V40
B6|2026-10-15|`49bf5c9` dropped `test` from pre-commit w/ a Pkl `for`/`when` generator; `pkl eval` & `hk validate` passed, hk's own pklr evaluated it EMPTY ∴ pre-commit ran 0 steps until fixed; caught by wave workers seeing "hk no steps to run"|V42
B7|2026-10-16|`b147180` made `ci.yml` generated; 5 goldens read it as the old hand-written shape ∴ 5 tests red, committed anyway: `test` runs at pre-push only \& `hk check --all` ran after the commit|V43
B8|2026-10-17|`schema-pin` `getFlake (toString ./.)` fetched the tree as `path:` ∴ copied `target/` while parallel cargo steps rewrote it; CI ubuntu-latest red on a vanished `target/debug/deps` file; fixed w/ `git+file:` (tracked files only)|V20
