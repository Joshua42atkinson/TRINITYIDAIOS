#!/bin/bash
# Trinity Root Hygiene — Move support docs to docs/, keep Sacred Four + README in root
# Run from: ~/Workflow/desktop_trinity/trinity-genesis/

set -e
cd "$(dirname "$0")"

echo "📂 Trinity Root Cleanup..."

# Ensure docs/ exists
mkdir -p docs

# Move support docs to docs/ (only if they exist in root)
for f in CONTEXT.md ADDIECRAPEYE_CANONICAL.md HOW_TO_USE_TRINITY.md INSTALL.md \
         LM_STUDIO_SETUP.md SESSION_TURNOVER.md TRINITY_QUICKSTART.md; do
    if [ -f "$f" ]; then
        mv "$f" "docs/$f"
        echo "  → docs/$f"
    fi
done

# Handle lowercase duplicate
if [ -f "context.md" ]; then
    mv "context.md" "docs/context_old.md"
    echo "  → docs/context_old.md (old lowercase duplicate)"
fi

echo ""
echo "✅ Root is clean. Remaining .md files:"
ls -1 *.md
echo ""
echo "📚 docs/ contents:"
ls -1 docs/*.md
