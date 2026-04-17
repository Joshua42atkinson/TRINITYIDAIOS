#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════════════════════
# TRINITY ID AI OS — Desktop Ignition (One-Click Launch)
# ═══════════════════════════════════════════════════════════════════════════════
#
# PURPOSE: Serial startup of all Trinity services in correct dependency order:
#   1. Pete / Gemma 4 E4B AWQ (vLLM on :8001)
#   2. A.R.T.Y. Hub (FastAPI reverse proxy on :8000)
#   3. Trinity Rust Backend (Axum on :3000)
#   4. Open browser
#
# USAGE:
#   ./scripts/launch/trinity_ignition.sh          # Full startup
#   ./scripts/launch/trinity_ignition.sh --skip-ai # Trinity only (no LLM)
#   ./scripts/launch/trinity_ignition.sh --status  # Check status of all services
#
# MATURITY: L3 — Functional orchestrator (was L1 planned-only)
#
# ═══════════════════════════════════════════════════════════════════════════════

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

# Ports
PETE_PORT=8001
ARTY_PORT=8000
NOMIC_PORT=8005
TRINITY_PORT=3000

# ── Functions ─────────────────────────────────────────────────────────────────

check_port() {
    local port=$1
    curl -s --connect-timeout 2 "http://127.0.0.1:${port}/health" > /dev/null 2>&1
}

wait_for_port() {
    local port=$1
    local name=$2
    local max_wait=${3:-120}
    local elapsed=0

    echo -n "   Waiting for ${name} on :${port}"
    while [ $elapsed -lt $max_wait ]; do
        if check_port "$port"; then
            echo -e " ${GREEN}✓${NC} (${elapsed}s)"
            return 0
        fi
        echo -n "."
        sleep 2
        elapsed=$((elapsed + 2))
    done

    echo -e " ${RED}✗ TIMEOUT after ${max_wait}s${NC}"
    return 1
}

print_status() {
    echo -e "\n${BOLD}${CYAN}═══════════════════════════════════════════════════${NC}"
    echo -e "${BOLD}${CYAN}  Trinity ID AI OS — Service Status${NC}"
    echo -e "${BOLD}${CYAN}═══════════════════════════════════════════════════${NC}\n"

    if check_port $PETE_PORT; then
        echo -e "  ${GREEN}✅${NC} Pete / Gemma 4 E4B AWQ          :${PETE_PORT}"
    else
        echo -e "  ${RED}❌${NC} Pete / Gemma 4 E4B AWQ          :${PETE_PORT}  (offline)"
    fi

    if check_port $ARTY_PORT; then
        echo -e "  ${GREEN}✅${NC} A.R.T.Y. Hub                    :${ARTY_PORT}"
    else
        echo -e "  ${YELLOW}⬚ ${NC} A.R.T.Y. Hub                    :${ARTY_PORT}  (not running)"
    fi

    if check_port $NOMIC_PORT; then
        echo -e "  ${GREEN}✅${NC} nomic-embed (Research)           :${NOMIC_PORT}"
    else
        echo -e "  ${YELLOW}⬚ ${NC} nomic-embed (Research)           :${NOMIC_PORT}  (not running)"
    fi

    if check_port $TRINITY_PORT; then
        echo -e "  ${GREEN}✅${NC} Trinity Backend                  :${TRINITY_PORT}"
    else
        echo -e "  ${RED}❌${NC} Trinity Backend                  :${TRINITY_PORT}  (offline)"
    fi

    echo ""
}

# ── Parse Arguments ───────────────────────────────────────────────────────────

SKIP_AI=false
STATUS_ONLY=false

for arg in "$@"; do
    case $arg in
        --skip-ai)
            SKIP_AI=true
            ;;
        --status)
            STATUS_ONLY=true
            ;;
        --help|-h)
            echo "Trinity Ignition — One-Click Launch"
            echo ""
            echo "Usage: $0 [options]"
            echo ""
            echo "Options:"
            echo "  --skip-ai    Skip AI sidecar startup (Trinity only)"
            echo "  --status     Show service status and exit"
            echo "  --help       Show this help"
            exit 0
            ;;
    esac
done

if [ "$STATUS_ONLY" = true ]; then
    print_status
    exit 0
fi

# ── Banner ────────────────────────────────────────────────────────────────────

echo -e "\n${BOLD}${BLUE}🚂 ═══════════════════════════════════════════════════${NC}"
echo -e "${BOLD}${BLUE}   TRINITY ID AI OS — Desktop Ignition${NC}"
echo -e "${BOLD}${BLUE}   One-Click Launch Sequence${NC}"
echo -e "${BOLD}${BLUE}═══════════════════════════════════════════════════════${NC}\n"

# ── Stage 1: Pete / Gemma 4 E4B AWQ (vLLM :8001) ─────────────────────────────

