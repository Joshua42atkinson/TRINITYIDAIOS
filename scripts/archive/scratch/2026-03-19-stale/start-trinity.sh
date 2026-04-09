#!/bin/bash
# Trinity ID AI OS - Startup Script
# Starts llama.cpp + Trinity headless server

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║           TRINITY ID AI OS - Starting...                     ║"
echo "╚══════════════════════════════════════════════════════════════╝"

# Kill any existing processes
echo "[1/4] Stopping existing processes..."
pkill -f "longcat-sglang" 2>/dev/null || true
pkill -f "trinity" 2>/dev/null || true
sleep 1

# Check for GGUF models
MODEL_PATH="$HOME/trinity-models/gguf/gpt-oss-20b-UD-Q4_K_XL.gguf"
if [ ! -f "$MODEL_PATH" ]; then
    echo "ERROR: Model not found at $MODEL_PATH"
    echo "Available models:"
    ls -la ~/trinity-models/gguf/ 2>/dev/null || echo "  (no models found)"
    exit 1
fi

# Start llama.cpp
echo "[2/4] Starting llama.cpp on :8080..."
LLAMA_BIN="$HOME/Workflow/desktop_trinity/trinity-genesis/llama.cpp/build-vulkan/bin/longcat-sglang"
if [ ! -f "$LLAMA_BIN" ]; then
    echo "ERROR: longcat-sglang not found at $LLAMA_BIN"
    exit 1
fi

nohup "$LLAMA_BIN" \
    -m "$MODEL_PATH" \
    -c 4096 \
    --port 8080 \
    -ngl 99 \
    > /tmp/longcat-sglang.log 2>&1 &

LLAMA_PID=$!
echo "  longcat-sglang PID: $LLAMA_PID"

# Wait for llama.cpp to load model
echo "[3/4] Loading model (this may take a minute)..."
for i in {1..30}; do
    if curl -s http://localhost:8080/v1/models > /dev/null 2>&1; then
        echo "  ✓ Model loaded!"
        break
    fi
    if [ $i -eq 30 ]; then
        echo "ERROR: Model failed to load after 30 seconds"
        echo "Check /tmp/longcat-sglang.log"
        kill $LLAMA_PID 2>/dev/null || true
        exit 1
    fi
    sleep 1
done

# Start Trinity server
echo "[4/4] Starting Trinity server on :3000..."
cd "$PROJECT_DIR"
nohup cargo run -p trinity --release > /tmp/trinity-server.log 2>&1 &

TRINITY_PID=$!
echo "  Trinity PID: $TRINITY_PID"

# Wait for Trinity to start
sleep 3

echo ""
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║                    TRINITY IS READY!                         ║"
echo "╠══════════════════════════════════════════════════════════════╣"
echo "║  Web Interfaces:                                              ║"
echo "║    • http://localhost:3000/          - Main                  ║"
echo "║    • http://localhost:3000/ask-pete.html - Ask Pete (AI)     ║"
echo "║    • http://localhost:3000/book.html    - Iron Road Book    ║"
echo "║    • http://localhost:3000/dev.html     - Dev Console       ║"
echo "║                                                              ║"
echo "║  API Endpoints:                                               ║"
echo "║    • GET  /api/health    - Health check                      ║"
echo "║    • POST /api/chat      - Chat with AI                      ║"
echo "║    • GET  /api/status    - System status                     ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""
echo "Logs:"
echo "  llama.cpp: tail -f /tmp/longcat-sglang.log"
echo "  Trinity:   tail -f /tmp/trinity-server.log"
echo ""
