#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# TRINITY ID AI OS — Demo Quick Start Script
# Simplified launch for recording sessions - essential services only
# ═══════════════════════════════════════════════════════════════════════════════
set -e

SESSION="trinity-demo"
GENESIS="$HOME/Workflow/desktop_trinity/trinity-genesis"
LLAMA_BIN="$GENESIS/llama.cpp/build-rocm/bin/longcat-sglang"
LLAMA_LIB="$GENESIS/llama.cpp/build-rocm/bin"
TRINITY_BIN="$GENESIS/target/release/trinity"

# Demo model - ensure it's available
MODEL="${TRINITY_MODEL:-$HOME/trinity-models/gguf/Mistral-Small-4-119B-2603-Q4_K_M-00001-of-00002.gguf}"
PORT="${TRINITY_LLM_PORT:-8080}"
CONTEXT="${TRINITY_CONTEXT:-262144}"  # Full 256K context — Mistral Small 4 MLA

# AMD Strix Halo optimizations
export HSA_OVERRIDE_GFX_VERSION=11.0.0
export ROCBLAS_USE_HIPBLASLT=1
export LD_LIBRARY_PATH="$LLAMA_LIB:$LD_LIBRARY_PATH"

echo "═══════════════════════════════════════"
echo "  TRINITY DEMO QUICK START"
echo "  Model: $(basename $MODEL)"
echo "  Port: $PORT"
echo "  Context: $CONTEXT (reduced for demo)"
echo "═══════════════════════════════════════"

# Preflight checks
if [ ! -f "$LLAMA_BIN" ]; then
    echo "ERROR: longcat-sglang not found"
    echo "Run: cd $GENESIS/llama.cpp && mkdir -p build-rocm && cd build-rocm && cmake .. -DGGML_HIPBLAS=ON && make -j\$(nproc) longcat-sglang"
    exit 1
fi

if [ ! -f "$TRINITY_BIN" ]; then
    echo "ERROR: trinity binary not found"
    echo "Run: cd $GENESIS && cargo build --release -p trinity"
    exit 1
fi

if [ ! -f "$MODEL" ]; then
    echo "ERROR: Model not found at $MODEL"
    echo "Available models:"
    ls -lh ~/trinity-models/gguf/*.gguf 2>/dev/null || echo "No models found"
    exit 1
fi

# Kill existing sessions
if tmux has-session -t "$SESSION" 2>/dev/null; then
    echo "Killing existing demo session..."
    tmux kill-session -t "$SESSION"
    sleep 2
fi

pkill -f "longcat-sglang.*--port $PORT" 2>/dev/null || true
pkill -f "target/release/trinity" 2>/dev/null || true
sleep 1

# Create tmux session
echo "Starting LLM server..."
tmux new-session -d -s "$SESSION" -n "llm" \
    "export LD_LIBRARY_PATH=$LLAMA_LIB:\$LD_LIBRARY_PATH && export HSA_OVERRIDE_GFX_VERSION=11.0.0 && $LLAMA_BIN -m $MODEL -c $CONTEXT --port $PORT -ngl 99 --host 0.0.0.0 --threads 8 --parallel 1 --jinja; read"

# Wait for LLM with timeout
echo "Waiting for LLM to load (max 120s for demo)..."
for i in $(seq 1 120); do
    if curl -s http://127.0.0.1:$PORT/health > /dev/null 2>&1; then
        echo "✓ LLM ready! (${i}s)"
        break
    fi
    sleep 1
    if [ $i -eq 120 ]; then
        echo "⚠ LLM not ready after 120s - check model path"
        echo "Model: $MODEL"
        exit 1
    fi
done

# Start Trinity
echo "Starting Trinity server..."
tmux new-window -t "$SESSION" -n "trinity" \
    "VLLM_URL=http://127.0.0.1:$PORT $TRINITY_BIN; read"

sleep 3

# Verify Trinity is responding
echo "Checking Trinity health..."
for i in $(seq 1 30); do
    if curl -s http://127.0.0.1:3000/api/health > /dev/null 2>&1; then
        echo "✓ Trinity ready! (${i}s)"
        break
    fi
    sleep 1
    if [ $i -eq 30 ]; then
        echo "⚠ Trinity not responding - check logs"
        tmux capture-pane -t "$SESSION:trinity" -p | tail -10
        exit 1
    fi
done

# Create demo monitoring window
tmux new-window -t "$SESSION" -n "demo-status" \
    "watch -n 10 'echo \"=== TRINITY DEMO STATUS ===\"; date; echo; echo \"Services:\"; curl -s http://127.0.0.1:3000/api/health 2>/dev/null | python3 -c \"import sys,json; print(json.dumps(json.load(sys.stdin), indent=2))\" 2>/dev/null || echo \"Trinity: DOWN\"; echo; echo \"LLM:\"; curl -s http://127.0.0.1:$PORT/health 2>/dev/null || echo \"LLM: DOWN\"; echo; echo \"Memory:\"; free -h | head -2'"

echo ""
echo "═══════════════════════════════════════"
echo "  ✓ TRINITY DEMO READY!"
echo ""
echo "  Browser: http://localhost:3000"
echo "  Monitor: tmux attach -t trinity-demo -t demo-status"
echo "  Logs: tmux attach -t trinity-demo"
echo ""
echo "  Window 0: LLM Server (port $PORT)"
echo "  Window 1: Trinity (port 3000)"
echo "  Window 2: Demo Status Monitor"
echo ""
echo "  Stop demo: tmux kill-session -t trinity-demo"
echo "═══════════════════════════════════════"

# Open browser automatically if possible
if command -v xdg-open > /dev/null; then
    echo "Opening browser..."
    xdg-open http://localhost:3000
elif command -v open > /dev/null; then
    echo "Opening browser..."
    open http://localhost:3000
fi
