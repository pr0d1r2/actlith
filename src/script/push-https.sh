# shellcheck shell=bash
#
# push-https.sh ATTR...
#
# Copy each ATTR's closure to the https binary cache at $URL, over the
# authenticated HTTP PUT the netrc file in $NETRC allows (src/kind/cache
# V61, V62). An ATTR is a flake output path without its system,
# `<output>.<rest>`: `packages.hk` copies `.#packages.<system>.hk`, the
# system being this runner's; each `<output>.<system>.<name>` must
# evaluate, or the script fails naming it before any netrc is written.
# The url can be a secret, so it is masked
# before anything is printed and never printed by this script; nix's
# own output is masked by the runner. A run without the netrc secret (a
# fork's pull request) skips the push: the cache is still read.
set -euo pipefail

if [ "$#" -lt 1 ]; then
  echo "usage: push-https.sh ATTR..." >&2
  exit 2
fi
if [ -z "${URL:-}" ]; then
  echo "::error::push-https.sh: URL is empty; name the cache url in .actlith." >&2
  exit 2
fi
echo "::add-mask::$URL"
if [ -z "${NETRC:-}" ]; then
  echo "push-https: no netrc secret on this run; the push is skipped."
  exit 0
fi

system="$(nix eval --impure --raw --expr builtins.currentSystem)"
installables=()
for attr in "$@"; do
  rest="${attr#*.}"
  # The flake output the attribute lives in, `<output>.<system>.<name>`:
  # missing, it fails here by name, not halfway through `nix copy`.
  output="${attr%%.*}.$system.${rest%%.*}"
  if ! nix eval ".#$output" --apply 'x: true' >/dev/null; then
    echo "::error::push-https: the flake has no output \`$output\` to push to the https cache." >&2
    echo "::error::Fix: add \`${attr%%.*}.<system>.${rest%%.*}\` to flake.nix (every flake keeps a default cacheable output), or set cache-push=false on the row." >&2
    exit 1
  fi
  installables+=(".#${attr%%.*}.$system.$rest")
done

umask 077
netrc="${RUNNER_TEMP:?RUNNER_TEMP is not set}/netrc"
trap 'rm -f "$netrc"' EXIT
printf '%s\n' "$NETRC" >"$netrc"
nix copy --option netrc-file "$netrc" --to "$URL" "${installables[@]}"
