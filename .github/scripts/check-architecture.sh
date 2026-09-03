#!/usr/bin/env bash
set -euo pipefail

metadata="$(cargo metadata --format-version 1 --no-deps)"
edges="$({
  jq -r '
    .packages[]
    | select(.source == null)
    | .name as $from
    | .dependencies[]
    | select(.path != null)
    | "\($from) \(.name)"
  ' <<<"$metadata"
})"

reject_edge() {
  local from_pattern="$1"
  local to_pattern="$2"
  local description="$3"
  if grep -Eq "^(${from_pattern}) (${to_pattern})$" <<<"$edges"; then
    echo "architecture dependency violation: ${description}" >&2
    grep -E "^(${from_pattern}) (${to_pattern})$" <<<"$edges" >&2
    exit 1
  fi
}

reject_edge 'ww-types|ww-store|ww-store-sqlite|ww-runtime' 'ww-agent-.*|ww-flow-.*' \
  'shared runtime depends on an engine-specific crate'
reject_edge 'ww-agent-provider' 'ww-runtime|ww-store|ww-store-sqlite|ww-flow-.*' \
  'provider contracts depend on runtime, persistence, or Flow'
reject_edge 'ww-agent-core' 'ww-runtime|ww-store|ww-store-sqlite|ww-agent-store-sqlite|ww-flow-.*' \
  'Agent core depends on concrete runtime, persistence, or Flow'
reject_edge 'ww-agent-.*' 'ww-flow-.*' \
  'Agent crate depends on Flow'
reject_edge 'ww-cli' 'ww-store|ww-store-sqlite|ww-agent-store-sqlite' \
  'CLI bypasses the SDK/runtime boundary'
