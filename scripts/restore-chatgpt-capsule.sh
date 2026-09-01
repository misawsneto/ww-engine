#!/usr/bin/env bash
set -euo pipefail

ARCHIVE="${1:-ww-search-rag-kb-dev-capsule.tar.gz}"
DEST="${2:-/mnt/data/ww-search-rag-kb}"

if [[ ! -f "$ARCHIVE" ]]; then
  echo "Capsule archive not found: $ARCHIVE" >&2
  exit 1
fi

if [[ -e "$DEST" ]] && find "$DEST" -mindepth 1 -maxdepth 1 -print -quit | grep -q .; then
  echo "Destination is not empty: $DEST" >&2
  echo "Refusing to overwrite an existing workspace." >&2
  exit 1
fi

TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

tar -xzf "$ARCHIVE" -C "$TMP"
SRC="$TMP/ww-search-rag-kb"
MANIFEST="$SRC/capsule-manifest.json"
TRACKED="$SRC/capsule-tracked-files.zlist"

[[ -f "$MANIFEST" ]] || { echo "Capsule manifest is missing." >&2; exit 1; }
[[ -f "$TRACKED" ]] || { echo "Capsule tracked-file list is missing." >&2; exit 1; }

BASE_COMMIT="$(sed -n 's/^[[:space:]]*"base_commit":[[:space:]]*"\([0-9a-fA-F]*\)".*/\1/p' "$MANIFEST")"
if [[ ! "$BASE_COMMIT" =~ ^[0-9a-fA-F]{40,64}$ ]]; then
  echo "Capsule manifest has an invalid base_commit." >&2
  exit 1
fi

mkdir -p "$(dirname "$DEST")"
if [[ -e "$DEST" ]]; then
  rmdir "$DEST"
fi
mv "$SRC" "$DEST"
trap - EXIT
rm -rf "$TMP"

cd "$DEST"
git init -q -b main
printf '%s\n' '/capsule-manifest.json' '/capsule-tracked-files.zlist' >> .git/info/exclude
git add --pathspec-from-file=capsule-tracked-files.zlist --pathspec-file-nul
git -c user.name='ChatGPT Sandbox' -c user.email='sandbox@localhost' commit -q -m "baseline: GitHub main $BASE_COMMIT"

if [[ -n "$(git status --short)" ]]; then
  echo "Restored workspace is not clean after baseline initialization." >&2
  git status --short >&2
  exit 1
fi

printf 'restored_workspace=%s\n' "$DEST"
printf 'base_commit=%s\n' "$BASE_COMMIT"
printf 'local_baseline_commit=%s\n' "$(git rev-parse HEAD)"
