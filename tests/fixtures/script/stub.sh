# shellcheck shell=bash
#
# Test double for git, gh, nix, curl and actlith (src/script T21). The
# unit tests copy it under each tool's name, first on PATH, with a shebang
# naming the bash they found. Every call is appended to $STUB_LOG as
# "<tool> <args>"; the STUB_* variables below choose the answers.
set -euo pipefail

tool="${0##*/}"
printf '%s %s\n' "$tool" "$*" >>"$STUB_LOG"

case "$tool $*" in
"git ls-remote"*)
  # --exit-code: 0 when the branch exists on origin, 2 when it does not.
  if [ "${STUB_BRANCH_EXISTS:-}" = 1 ]; then exit 0; fi
  exit 2
  ;;
"git diff --cached --quiet"*)
  if [ "${STUB_NOTHING_STAGED:-}" = 1 ]; then exit 0; fi
  exit 1
  ;;
"nix flake update"*)
  : >"$STUB_LOG.updated"
  ;;
"nix eval --raw --impure"*)
  # With renamed nodes, `.nodes.nixpkgs` is a decoy that never moves while
  # the root's `nixpkgs-lock` follows the moving `.nodes.nixpkgs_2` node.
  if [ "${STUB_RENAMED_NODES:-}" = 1 ] &&
    [[ "$*" != *"nixpkgsLock = resolveInput"* ]]; then
    echo "${STUB_REV_BEFORE:-}"
    exit 0
  fi
  # The root-followed nixpkgs rev: BEFORE until `flake update`, then AFTER.
  if [ -e "$STUB_LOG.updated" ]; then
    echo "${STUB_REV_AFTER:-}"
  else
    echo "${STUB_REV_BEFORE:-}"
  fi
  ;;
"nix eval --raw .#packages."*)
  attr="${3#.#packages.}"
  sys="${attr%%.*}"
  printf '/nix/store/%s-%s\n' "${sys//-/}" "${attr#*.}"
  ;;
"nix build"*)
  if [ "${STUB_BUILD_FAILS:-}" = 1 ]; then exit 1; fi
  ;;
"curl "*)
  url="${*: -1}"
  case "$url" in
  *"${STUB_CURL_MISSING:-no-such-hash}"*) printf '404' ;;
  *) printf '200' ;;
  esac
  ;;
esac
