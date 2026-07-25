#!/usr/bin/env bash

set -euo pipefail

hooks_dir=$(git rev-parse --path-format=absolute --git-path hooks)
mkdir -p "$hooks_dir"

write_hook() {
  local hook_path=${1:?}
  local command=${2:?}

  {
    echo '#!/bin/sh'
    echo "$command"
  } >"$hook_path"
  chmod 0755 "$hook_path"
}

# These repository-owned hooks intentionally replace the previous Lefthook launchers.
write_hook "$hooks_dir/pre-commit" 'exec mise run pre-commit -- "$@"'
write_hook "$hooks_dir/pre-merge-commit" 'exec mise run pre-merge-commit -- "$@"'
write_hook "$hooks_dir/post-commit" 'exec mise run post-commit -- "$@"'
write_hook "$hooks_dir/commit-msg" 'exec mise run commit-msg -- "$@"'
