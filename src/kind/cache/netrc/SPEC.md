# SPEC

## §G GOAL

`src/kind/cache/netrc` — netrc rules shared by ∀ cache step that hands one to nix | curl: run-time `https://` check (pipelith PR #46).

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: registry, runners, pins, YAML writer, kinds, render, verbs, CLI
up|src/kind|kind trait, arg schemas, common parts: permissions, concurrency, timeouts, checkout, install-nix, cachix, secrets
up|src/kind/cache|cache steps: install-nix config, cachix & https push, cache secrets
self|src/kind/cache/netrc|netrc rules ∀ netrc user: run-time https check

## §C CONSTRAINTS

- netrc reaches a script only via `env:` `${{ secrets.N }}` (`src/kind:V3`); check & write = seeded script (`src/script:V13`).

## §V INVARIANTS

V65: ∀ netrc user (`push-https.sh`, `verify-https.sh`): resolved `URL` ! start `https://`, checked at run time first; else exit 1, `::error::` names `CACHE` & `NETRC_SECRET` (step `env:`), ⊥ URL (pipelith PR #46)

## §T TASKS

id|status|task|cites
T84|x|run-time `https://` check before ∀ netrc; tested w/ stubs|V65

## §B BUGS

id|date|cause|fix
