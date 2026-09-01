#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
cd "$ROOT"

STAGE="$(mktemp -d)"
trap 'rm -rf "$STAGE"' EXIT
DEST="$STAGE/ww-search-rag-kb"
mkdir -p "$DEST"

while IFS= read -r -d '' path; do
  mkdir -p "$DEST/$(dirname "$path")"
  cp -a "$path" "$DEST/$path"
done < <(git ls-files -z)

git ls-files -z > "$DEST/capsule-tracked-files.zlist"

CREATED_AT="$(date -u +'%Y-%m-%dT%H:%M:%SZ')"

cat > "$DEST/capsule-manifest.json" <<JSON
{
  "schema": "ww-search-rag-kb/chatgpt-dev-capsule/v1",
  "repository": "${GITHUB_REPOSITORY:-misawsneto/ww-search-rag-kb}",
  "base_commit": "$(git rev-parse HEAD)",
  "created_at": "$CREATED_AT",
  "archive_root": "ww-search-rag-kb"
}
JSON

OUTPUT="$ROOT/ww-search-rag-kb-dev-capsule.tar.gz"
tar --owner=0 --group=0 -C "$STAGE" -czf "$OUTPUT" ww-search-rag-kb

SIZE_BYTES="$(stat -c '%s' "$OUTPUT")"
MAX_BYTES=$((200 * 1024 * 1024))
if (( SIZE_BYTES > MAX_BYTES )); then
  echo "Capsule is ${SIZE_BYTES} bytes, above the 200 MiB ceiling." >&2
  rm -f "$OUTPUT"
  exit 1
fi

printf 'capsule_path=%s\n' "$OUTPUT"
printf 'capsule_size_bytes=%s\n' "$SIZE_BYTES"
printf 'base_commit=%s\n' "$(git rev-parse HEAD)"
