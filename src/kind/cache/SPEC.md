# SPEC

## §G GOAL

`src/kind/cache` — GitHub steps for the cache a row trusts (pipelith `BinaryCache`: `## caches` row | built-in cachix): install-nix config, cachix & https push, cache secrets. ⊥ when a kind pushes (kind nodes pick a `CachixPush` mode).

## §N NAV

rel|path|lens
up|.|-
up|src|the tool: registry, runners, pins, YAML writer, kinds, render, verbs, CLI
up|src/kind|kind trait, arg schemas, common parts: permissions, concurrency, timeouts, checkout, install-nix, cachix, secrets
self|src/kind/cache|cache steps: install-nix config, cachix & https push, cache secrets

## §C CONSTRAINTS

- input = `Checked.cache` ONLY, ⊥ `Checked.row.cache` (pipelith: `.row.cache` = 0.2.0 type, built-in only).
- built-in cache (`Push::Forge`) → same bytes as before (cachix step, `CACHIX_AUTH_TOKEN`) ∴ golden w/o `## caches` unchanged (`.:V18`).
- secret value ⊥ in any emitted file: only `${{ secrets.N }}`, in `with:` | `env:`, ⊥ `run:` (`src/kind:V3`).
- `https` push logic = seeded script `push-https.sh` (`src/script:V13`).

## §I INTERFACES

- fns (`Result<_, KindError>`): `install_nix(&PinTable, Option<&BinaryCache>) -> Result<Vec<Step>>`; `cachix(&PinTable, &BinaryCache, CachixPush) -> Result<Option<Step>>`; `push_https(line, &BinaryCache, CachixPush, attr) -> Result<Option<Step>>`; `secrets(Option<&BinaryCache>, CachixPush) -> Vec<Secret>`.
- script `scripts/actlith/push-https.sh ATTR…`, env `URL`, `NETRC`: ATTR `<output>.<rest>` → installable `.#<output>.<system>.<rest>`, system = runner's.
- secret purposes: `cache read` (url | key secret), `cachix push`, `cache push (netrc)`.

## §V INVARIANTS

V60: read: url & key literal → `extra_nix_config` = `nix_config()` as is. a secret → 2 install-nix steps: `if:` trusted (⊥ fork PR, `src/kind:V54` guard) w/ `${{ secrets.N }}` in `extra_nix_config`; `if:` fork PR w/o cache ∴ ⊥ empty substituter. `read_secrets()` ∈ kind `secrets()` (`src/kind:V14`)
V61: push per `Push`: `ReadOnly` → ⊥ cachix step, ⊥ push step, ⊥ push secret · `Forge` → cachix step as before, `CACHIX_AUTH_TOKEN` · cachix `Secret(N)` → that step w/ `authToken: ${{ secrets.N }}` · https `Secret(N)` → `push-https.sh` step, `env:` `NETRC: ${{ secrets.N }}` & `URL`, `if:` from mode (`SkipForkPr` ⊥ fork PR, `MainOnly`|`ExplicitMain` main push), ⊥ cachix step. mode `ReadOnly` → ⊥ push, ⊥ push secret, whatever the cache
V62: `push-https.sh`: `::add-mask::` URL before any output, ⊥ echo URL; ⊥ ATTR | ⊥ `URL` → exit 2; empty `NETRC` (fork) → skip, exit 0; `umask 077`, netrc → `$RUNNER_TEMP/netrc`, `trap` removes it; ∀ ATTR before netrc: `<output>.<system>.<name>` ! exist (`nix eval … --apply`) & its installable's `drvPath` ! evaluate (forced), else `::error::` naming it & fix, exit 1; `nix copy --option netrc-file <it> --to "$URL" <installables>`
V65: ∀ netrc user (`push-https.sh`, `verify-https.sh`): resolved `URL` ! start `https://`, checked at run time first; else exit 1, `::error::` names `CACHE` & `NETRC_SECRET` (step `env:`), ⊥ URL (pipelith PR #46)

## §T TASKS

id|status|task|cites
T44|x|`## caches` w/ `name, public-key`; superseded by T79|`src/registry:V10`
T79|x|render pipelith 0.2.1 `## caches` (its T41): `check` → `Row::check_with` → `Checked`; `nix.rs`, `cachix.rs` → here; read, push, secrets per V60–V62; scripts tested w/ stubs; new goldens pass actionlint & zizmor, old unchanged|V60,V61,V62,`src/kind:V14`,`tests:V4`,`.:V18`
T84|x|run-time `https://` check before ∀ netrc; tested w/ stubs|V65

## §B BUGS

id|date|cause|fix
