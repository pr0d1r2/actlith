# SPEC

## §G GOAL

`src/kind/cache/netrc` — netrc rules shared by ∀ cache step that hands one to nix | curl: run-time `https://` check (pipelith PR #46); read netrc of a private `https` cache (pipelith 0.2.2 `read=secret:R`).

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: registry, runners, pins, YAML writer, kinds, render, verbs, CLI
up|src/kind|kind trait, arg schemas, common parts: permissions, concurrency, timeouts, checkout, install-nix, cachix, secrets
up|src/kind/cache|cache steps: install-nix config, cachix & https push, cache secrets
self|src/kind/cache/netrc|netrc rules ∀ netrc user: run-time https check, read netrc step

## §C CONSTRAINTS

- netrc reaches a script only via `env:` `${{ secrets.N }}` (`src/kind:V3`); check & write = seeded script (`src/script:V13`).

## §I INTERFACES

- fn `read_netrc(line, &BinaryCache) -> Result<Option<Step>, KindError>`: `Some` ⇔ `read_netrc()` = secret; `install_nix` puts it first.
- script `scripts/actlith/read-netrc.sh`, ⊥ args, env `URL`, `NETRC`, `CACHE`, `NETRC_SECRET` → file `$RUNNER_TEMP/read-netrc`.
- secret purpose `cache read (netrc)`.

## §V INVARIANTS

V65: ∀ netrc user (`push-https.sh`, `verify-https.sh`, `read-netrc.sh`): resolved `URL` ! start `https://`, checked at run time first; else exit 1, `::error::` names `CACHE` & `NETRC_SECRET` (step `env:`), ⊥ URL (pipelith PR #46)
V66: read netrc `secret:R` → step before install-nix, `if:` trusted: `read-netrc.sh`, `env:` `NETRC: ${{ secrets.R }}`, `URL`, `CACHE`, `NETRC_SECRET`. script: `::add-mask::` URL first, ⊥ echo; ⊥ `URL` → exit 2; empty `NETRC` → skip, exit 0; V65; `umask 077` → `$RUNNER_TEMP/read-netrc` (kept for later steps; runner empties temp dir after job). trusted install-nix `extra_nix_config` += `netrc-file = ${{ runner.temp }}/read-netrc`; fork PR → ⊥ step, ⊥ cache. push netrc apart: `push-https.sh` own file & `--option netrc-file`; `verify-https.sh` `NETRC` = read netrc if any, else push's

## §T TASKS

id|status|task|cites
T84|x|run-time `https://` check before ∀ netrc; tested w/ stubs|V65
T85|x|pipelith 0.2.2 read netrc (`read=secret:R`): step, config line, purpose, verify netrc; `read-netrc.sh` tested w/ stubs; goldens w/ it pass actionlint & zizmor, old unchanged|V65,V66,`src/kind/cache:V60`,`tests:V4`,`.:V18`

## §B BUGS

id|date|cause|fix
