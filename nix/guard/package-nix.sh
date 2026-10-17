#!/usr/bin/env bash
#
# The package consumers get is tested as built, not only the source tree
# (actlith nix T32, as pklith root T62). It builds the flake's actlith with
# nix and runs the built binary through the core loop on a fixture
# repository: detect, seed --init, gen, gen --check, check, and a drifted
# file that gen --check must refuse.
#
#   nix/guard/package-nix.sh
set -euo pipefail

root="$(git rev-parse --show-toplevel)"
unset GIT_DIR GIT_INDEX_FILE GIT_WORK_TREE GIT_PREFIX GIT_OBJECT_DIRECTORY \
  GIT_ALTERNATE_OBJECT_DIRECTORIES GIT_COMMON_DIR

fail() {
  echo "package-nix: $*" >&2
  exit 1
}

# `git+file:` builds from tracked files only; a bare path would copy
# target/ while cargo steps rewrite it (nix B8).
out="$(nix build "git+file://$root#default" --no-link --print-out-paths)"
actlith="$out/bin/actlith"
[ -x "$actlith" ] || fail "$actlith is not an executable"

expect() {
  local what="$1" want="$2" got="$3"
  [ "$got" = "$want" ] || fail "$what: expected $(printf '%q' "$want"), got $(printf '%q' "$got")"
}

version="$(sed -n 's/^version = "\(.*\)"/\1/p' "$root/Cargo.toml" | head -1)"
expect "actlith --version" "actlith $version" "$("$actlith" --version)"

fixture="$(mktemp -d)"
trap 'rm -rf "$fixture"' EXIT
git -C "$fixture" init -q
printf '{ outputs = _: { }; }\n' >"$fixture/flake.nix"
git -C "$fixture" add -A

run() { (cd "$fixture" && "$actlith" "$@"); }

expect "actlith detect" "ci|gate|-|-|-" "$(run detect)"
run seed --init >/dev/null || fail "seed --init failed on a fresh repository"
run gen >/dev/null || fail "gen failed"
[ -s "$fixture/.github/workflows/ci.yml" ] || fail "gen wrote no .github/workflows/ci.yml"
git -C "$fixture" add -A
run gen --check || fail "gen --check refused what gen just wrote"
run check || fail "check refused a repository gen just wrote"

printf '# edited by hand\n' >>"$fixture/.github/workflows/ci.yml"
if run gen --check 2>/dev/null; then
  fail "gen --check passed a hand-edited workflow"
fi
