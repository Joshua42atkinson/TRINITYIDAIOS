#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# TRINITY ID AI OS — One-Command Launch Script
# Starts all services in a tmux session: llama-server + trinity + voice loop
# ═══════════════════════════════════════════════════════════════════════════════
set -e

SESSION="trinity"
GENESIS="$HOME/Workflow/desktop_trinity/trinity-genesis"
LLAMA_BIN="$GENESIS/llama.cpp/build-rocm/bin/llama-server"
LLAMA_LIB="$GENESIS/llama.cpp/build-rocm/bin"
TRINITY_BIN="$GENESIS/target/release/trinity"
VOICE_BIN="$GENESIS/target/release/trinity-voice"

# Default model — Mistral Small 4 119B MoE (68GB Q4_K_M, MLA = tiny KV cache)
MODEL="${TRINITY_MODEL:-$HOME/trinity-models/gguf/Mistral-Small-4-119B-2603-Q4_K_M-00001-of-00002.gguf}"
PORT="${TRINITY_LLM_PORT:-8080}"
CONTEXT="${TRINITY_CONTEXT:-262144}"
VOICE_SCRIPT="$GENESIS/scripts/launch/trinity_voice_server.py"
VENV="$HOME/trinity-vllm-env"

# AMD Strix Halo optimizations
export HSA_OVERRIDE_GFX_VERSION=11.0.0
export ROCBLAS_USE_HIPBLASLT=1
export LD_LIBRARY_PATH="$LLAMA_LIB:$LD_LIBRARY_PATH"

# ─────────────────────────────────────────────────────────────────────────────
# Preflight checks
# ─────────────────────────────────────────────────────────────────────────────

if [ ! -f "$LLAMA_BIN" ]; then
    echo "ERROR: llama-server not found. Build: cd $GENESIS/llama.cpp && mkdir -p build-rocm && cd build-rocm && cmake .. -DGGML_HIPBLAS=ON && make -j\$(nproc) llama-server"
    exit 1
fi

if [ ! -f "$TRINITY_BIN" ]; then
    echo "ERROR: trinity binary not found. Build: cd $GENESIS && cargo build --release -p trinity"
    exit 1
fi

if [ ! -f "$MODEL" ]; then
    echo "ERROR: Model not found at $MODEL"
    echo "Available models:"
    ls -lh ~/trinity-models/gguf/*.gguf 2>/dev/null
    exit 1
fi

# ─────────────────────────────────────────────────────────────────────────────
# Kill existing session if running
# ─────────────────────────────────────────────────────────────────────────────

if tmux has-session -t "$SESSION" 2>/dev/null; then
    echo "Killing existing trinity session..."
    tmux kill-session -t "$SESSION"
    sleep 2
fi

# Also kill any stale processes
pkill -f "llama-server.*--port $PORT" 2>/dev/null || true
pkill -f "target/release/trinity" 2>/dev/null || true
sleep 1

# ─────────────────────────────────────────────────────────────────────────────
# Create tmux session with 3 panes
# ─────────────────────────────────────────────────────────────────────────────

echo "═══════════════════════════════════════"
echo "  Starting TRINITY ID AI OS"
echo "  Model: $(basename $MODEL)"
echo "  LLM Port: $PORT"
echo "  Context: $CONTEXT"
echo "═══════════════════════════════════════"

# Create session with llama-server in first window
tmux new-session -d -s "$SESSION" -n "llm" \
    "export LD_LIBRARY_PATH=$LLAMA_LIB:\$LD_LIBRARY_PATH && export HSA_OVERRIDE_GFX_VERSION=11.0.0 && $LLAMA_BIN -m $MODEL -c $CONTEXT --port $PORT -ngl 99 --host 0.0.0.0 --threads 12 --parallel 1; read"

# Wait for LLM to load
echo "Waiting for LLM to load..."
for i in $(seq 1 300); do
    if curl -s http://127.0.0.1:$PORT/health > /dev/null 2>&1; then
        echo "LLM ready! (${i}s)"
        break
    fi
    sleep 1
    if [ $i -eq 300 ]; then
        echo "WARNING: LLM not ready after 300s (68GB model), starting Trinity anyway"
    fi
done

# Create Trinity server window
tmux new-window -t "$SESSION" -n "trinity" \
    "VLLM_URL=http://127.0.0.1:$PORT $TRINITY_BIN; read"

sleep 3

# Create voice server window (Python — wake word + ASR + TTS)
if [ -f "$VOICE_SCRIPT" ] && [ -d "$VENV" ]; then
    tmux new-window -t "$SESSION" -n "voice" \
        "source $VENV/bin/activate && CUDA_VISIBLE_DEVICES='' TRINITY_LLM_URL=http://127.0.0.1:$PORT python3 $VOICE_SCRIPT; read"
    echo "Voice server starting on :7777"
else
    echo "Voice server not available (need $VOICE_SCRIPT + $VENV)"
    echo "Creating placeholder window for voice..."
    tmux new-window -t "$SESSION" -n "voice" \
        "echo 'Voice server: Not configured for demo' && echo 'See scripts/launch/trinity_voice_server.py' && sleep 30; read"
fi

# Create a monitoring window
tmux new-window -t "$SESSION" -n "monitor" \
    "watch -n 5 'echo \"=== Services ===\"; curl -s http://127.0.0.1:3000/api/health 2>/dev/null | python3 -m json.tool 2>/dev/null || echo \"Trinity: DOWN\"; echo; curl -s http://127.0.0.1:$PORT/health 2>/dev/null || echo \"LLM: DOWN\"; echo; echo \"=== Memory ===\"; free -h | head -2; echo; echo \"=== Downloads ===\"; du -sh ~/trinity-models/safetensors/Mistral-Small-4-119B-2603/ 2>/dev/null || echo \"No Mistral download\"'"

echo ""
echo "═══════════════════════════════════════"
echo "  TRINITY is running!"
echo ""
echo "  tmux attach -t trinity    — view all panes"
echo "  Window 0: LLM (llama-server)"
echo "  Window 1: Trinity server (:3000)"
echo "  Window 2: Voice server (:7777)"
echo "  Window 3: Monitor"
echo ""
echo "  Test: curl http://localhost:3000/api/health"
echo "  Voice: Open http://localhost:7777 or say 'Hey Trinity'"
echo "  Stop:  tmux kill-session -t trinity"
echo "═══════════════════════════════════════"
