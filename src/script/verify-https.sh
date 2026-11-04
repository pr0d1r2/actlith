# shellcheck shell=bash
#
# verify-https.sh PACKAGE SYSTEM...
#
# Every SYSTEM's `.#packages.<system>.<PACKAGE>` output path answers 200
# from the https binary cache at $URL, as `<url>/<hash>.narinfo` (kind
# `build` on an https cache, src/build V64). A cache that wants
# credentials to read gets the netrc file in $NETRC when it is set. The
# url can be a secret, so it is masked before anything is printed and
# never printed by this script. Evaluation is cross-system: one runner
# resolves every path without building any of them.
set -euo pipefail

if [ "$#" -lt 2 ]; then
  echo "usage: verify-https.sh PACKAGE SYSTEM..." >&2
  exit 2
fi
if [ -z "${URL:-}" ]; then
  echo "::error::verify-https.sh: URL is empty; name the cache url in .actlith." >&2
  exit 2
fi
echo "::add-mask::$URL"
package="$1"
shift

auth=()
if [ -n "${NETRC:-}" ]; then
  umask 077
  netrc="${RUNNER_TEMP:?RUNNER_TEMP is not set}/netrc"
  trap 'rm -f "$netrc"' EXIT
  printf '%s\n' "$NETRC" >"$netrc"
  auth=(--netrc-file "$netrc")
fi

rc=0
for sys in "$@"; do
  path="$(nix eval --raw ".#packages.$sys.$package")"
  name="${path##*/}"
  hash="${name%%-*}"
  code="$(curl -s -o /dev/null -w '%{http_code}' "${auth[@]}" "${URL%/}/$hash.narinfo" || true)"
  echo "verify-cache: $sys $hash -> $code"
  if [ "$code" != 200 ]; then
    rc=1
  fi
done

if [ "$rc" -ne 0 ]; then
  echo "::error::$package missing from the https cache after a main build." >&2
  echo "::error::Check the push secret: a netrc file allowed to PUT to the" >&2
  echo "::error::cache. A failed push does not always fail its job." >&2
  exit 1
fi
