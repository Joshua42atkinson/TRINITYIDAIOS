#!/bin/bash
# 📦 Create a clean source archive of Trinity ID AI OS
# Designed for academic review — professors can feed this to their own AI.
#
# Excludes: target/, node_modules/, .git/, model files (68GB+), archive/legacy
# Includes: all source, docs, configs, frontend dist, scripts, quests, migrations

set -euo pipefail

VERSION="1.0"
ARCHIVE_NAME="TRINITY_ID_AI_OS_v${VERSION}_source"
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUTPUT_DIR="${PROJECT_ROOT}/dist"
ARCHIVE_PATH="${OUTPUT_DIR}/${ARCHIVE_NAME}.tar.gz"

echo "📦 Creating Trinity ID AI OS source archive v${VERSION}..."
echo "   Project root: ${PROJECT_ROOT}"

# Create output directory
mkdir -p "${OUTPUT_DIR}"

# Build the archive using tar with exclusions
cd "${PROJECT_ROOT}"

tar -czf "${ARCHIVE_PATH}" \
    --exclude='./target' \
    --exclude='./dist' \
    --exclude='./.git' \
    --exclude='./node_modules' \
    --exclude='./LDTAtkinson/client/node_modules' \
    --exclude='./llama.cpp' \
    --exclude='./archive/legacy-sandbox' \
    --exclude='./windsurf-conductor-mcp' \
    --exclude='./nohup.out' \
    --exclude='*.gguf' \
    --exclude='*.safetensors' \
    --exclude='*.bin' \
    --exclude='*.m4a' \
    --exclude='*.mp4' \
    --exclude='*.mp3' \
    --exclude='*.wav' \
    --exclude='*.pth' \
    --exclude='*.pt' \
    --exclude='./scripts/launch/venv' \
    --exclude='./LDTAtkinson/*.mp4' \
    --exclude='./LDTAtkinson/*.m4a' \
    --exclude='./.windsurf' \
    --exclude='./.cursorrules' \
    --exclude='./windsurf-config.json' \
    --exclude='./trinity_key' \
    --exclude='./trinity_key.pub' \
    --transform "s|^\.|${ARCHIVE_NAME}|" \
    .

# Report results
ARCHIVE_SIZE=$(du -h "${ARCHIVE_PATH}" | cut -f1)
FILE_COUNT=$(tar -tzf "${ARCHIVE_PATH}" | wc -l)

echo ""
echo "✅ Archive created successfully!"
echo "   📁 Path:  ${ARCHIVE_PATH}"
echo "   📏 Size:  ${ARCHIVE_SIZE}"
echo "   📄 Files: ${FILE_COUNT}"
echo ""
echo "To extract:"
echo "   tar -xzf ${ARCHIVE_PATH}"
echo "   cd ${ARCHIVE_NAME}"
echo "   cat INSTALL.md"
