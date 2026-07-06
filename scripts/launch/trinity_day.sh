#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# TRINITY ID AI OS — Day Mode Launcher
# ═══════════════════════════════════════════════════════════════════════════════
#
# Launches both models + Trinity server for the dual-model setup:
#   P — DiffusionGemma 26B (vLLM, :8000, ~42GB VRAM) — Builder
#   R — Qwythos-9B (llama.cpp, :8002, ~13GB VRAM) — Socratic Guide
#
# Total VRAM: ~55GB of 120GB. Both run simultaneously.
#
# USAGE:
#   ./scripts/launch/trinity_day.sh          # foreground Trinity
#   ./scripts/launch/trinity_day.sh --bg     # everything background
#
# ═══════════════════════════════════════════════════════════════════════════════

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
BG_MODE="${1:-}"

echo "═══════════════════════════════════════════════"
echo "  TRINITY DAY MODE — Dual Model Launch"
echo "  P: DiffusionGemma 26B  (:8000)  Builder"
echo "  R: Qwythos-9B          (:8002)  Socratic"
echo "═══════════════════════════════════════════════"
echo ""

# ─── 1. Check / Start DiffusionGemma 26B (vLLM in podman) ───────────────────
if curl -s --connect-timeout 2 http://127.0.0.1:8000/v1/models > /dev/null 2>&1; then
  echo "✅ P (DiffusionGemma 26B) already running on :8000"
else
  echo "🚀 Starting P (DiffusionGemma 26B) on :8000..."
  if [ -f "$HOME/trinity-models/start-diffusiongemma.sh" ]; then
    bash "$HOME/trinity-models/start-diffusiongemma.sh"
    echo -n "   Waiting for health"
    for i in $(seq 1 60); do
      if curl -s --connect-timeout 2 http://127.0.0.1:8000/v1/models > /dev/null 2>&1; then
        echo ""
        echo "   ✅ P ONLINE"
        break
      fi
      echo -n "."
      sleep 3
    done
  else
    echo "❌ start-diffusiongemma.sh not found at $HOME/trinity-models/"
    exit 1
  fi
fi
echo ""

# ─── 2. Check / Start Qwythos-9B (llama.cpp) ────────────────────────────────
if curl -s --connect-timeout 2 http://127.0.0.1:8002/v1/models > /dev/null 2>&1; then
  echo "✅ R (Qwythos-9B) already running on :8002"
else
  echo "🚀 Starting R (Qwythos-9B) on :8002..."
  LLAMA_BIN="/home/joshua/llama.cpp/build/bin/llama-server"
  QWYTHOS_MODEL="$HOME/trinity-models/Qwythos-9B-Claude-Mythos-5-1M-Q4_K_M.gguf"
  
  if [ ! -f "$LLAMA_BIN" ]; then
    echo "❌ llama-server not found at $LLAMA_BIN"
    exit 1
  fi
  if [ ! -f "$QWYTHOS_MODEL" ]; then
    echo "❌ Qwythos model not found at $QWYTHOS_MODEL"
    exit 1
  fi
  
  nohup "$LLAMA_BIN" \
    --model "$QWYTHOS_MODEL" \
    --port 8002 \
    --host 0.0.0.0 \
    --n-gpu-layers 99 \
    --ctx-size 32768 \
    --no-mmap \
    --cache-type-k q8_0 \
    --cache-type-v q8_0 \
    --threads 4 \
    --temp 0.6 \
    --top-p 0.95 \
    --top-k 20 \
    --repeat-penalty 1.05 \
    > /tmp/qwythos-llama.log 2>&1 &
  
  echo "   PID: $!"
  echo -n "   Waiting for health"
  for i in $(seq 1 30); do
    if curl -s --connect-timeout 2 http://127.0.0.1:8002/v1/models > /dev/null 2>&1; then
      echo ""
      echo "   ✅ R ONLINE"
      break
    fi
    echo -n "."
    sleep 2
  done
fi
echo ""

# ─── 3. Check / Start Trinity Server ────────────────────────────────────────
if curl -s --connect-timeout 2 http://127.0.0.1:3000/api/health > /dev/null 2>&1; then
  echo "✅ Trinity server already running on :3000"
else
  echo "🚀 Starting Trinity server on :3000..."
  cd "$PROJECT_ROOT"
  
  if [ "$BG_MODE" = "--bg" ]; then
    nohup target/debug/trinity --headless > /tmp/trinity.log 2>&1 &
    echo "   PID: $!"
    echo -n "   Waiting for health"
    for i in $(seq 1 20); do
      if curl -s --connect-timeout 2 http://127.0.0.1:3000/api/health > /dev/null 2>&1; then
        echo ""
        echo "   ✅ Trinity ONLINE"
        break
      fi
      echo -n "."
      sleep 2
    done
  else
    echo "   Starting in foreground (Ctrl+C to stop)..."
    exec target/debug/trinity --headless
  fi
fi
echo ""

# ─── 4. Sync PWA ────────────────────────────────────────────────────────────
if [ -f "$PROJECT_ROOT/crates/trinity/static/phone.html" ]; then
  cp "$PROJECT_ROOT/crates/trinity/static/phone.html" \
     "$PROJECT_ROOT/crates/trinity/frontend/dist/phone.html" 2>/dev/null || true
  echo "📱 PWA synced to frontend/dist/"
fi
echo ""

echo "═══════════════════════════════════════════════"
echo "  ✅ TRINITY DAY MODE READY"
echo ""
echo "  Phone PWA:  http://100.83.222.35:3000/trinity/phone.html"
echo "  Research:   Qwythos-9B Socratic (🔍 mode)"
echo "  Build:      DiffusionGemma 26B (⚙️ mode)"
echo "  Voice:      🎤 to talk, 🔊 to hear responses"
echo "═══════════════════════════════════════════════"