if [ "$SKIP_AI" = false ]; then
    echo -e "${BOLD}[1/3] Pete / Gemma 4 E4B AWQ${NC}"

    if check_port $PETE_PORT; then
        echo -e "   ${GREEN}Already running on :${PETE_PORT}${NC}"
    else
        LAUNCH_SCRIPT="$SCRIPT_DIR/launch_pete.sh"
        if [ -f "$LAUNCH_SCRIPT" ]; then
            echo -e "   Launching via distrobox → vllm..."
            # Launch in background — vLLM takes 15-60s to load the model
            bash "$LAUNCH_SCRIPT" &
            PETE_PID=$!

            # Wait up to 120s for model to load
            if wait_for_port $PETE_PORT "Pete/Gemma 4" 120; then
                echo -e "   ${GREEN}Pete is online!${NC}"
            else
                echo -e "   ${YELLOW}Proceeding without Pete — model may still be loading${NC}"
                echo -e "   ${YELLOW}Monitor: curl http://127.0.0.1:${PETE_PORT}/health${NC}"
            fi
        else
            echo -e "   ${YELLOW}Launch script not found: $LAUNCH_SCRIPT${NC}"
            echo -e "   ${YELLOW}Start manually: ./scripts/launch/launch_pete.sh${NC}"
        fi
    fi

    echo ""

    # ── Stage 2: A.R.T.Y. Hub (FastAPI :8000) ────────────────────────────────

    echo -e "${BOLD}[2/3] A.R.T.Y. Hub (Embeddings + Research)${NC}"

    if check_port $ARTY_PORT; then
        echo -e "   ${GREEN}Already running on :${ARTY_PORT}${NC}"
    else
        ARTY_SCRIPT="$SCRIPT_DIR/launch_arty_hub.sh"
        if [ -f "$ARTY_SCRIPT" ]; then
            echo -e "   Launching A.R.T.Y. Hub..."
            bash "$ARTY_SCRIPT" &

            if wait_for_port $ARTY_PORT "A.R.T.Y. Hub" 30; then
                echo -e "   ${GREEN}A.R.T.Y. Hub is online!${NC}"
            else
                echo -e "   ${YELLOW}A.R.T.Y. Hub not responding — RAG will use text-only fallback${NC}"
            fi
        else
            echo -e "   ${YELLOW}Launch script not found: $ARTY_SCRIPT${NC}"
        fi
    fi
else
    echo -e "${YELLOW}[1-2/3] Skipping AI sidecars (--skip-ai)${NC}"
fi

echo ""

# ── Stage 3: Trinity Rust Backend (Axum :3000) ───────────────────────────────

echo -e "${BOLD}[3/3] Trinity Backend${NC}"

if check_port $TRINITY_PORT; then
    echo -e "   ${GREEN}Already running on :${TRINITY_PORT}${NC}"
else
    echo -e "   Building and starting Trinity..."

    # Check if release binary exists (prefer release for speed)
    TRINITY_BIN="$PROJECT_ROOT/target/release/trinity"
    if [ ! -f "$TRINITY_BIN" ]; then
        TRINITY_BIN="$PROJECT_ROOT/target/debug/trinity"
    fi
    if [ ! -f "$TRINITY_BIN" ]; then
        echo -e "   ${YELLOW}Binary not found — building release...${NC}"
        (cd "$PROJECT_ROOT" && cargo build --release --bin trinity -p trinity 2>&1 | tail -5)
    fi

    # Start in headless mode (prefer release)
    export TRINITY_HEADLESS=1
    if [ -f "$PROJECT_ROOT/target/release/trinity" ]; then
        (cd "$PROJECT_ROOT" && cargo run --release --bin trinity -p trinity) &
    else
        (cd "$PROJECT_ROOT" && cargo run --bin trinity -p trinity) &
    fi
    TRINITY_PID=$!

    if wait_for_port $TRINITY_PORT "Trinity" 60; then
        echo -e "   ${GREEN}Trinity is online!${NC}"
    else
        echo -e "   ${RED}Trinity failed to start. Check logs above.${NC}"
        exit 1
    fi
fi

echo ""

# ── Stage 4: Open Browser ────────────────────────────────────────────────────

echo -e "${GREEN}🌐 Opening browser...${NC}"
xdg-open "http://localhost:${TRINITY_PORT}/trinity" 2>/dev/null || \
    echo -e "   → Open ${BLUE}http://localhost:${TRINITY_PORT}/trinity${NC} in your browser"

# ── Final Status ──────────────────────────────────────────────────────────────

print_status

echo -e "${BOLD}${GREEN}🚀 Ignition complete! Trinity ID AI OS is running.${NC}"
echo -e "${GREEN}   Press Ctrl+C to stop all services.${NC}\n"

# Wait for background processes
wait
