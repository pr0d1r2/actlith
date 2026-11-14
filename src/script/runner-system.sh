# shellcheck shell=bash
#
# runner-system.sh SYSTEM
#
# Stop unless this runner's nix really builds SYSTEM, the system the job's
# row names (src/kind V67). Native: the runner's own `system` is SYSTEM.
# Emulated (Rosetta, binfmt): SYSTEM is in nix's `extra-platforms` AND the
# row chose it with `emulate=SYSTEM`, which sets EMULATE=true; then every
# later step runs with `system = SYSTEM`, so it evaluates and builds
# SYSTEM, not the host. Anything else fails, naming the fix, rather than
# going green on the wrong system.
set -euo pipefail

if [ "$#" -ne 1 ]; then
  echo "usage: runner-system.sh SYSTEM" >&2
  exit 2
fi
want="$1"
have="$(nix eval --impure --raw --expr builtins.currentSystem)"
if [ "$have" = "$want" ]; then
  echo "runner-system: native $want"
  exit 0
fi

extra="$(nix config show extra-platforms 2>/dev/null || true)"
case " $extra " in
*" $want "*) ;;
*)
  echo "::error::runner-system: this runner builds $have and its nix does not list $want in extra-platforms, so a job for $want would not build it." >&2
  echo "::error::Fix: point the \`## runners\` row at a runner of $want, or enable Rosetta/binfmt for it and name it \`emulate=$want\` on the row." >&2
  exit 1
  ;;
esac
if [ "${EMULATE:-}" != "true" ]; then
  echo "::error::runner-system: this runner builds $have; $want would be emulated, which the row did not choose." >&2
  echo "::error::Fix: add \`emulate=$want\` to the row's args, or use a native runner for $want." >&2
  exit 1
fi

echo "runner-system: emulating $want on $have"
# A later step reads this: nix then evaluates and builds $want.
delim="ACTLITH_NIX_CONFIG_$$"
{
  echo "NIX_CONFIG<<$delim"
  if [ -n "${NIX_CONFIG:-}" ]; then
    echo "$NIX_CONFIG"
  fi
  echo "system = $want"
  echo "$delim"
} >>"${GITHUB_ENV:?GITHUB_ENV is not set}"
