#!/usr/bin/env bash
#
# The vendored hk schema is the one the pinned hk ships (actlith nix T58,
# as pklith V28). hk evaluates hk.pkl against pkl/Config.pkl; a copy from
# another hk version type-checks a config hk itself would read differently.
# The pinned hk's own source is the reference, and pkl/VENDORED.md must
# name that version and the copy's sha256.
#
#   nix/guard/schema-pin.sh
set -euo pipefail

root="$(git rev-parse --show-toplevel)"
cd "$root"

fail() {
  echo "schema-pin: $*" >&2
  echo "  fix: copy Config.pkl from the pinned hk's release and update pkl/VENDORED.md in the commit that moves the pin (nix T58)." >&2
  exit 1
}

# The hk this flake's dev shell runs: nix-hk's package for this system.
# The `${...}` is Nix interpolation, for nix to expand, not the shell.
# shellcheck disable=SC2016
hk='(builtins.getFlake (toString ./.)).inputs.nix-hk.packages.${builtins.currentSystem}.default'
src="$(nix build --impure --no-link --print-out-paths --expr "$hk.src")"
version="$(nix eval --impure --raw --expr "$hk.version")"

cmp -s "$src/pkl/Config.pkl" pkl/Config.pkl ||
  fail "pkl/Config.pkl differs from the schema hk $version ships"
grep -q "hk $version release" pkl/VENDORED.md ||
  fail "pkl/VENDORED.md does not name hk $version"
sum="$(sha256sum pkl/Config.pkl | cut -d' ' -f1)"
grep -q "\`Config.pkl\` sha256: $sum" pkl/VENDORED.md ||
  fail "pkl/VENDORED.md does not carry Config.pkl's sha256 $sum"
