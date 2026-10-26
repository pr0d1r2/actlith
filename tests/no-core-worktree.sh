#!/usr/bin/env bash
set -euo pipefail

root="$(git rev-parse --show-toplevel)"
guard="$root/nix/guard/no-core-worktree.sh"
fixture="$(mktemp -d)"
trap 'rm -rf "$fixture"' EXIT

git -C "$fixture" init -q
git -C "$fixture" config user.email test@example.invalid
git -C "$fixture" config user.name test

if ! (cd "$fixture" && bash "$guard"); then
  echo "no-core-worktree proof: clean repository was rejected" >&2
  exit 1
fi

git -C "$fixture" config --local core.worktree "$fixture"
if (cd "$fixture" && bash "$guard"); then
  echo "no-core-worktree proof: planted core.worktree was accepted" >&2
  exit 1
fi
