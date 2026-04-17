#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════
# Trinity ID AI OS — One-Click Launcher
# ═══════════════════════════════════════════════════════════════
# Usage: ./run-trinity.sh
#
# This script:
#   1. Checks for Pete (Gemma 4 on port 8001) and A.R.T.Y. Hub (port 8000)
#   2. Starts Trinity in headless mode
#   3. Opens the browser to http://localhost:3000
# ═══════════════════════════════════════════════════════════════

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TRINITY_BIN="$SCRIPT_DIR/trinity"
PORT=3000

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚂 Trinity ID AI OS — Starting...${NC}"
echo ""

# ─── Check if Trinity binary exists ──────────────────────────
if [ ! -f "$TRINITY_BIN" ]; then
    echo -e "${RED}❌ Trinity binary not found at: $TRINITY_BIN${NC}"
    echo "   Make sure you extracted the archive correctly."
    exit 1
fi

# ─── Check if port 3000 is already in use ────────────────────
if lsof -ti :$PORT > /dev/null 2>&1; then
    echo -e "${YELLOW}⚠️  Port $PORT is already in use.${NC}"
    echo "   Trinity may already be running. Opening browser..."
    xdg-open "http://localhost:$PORT" 2>/dev/null || echo "   → Open http://localhost:$PORT in your browser"
    exit 0
fi

# ─── Check for LLM backend ──────────────────────────────────
echo -n "Checking for LLM backends... "
LLM_FOUND=false

# Check Pete / Gemma 4 (vLLM on port 8001)
if curl -s --connect-timeout 2 http://127.0.0.1:8001/health > /dev/null 2>&1; then
    echo -e "${GREEN}✅ Pete / Gemma 4 detected on port 8001${NC}"
    LLM_FOUND=true
fi

# Check A.R.T.Y. Hub (vLLM reverse proxy — port 8000)
if curl -s --connect-timeout 2 http://127.0.0.1:8000/health > /dev/null 2>&1; then
    echo -e "${GREEN}✅ A.R.T.Y. Hub detected on port 8000${NC}"
else
    echo -e "${YELLOW}⬚  A.R.T.Y. Hub not running (port 8000 — embeddings/RAG degraded)${NC}"
fi

if [ "$LLM_FOUND" = false ]; then
    echo -e "${YELLOW}⚠️  No LLM backend detected${NC}"
    echo ""
    echo -e "   Trinity needs Pete (Gemma 4 E4B AWQ) on port 8001 to function."
    echo -e "   Start via:"
    echo -e "   ${BLUE}./scripts/launch/launch_pete.sh${NC}"
    echo -e ""
    echo -e "   For A.R.T.Y. Hub (embeddings/RAG):"
    echo -e "   ${BLUE}./scripts/launch/launch_arty_hub.sh${NC}"
    echo ""
    read -p "Start Trinity anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# ─── Start Trinity ───────────────────────────────────────────
echo ""
echo -e "${BLUE}Starting Trinity server on port $PORT...${NC}"

export TRINITY_HEADLESS=1
"$TRINITY_BIN" &
TRINITY_PID=$!

# Wait for server to be ready (up to 30 seconds)
echo -n "Waiting for server to be ready"
for i in $(seq 1 30); do
    if curl -s --connect-timeout 1 http://127.0.0.1:$PORT/api/health > /dev/null 2>&1; then
        echo ""
        echo -e "${GREEN}✅ Trinity is running at http://localhost:$PORT${NC}"
        break
    fi
    echo -n "."
    sleep 1
done

# Check if server started successfully
if ! kill -0 $TRINITY_PID 2>/dev/null; then
    echo ""
    echo -e "${RED}❌ Trinity failed to start. Check the terminal output above for errors.${NC}"
    exit 1
fi

# ─── Open browser ────────────────────────────────────────────
echo ""
echo -e "${GREEN}🌐 Opening browser...${NC}"
xdg-open "http://localhost:$PORT" 2>/dev/null || echo -e "   → Open ${BLUE}http://localhost:$PORT${NC} in your browser"

echo ""
echo -e "${GREEN}═══════════════════════════════════════════════════${NC}"
echo -e "${GREEN}  Trinity ID AI OS is running!${NC}"
echo -e "${GREEN}  Press Ctrl+C to stop the server.${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════${NC}"
echo ""

# Wait for Trinity process
wait $TRINITY_PID
