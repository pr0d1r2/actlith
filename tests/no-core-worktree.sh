#!/usr/bin/env bash
set -euo pipefail

root="$(git rev-parse --show-toplevel)"
guard="$root/nix/guard/no-core-worktree.sh"
fixture="$(mktemp -d)"
trap 'rm -rf "$fixture"' EXIT

git_scratch=(env -u GIT_DIR -u GIT_WORK_TREE -u GIT_INDEX_FILE
  -u GIT_OBJECT_DIRECTORY -u GIT_ALTERNATE_OBJECT_DIRECTORIES
  -u GIT_COMMON_DIR git)

"${git_scratch[@]}" -C "$fixture" init -q
"${git_scratch[@]}" -C "$fixture" config --local core.hooksPath /dev/null
"${git_scratch[@]}" -C "$fixture" config user.email test@example.invalid
"${git_scratch[@]}" -C "$fixture" config user.name test

bare_fixture="$(mktemp -d)"
"${git_scratch[@]}" -C "$bare_fixture" init --bare -q
"${git_scratch[@]}" -C "$bare_fixture" config --local core.hooksPath /dev/null
if (cd "$bare_fixture" && env -u GIT_DIR -u GIT_WORK_TREE -u GIT_INDEX_FILE \
  -u GIT_OBJECT_DIRECTORY -u GIT_ALTERNATE_OBJECT_DIRECTORIES -u GIT_COMMON_DIR \
  bash "$guard"); then
  echo "no-core-worktree proof: bare repository was accepted" >&2
  exit 1
fi

if ! (cd "$fixture" && env -u GIT_DIR -u GIT_WORK_TREE -u GIT_INDEX_FILE \
  -u GIT_OBJECT_DIRECTORY -u GIT_ALTERNATE_OBJECT_DIRECTORIES -u GIT_COMMON_DIR \
  bash "$guard"); then
  echo "no-core-worktree proof: clean repository was rejected" >&2
  exit 1
fi

"${git_scratch[@]}" -C "$fixture" config --local core.worktree "$fixture"
if (cd "$fixture" && env -u GIT_DIR -u GIT_WORK_TREE -u GIT_INDEX_FILE \
  -u GIT_OBJECT_DIRECTORY -u GIT_ALTERNATE_OBJECT_DIRECTORIES -u GIT_COMMON_DIR \
  bash "$guard"); then
  echo "no-core-worktree proof: planted core.worktree was accepted" >&2
  exit 1
fi
