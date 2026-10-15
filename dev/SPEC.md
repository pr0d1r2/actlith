# SPEC

## §G GOAL

`dev/` — `actlith-dev`, `publish = false`: repo tooling & its drift checks.

## §N NAV

rel|path|lens
up|.|-
self|dev|`actlith-dev`, `publish = false`: commit rules, README badges, notices, AGENTS step table & their drift checks
sib|src|the tool: registry, runners, pins, YAML writer, kinds, render, verbs, CLI
sib|tests|integration tests driving `actlith` binary, actionlint & zizmor harness, golden workflows, fixtures
sib|nix|repo gate: dev shell, guardrails, hk steps, CI, release, dogfood

## §C CONSTRAINTS

- workspace member, `publish = false`, `release = false`. shape copied from congestimancer-dev & pklith-dev.

## §V INVARIANTS

V36: README badges block, `docs/THIRD-PARTY-NOTICES.md` & AGENTS.md step table generated; drift → gate fails
V37: commit-msg & changelog rules = tested Rust here; hk `commit-msg` hook runs them via `cargo run -p actlith-dev`

## §T TASKS

id|status|task|cites
T11|~|port commit-msg & changelog rules (RED, GREEN) & lay `test` (nextest) + `coverage` over its tests; README badges, THIRD-PARTY-NOTICES, AGENTS step table + drift steps|V36,V37,`nix:V20`

## §B BUGS

id|date|cause|fix
