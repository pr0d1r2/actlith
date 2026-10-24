# SPEC

## §G GOAL

`tests/` — integration tests driving the `actlith` binary; the actionlint & zizmor harness.

## §N NAV

rel|path|lens
up|.|-
self|tests|integration tests driving `actlith` binary, actionlint & zizmor harness, golden workflows, fixtures
sib|src|the tool: registry, runners, pins, YAML writer, kinds, render, verbs, CLI
sib|dev|`actlith-dev`, `publish = false`: commit rules, README badges, notices, AGENTS step table & their drift checks
sib|nix|repo gate: dev shell, guardrails, hk steps, CI, release, dogfood

## §C CONSTRAINTS

- CLI tests in `tests/cli.rs` (`assert_cmd`-free: `std::process::Command` on the built bin), fixtures `tests/fixtures/<node>/`, golden workflows `tests/golden/<kind>/*.yml`.
- `actionlint` & `zizmor --offline` from the dev shell; missing tool → test FAILS, ⊥ skip.

## §V INVARIANTS

V4: ∀ kind × fixture args → rendered workflow passes `actionlint` & `zizmor` w/ 0 findings, in actlith's OWN test suite. template change failing either = red build, ⊥ consumer surprise
V35: golden files change only w/ a CHANGELOG entry (`.:V18`); `UPDATE_GOLDEN=1` rewrites them, ⊥ default
V48: pipelith switch ⊥ changes any golden byte; golden diff = extraction bug. exception: pipelith 0.2 `x86_64-darwin` in `DEFAULT_SYSTEMS` (T38) → default matrices gain it (CHANGELOG)

## §T TASKS

id|status|task|cites
T18|.|harness: render ∀ kind × fixture args → `actionlint` + `zizmor` 0 findings; golden compare|V4,V35
T50|x|switch `src/{registry,yaml,system,gen,detect,script}` neutral parts → `pipelith`; forge parts stay|V48

## §B BUGS

id|date|cause|fix
