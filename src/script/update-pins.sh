# shellcheck shell=bash
#
# update-pins.sh [BUILD]
#
# Follow nixpkgs-lock and propose the bump as a pull request (kind
# `update-pins`). Pull model: this repository polls; nixpkgs-lock holds no
# cross-repository token.
#
#   1. update the `nixpkgs-lock` input; nixpkgs did not move -> exit 0
#   2. branch `pins/nixpkgs-<short rev>` already on origin -> exit 0
#      (src/bump:V26: an earlier run opened that PR)
#   3. build BUILD (default `.#default`) against the new pin: a bump that
#      breaks the build is found here, not after merge
#   4. `actlith pin` and `actlith gen`, so action pins and a new actlith
#      arrive through the same reviewed PR ($ACTLITH names the command)
#   5. commit, push and open the PR with open-pr.sh beside this script
set -euo pipefail

build="${1:-.#default}"
actlith="${ACTLITH:-actlith}"
here="$(dirname "${BASH_SOURCE[0]}")"

nixpkgs_rev() {
  nix eval --raw --impure --expr \
    '(builtins.fromJSON (builtins.readFile ./flake.lock)).nodes.nixpkgs.locked.rev'
}

before="$(nixpkgs_rev)"
nix flake update nixpkgs-lock
after="$(nixpkgs_rev)"
if [ "$before" = "$after" ]; then
  echo "update-pins: nixpkgs still at $after, nothing to do"
  exit 0
fi

short="${after:0:12}"
branch="pins/nixpkgs-$short"
if git ls-remote --exit-code --heads origin "$branch" >/dev/null 2>&1; then
  echo "update-pins: branch $branch already exists, nothing to do"
  exit 0
fi

nix build --no-link --print-build-logs "$build"
"$actlith" pin
"$actlith" gen

title="build: follow nixpkgs-lock to $short"
body="nixpkgs-lock moved \`$before\` -> \`$after\`.

\`$build\` was built against the new pin, and \`actlith pin\` and
\`actlith gen\` ran in this branch, before this PR was opened."
bash "$here/open-pr.sh" "$branch" "$title" "$body" \
  flake.lock .actlith .github/workflows scripts/actlith
