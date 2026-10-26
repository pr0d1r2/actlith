#!/usr/bin/env bash
#
# A worktree-specific core.worktree in .git/config is shared by every
# parallel worker. Refuse it before hk can operate on the wrong checkout
# (actlith nix V40).
#
#   nix/guard/no-core-worktree.sh
set -euo pipefail

if [ "$(git rev-parse --is-inside-work-tree 2>/dev/null || true)" != true ]; then
  echo "no-core-worktree: not inside a git work tree" >&2
  exit 1
fi

if [ -n "$(git config --local --get core.worktree || true)" ]; then
  echo "no-core-worktree: local core.worktree is set; remove it from .git/config" >&2
  exit 1
fi
