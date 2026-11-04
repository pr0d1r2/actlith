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

- fns: `install_nix(&PinTable, Option<&BinaryCache>) -> Vec<Step>`; `cachix(&PinTable, &BinaryCache, CachixPush) -> Option<Step>`; `push_https(line, &BinaryCache, CachixPush, attr) -> Option<Step>`; `secrets(Option<&BinaryCache>, CachixPush) -> Vec<Secret>`.
- script `scripts/actlith/push-https.sh ATTR…`, env `URL`, `NETRC`: ATTR `<output>.<rest>` → installable `.#<output>.<system>.<rest>`, system = runner's.
- secret purposes: `cache read` (url | key secret), `cachix push`, `cache push (netrc)`.

## §V INVARIANTS

V60: read: url & key literal → `extra_nix_config` = `nix_config()` as is. a secret → 2 install-nix steps: `if:` trusted (⊥ fork PR, `src/kind:V54` guard) w/ `${{ secrets.N }}` in `extra_nix_config`; `if:` fork PR w/o cache ∴ ⊥ empty substituter. `read_secrets()` ∈ kind `secrets()` (`src/kind:V14`)
V61: push per `Push`: `ReadOnly` → ⊥ cachix step, ⊥ push step, ⊥ push secret · `Forge` → cachix step as before, `CACHIX_AUTH_TOKEN` · cachix `Secret(N)` → that step w/ `authToken: ${{ secrets.N }}` · https `Secret(N)` → `push-https.sh` step, `env:` `NETRC: ${{ secrets.N }}` & `URL`, `if:` from mode (`SkipForkPr` ⊥ fork PR, `MainOnly`|`ExplicitMain` main push), ⊥ cachix step. mode `ReadOnly` → ⊥ push, ⊥ push secret, whatever the cache
V62: `push-https.sh`: `::add-mask::` URL before any output, ⊥ echo URL; ⊥ ATTR | ⊥ `URL` → exit 2; empty `NETRC` (fork) → skip, exit 0; `umask 077`, netrc → `$RUNNER_TEMP/netrc`, `trap` removes it; `nix copy --option netrc-file <it> --to "$URL" <installables>`

## §T TASKS

id|status|task|cites
T44|x|`.actlith` `## caches` section (columns name, public-key; `src/registry`) so a repo names its own cachix cache; built-in table stays the default. superseded by T79 (unreleased)|`src/registry:V10`
T79|.|render pipelith 0.2.1 `## caches` (its T41): `check` → `Row::check_with` → `Checked`; `nix.rs`, `cachix.rs` → here; read, push, secrets per V60–V62; `push-https.sh` tested under bash w/ stubs; goldens w/ `## caches` pass actionlint & zizmor (`tests:V4`), goldens w/o it unchanged|V60,V61,V62,`src/kind:V14`,`tests:V4`,`.:V18`

## §B BUGS

id|date|cause|fix
