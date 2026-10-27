#!/usr/bin/env bash
# nix:V42: hk evaluates hk.pkl with its own Pkl implementation, pklr. A
# construct pklr mishandles can make steps vanish or lose their command while
# pkl eval still lists them, leaving the gate to pass almost nothing.
set -euo pipefail

config="$(pkl eval --format json hk.pkl)"
for hook in check pre-commit commit-msg pre-push fix; do
  want="$(jq -r --arg hook "$hook" '.hooks[$hook].steps | to_entries[] | select(.value.check != null or .value.check_diff != null or .value.fix != null or .value.fix_diff != null) | .key' <<<"$config" | sort)"

  case "$hook" in
  check) seen_command=(hk check --check --all --plan --json) ;;
  fix) seen_command=(hk fix --all --plan --json) ;;
  commit-msg) seen_command=(hk run commit-msg "$(git rev-parse --git-path COMMIT_EDITMSG)" --plan --json) ;;
  *) seen_command=(hk run "$hook" --all --plan --json) ;;
  esac
  seen="$("${seen_command[@]}" |
    jq -r '.steps[] | select([.reasons[].kind] | index("no_command") | not) | .name' | sort)"

  if [ "$want" != "$seen" ]; then
    echo "hk-parity: hk's own Pkl evaluator does not see the $hook steps pkl eval sees (nix:V42):" >&2
    diff <(printf '%s\n' "$want") <(printf '%s\n' "$seen") |
      sed -n 's/^< /  missing in hk: /p; s/^> /  only in hk: /p' >&2
    exit 1
  fi
done
