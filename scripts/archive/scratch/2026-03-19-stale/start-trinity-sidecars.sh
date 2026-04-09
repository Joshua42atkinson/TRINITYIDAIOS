#!/bin/bash
# ════════════════════════════════════════════════════════════════════
# TRINITY SIDECAR STARTUP - Three Sidecars, Three UIs
# ════════════════════════════════════════════════════════════════════
# Usage: ./start-trinity-sidecars.sh [ironroad|dev|art|all]
#
# IRON ROAD (Conductor) - Single large inference - :8080
# DEV (Engineer)        - Single large inference - :8081/:8083
# ART (Artist)          - Multi-model management  - :8188/:8086/:8189
# ════════════════════════════════════════════════════════════════════

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
LLAMA_BIN="$PROJECT_DIR/llama.cpp/build-vulkan/bin/longcat-sglang"

MODE="${1:-all}"

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║           TRINITY SIDECAR SYSTEM                              ║"
echo "║           Mode: $MODE                                        ║"
echo "╚══════════════════════════════════════════════════════════════╝"

# Kill existing processes
pkill -f "longcat-sglang" 2>/dev/null || true
sleep 2

start_ironroad() {
    echo ""
    echo "🚂 IRON ROAD (Conductor) - Starting..."
    
    MODEL="$HOME/trinity-models/gguf/gpt-oss-20b-UD-Q4_K_XL.gguf"
    
    if [ ! -f "$MODEL" ]; then
        echo "❌ Model not found: $MODEL"
        return 1
    fi
    
    nohup "$LLAMA_BIN" \
        -m "$MODEL" \
        -c 8192 \
        --port 8080 \
        -ngl 99 \
        --threads 8 \
        --temp 0.7 \
        > /tmp/ironroad.log 2>&1 &
    
    echo "  PID: $! | Port: 8080 | Model: GPT-OSS-20B (12GB)"
    echo "  Log: /tmp/ironroad.log"
}

start_dev() {
    echo ""
    echo "⚡ DEV (Engineer) - Starting..."
    
    # Option 1: Crow-9B (lighter, faster)
    # MODEL="$HOME/trinity-models/gguf/OmniCoder-9B-Q4_K_M.gguf"
    # PORT=8081
    
    # Option 2: REAP-25B (heavier, better code)
    MODEL="$HOME/trinity-models/gguf/Qwen3-Coder-REAP-25B-A3B-Rust-Q4_K_M.gguf"
    PORT=8083
    
    if [ ! -f "$MODEL" ]; then
        echo "❌ Model not found: $MODEL"
        return 1
    fi
    
    nohup "$LLAMA_BIN" \
        -m "$MODEL" \
        -c 8192 \
        --port $PORT \
        -ngl 99 \
        --threads 8 \
        --temp 0.3 \
        > /tmp/dev.log 2>&1 &
    
    echo "  PID: $! | Port: $PORT | Model: REAP-25B (15GB)"
    echo "  Log: /tmp/dev.log"
}

start_art() {
    echo ""
    echo "🎨 ART (Artist) - Multi-model sidecar..."
    echo "  Note: ART uses external runtimes (ComfyUI, ACE-Step, Trellis)"
    echo "  These must be started separately:"
    echo "    - ComfyUI:  python ~/ComfyUI/main.py --port 8188"
    echo "    - ACE-Step: ~/src/acestep.cpp/build/bin/acestep-server --port 8086"
    echo "    - Trellis:  (coming soon)"
}

wait_for_port() {
    local port=$1
    local name=$2
    echo "  Waiting for $name on :$port..."
    for i in {1..30}; do
        if curl -s "http://localhost:$port/v1/models" > /dev/null 2>&1; then
            echo "  ✅ $name ready on :$port"
            return 0
        fi
        sleep 1
    done
    echo "  ⏳ $name still loading... check logs"
    return 1
}

case "$MODE" in
    ironroad|conductor)
        start_ironroad
        wait_for_port 8080 "IRON ROAD"
        ;;
    dev|engineer)
        start_dev
        wait_for_port 8083 "DEV"
        ;;
    art|artist)
        start_art
        ;;
    all)
        start_ironroad
        start_dev
        start_art
        sleep 5
        wait_for_port 8080 "IRON ROAD" &
        wait_for_port 8083 "DEV" &
        wait
        ;;
    *)
        echo "Usage: $0 [ironroad|dev|art|all]"
        exit 1
        ;;
esac

echo ""
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  TRINITY SIDECAR STATUS                                       ║"
echo "╠══════════════════════════════════════════════════════════════╣"
echo "║  IRON ROAD:  http://localhost:8080  → /book.html             ║"
echo "║  DEV:        http://localhost:8083  → /dev.html              ║"
echo "║  ART:        Multi-model (see above)                         ║"
echo "╚══════════════════════════════════════════════════════════════╝"
