# shellcheck shell=sh
#
# Stand-in for `gh api PATH --jq EXPR` on PATH in tests/cli.rs: no action
# has a release (so each recorded tag stays) and every tag resolves to
# one SHA. No network (src V4).
case "$2" in
*/releases/latest)
  echo 'gh: Not Found (HTTP 404)' >&2
  exit 1
  ;;
*/commits/*)
  echo 0123456789abcdef0123456789abcdef01234567
  ;;
*)
  echo "fake-gh: unexpected $*" >&2
  exit 1
  ;;
esac
