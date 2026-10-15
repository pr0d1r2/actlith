# shellcheck shell=sh
#
# Stand-in for `gh api PATH --jq EXPR` in src/fetch unit tests: canned
# answers keyed by PATH, no network (src V4). Run as `sh fake-gh.sh api ...`.
case "$2" in
repos/actions/checkout/releases/latest)
  echo v7.0.2
  ;;
repos/actions/checkout/commits/v7.0.2)
  echo 0123456789abcdef0123456789abcdef01234567
  ;;
repos/example/norelease/releases/latest | repos/example/norelease/commits/*)
  echo 'gh: Not Found (HTTP 404)' >&2
  exit 1
  ;;
repos/example/denied/releases/latest)
  echo 'gh: Bad credentials (HTTP 401)' >&2
  exit 1
  ;;
*)
  echo "fake-gh: unexpected $*" >&2
  exit 1
  ;;
esac
