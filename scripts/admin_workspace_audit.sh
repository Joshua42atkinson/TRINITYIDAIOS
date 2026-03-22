#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="${1:-.}"
cd "$ROOT_DIR"

echo "== Trinity Workspace Audit =="
echo "Root: $(pwd)"
echo

echo "-- Root-level untracked files --"
git ls-files --others --exclude-standard | awk -F/ 'NF==1 {print}' || true
echo

echo "-- Root-level markdown/txt files (visibility) --"
find . -maxdepth 1 -type f \( -name "*.md" -o -name "*.txt" \) -printf "%f\n" | sort
echo

echo "-- Large untracked directories (top level) --"
while IFS= read -r d; do
  size=$(du -sh "$d" 2>/dev/null | awk '{print $1}')
  echo "$size  $d"
done < <(find . -maxdepth 1 -mindepth 1 -type d -printf "%f\n" | sort)
echo

echo "-- Working tree summary --"
git status --short | sed -n '1,200p'
