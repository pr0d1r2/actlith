# shellcheck shell=bash
#
# read-netrc.sh
#
# Write the netrc file in $NETRC, the one a private https binary cache at
# $URL is read with, to $RUNNER_TEMP/read-netrc, mode 600, for nix's
# `netrc-file` (src/kind/cache/netrc V66). The file stays for the job's
# later steps, whose nix substitutes from the cache; the runner empties
# $RUNNER_TEMP after the job. The url can be a secret, so it is masked
# before anything is printed and never printed by this script. A run
# without the netrc secret (a fork's pull request) writes nothing and
# builds without the cache.
set -euo pipefail

if [ "$#" -ne 0 ]; then
  echo "usage: read-netrc.sh (no arguments; URL and NETRC come from env)" >&2
  exit 2
fi
if [ -z "${URL:-}" ]; then
  echo "::error::read-netrc.sh: URL is empty; name the cache url in .actlith." >&2
  exit 2
fi
echo "::add-mask::$URL"
if [ -z "${NETRC:-}" ]; then
  echo "read-netrc: no netrc secret on this run; nix reads the cache without it."
  exit 0
fi
case "$URL" in
https://*) ;;
*)
  # pipelith V87: a netrc goes only to an https url, checked here since a
  # secret url is not checked when the row is.
  echo "::error::read-netrc: cache \`${CACHE:-?}\`: its url does not start with https://, so the netrc from secret \`${NETRC_SECRET:-?}\` is not written." >&2
  exit 1
  ;;
esac

umask 077
printf '%s\n' "$NETRC" >"${RUNNER_TEMP:?RUNNER_TEMP is not set}/read-netrc"
