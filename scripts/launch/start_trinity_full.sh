#!/bin/bash
# ═══════════════════════════════════════════════════════════════
# Trinity Full Stack Launcher
# ═══════════════════════════════════════════════════════════════
#
# Starts the complete Trinity system:
#   1. Mistral Small 4 119B brain on :8080 (llama-server, ROCm)
#   2. Voice pipeline + web UI on :7777 (Python, CPU)
#   3. Rust Axum server on :3000 (optional, if --with-rust)
#
# Usage:
#   ./start_trinity_full.sh            # Brain + Voice
#   ./start_trinity_full.sh --with-rust # Brain + Voice + Rust server
#   ./start_trinity_full.sh --voice-only # Voice only (brain already running)
#
# ═══════════════════════════════════════════════════════════════

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE="$(cd "$SCRIPT_DIR/../.." && pwd)"
VENV_DIR="$HOME/trinity-vllm-env"
LLAMA_SERVER="$WORKSPACE/llama.cpp/build-rocm/bin/llama-server"
MISTRAL_GGUF="$HOME/trinity-models/gguf/Mistral-Small-4-119B-2603-Q4_K_M-00001-of-00002.gguf"

BRAIN_PORT="${TRINITY_BRAIN_PORT:-8080}"
VOICE_PORT="${TRINITY_VOICE_PORT:-7777}"
RUST_PORT="${TRINITY_RUST_PORT:-3000}"

# ─── Parse args ───────────────────────────────────────────────
VOICE_ONLY=false
WITH_RUST=false
for arg in "$@"; do
    case $arg in
        --voice-only) VOICE_ONLY=true ;;
        --with-rust)  WITH_RUST=true ;;
    esac
done

# ─── Cleanup function ────────────────────────────────────────
cleanup() {
    echo ""
    echo "Shutting down Trinity..."
    kill $BRAIN_PID 2>/dev/null || true
    kill $VOICE_PID 2>/dev/null || true
    kill $RUST_PID 2>/dev/null || true
    wait 2>/dev/null
    echo "All processes stopped."
}
trap cleanup EXIT INT TERM

BRAIN_PID=""
VOICE_PID=""
RUST_PID=""

echo "╔══════════════════════════════════════════════════════════╗"
echo "║           TRINITY ID AI OS — Full Stack                 ║"
echo "╚══════════════════════════════════════════════════════════╝"

# ─── Step 1: Brain (Mistral Small 4 119B) ────────────────────
if [ "$VOICE_ONLY" = false ]; then
    # Kill any existing brain
    pkill -f "llama-server.*8080" 2>/dev/null || true
    sleep 1

    if [ ! -f "$LLAMA_SERVER" ]; then
        echo "ERROR: llama-server not found at $LLAMA_SERVER"
        echo "Build with: cd $WORKSPACE/llama.cpp && cmake -B build-rocm -DGGML_HIP=ON && cmake --build build-rocm -j"
        exit 1
    fi

    if [ ! -f "$MISTRAL_GGUF" ]; then
        echo "ERROR: Mistral Small 4 GGUF not found at $MISTRAL_GGUF"
        exit 1
    fi

    echo ""
    echo "🧠 Starting Mistral Small 4 119B on :$BRAIN_PORT..."
    export LD_LIBRARY_PATH="$WORKSPACE/llama.cpp/build-rocm/bin:$LD_LIBRARY_PATH"
    "$LLAMA_SERVER" \
        -m "$MISTRAL_GGUF" \
        --host 0.0.0.0 --port "$BRAIN_PORT" \
        -ngl 99 -c 262144 --jinja \
        > /tmp/trinity_brain.log 2>&1 &
    BRAIN_PID=$!

    # Wait for brain to be ready
    echo "   Waiting for brain to load (this takes ~2 minutes)..."
    for i in $(seq 1 180); do
        if curl -sf "http://localhost:$BRAIN_PORT/health" > /dev/null 2>&1; then
            echo "   ✅ Brain ready on :$BRAIN_PORT (${i}s)"
            break
        fi
        if ! kill -0 $BRAIN_PID 2>/dev/null; then
            echo "   ❌ Brain process died. Check /tmp/trinity_brain.log"
            exit 1
        fi
        sleep 1
    done
else
    echo ""
    echo "🧠 Brain: using existing server on :$BRAIN_PORT"
    if ! curl -sf "http://localhost:$BRAIN_PORT/health" > /dev/null 2>&1; then
        echo "   ⚠️  Brain not responding on :$BRAIN_PORT — voice will work but LLM calls will fail"
    else
        echo "   ✅ Brain is live"
    fi
fi

# ─── Step 2: Voice Pipeline ──────────────────────────────────
pkill -f trinity_voice_server.py 2>/dev/null || true
sleep 1

echo ""
echo "🎙️ Starting Voice Pipeline on :$VOICE_PORT..."
source "$VENV_DIR/bin/activate"
CUDA_VISIBLE_DEVICES="" \
TRINITY_LLM_URL="http://localhost:$BRAIN_PORT" \
TRINITY_VOICE_PORT="$VOICE_PORT" \
python3 "$SCRIPT_DIR/trinity_voice_server.py" \
    > /tmp/trinity_voice.log 2>&1 &
VOICE_PID=$!

sleep 3
if kill -0 $VOICE_PID 2>/dev/null; then
    echo "   ✅ Voice pipeline on :$VOICE_PORT"
    echo "   🌐 Web UI: http://localhost:$VOICE_PORT"
else
    echo "   ❌ Voice pipeline failed. Check /tmp/trinity_voice.log"
fi

# ─── Step 3: Rust Server (optional) ──────────────────────────
if [ "$WITH_RUST" = true ]; then
    echo ""
    echo "🦀 Starting Rust server on :$RUST_PORT..."
    VLLM_URL="http://localhost:$BRAIN_PORT" \
    cargo run -p trinity --release \
        > /tmp/trinity_rust.log 2>&1 &
    RUST_PID=$!
    sleep 5
    if kill -0 $RUST_PID 2>/dev/null; then
        echo "   ✅ Rust server on :$RUST_PORT"
    else
        echo "   ❌ Rust server failed. Check /tmp/trinity_rust.log"
    fi
fi

# ─── Summary ─────────────────────────────────────────────────
echo ""
echo "═══════════════════════════════════════════════════════════"
echo "  Trinity is LIVE"
echo "  🧠 Brain:  http://localhost:$BRAIN_PORT  (Mistral Small 4 119B)"
echo "  🎙️ Voice:  http://localhost:$VOICE_PORT  (wake word + TTS)"
if [ "$WITH_RUST" = true ]; then
echo "  🦀 Server: http://localhost:$RUST_PORT  (Axum + quests + tools)"
fi
echo ""
echo "  Say 'Hey Trinity' for dev mode"
echo "  Say 'Hey Pete' for Iron Road mode"
echo "  Press Ctrl+C to stop everything"
echo "═══════════════════════════════════════════════════════════"

# Wait for all processes
wait
