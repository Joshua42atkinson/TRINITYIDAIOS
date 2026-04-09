#!/bin/bash
# ══════════════════════════════════════════════════════
# Trinity OS — Robust Startup Script
# SEQUENTIAL LOADING: Ensures longcat-sglang is HEALTHY
# (model fully loaded into VRAM) before starting Trinity.
#
# This prevents bus contention — the 119B model (68GB)
# must finish loading before any other heavy process runs.
# ══════════════════════════════════════════════════════
set -euo pipefail

LLAMA_PORT=8080
TRINITY_BIN="/home/joshua/Workflow/desktop_trinity/trinity-genesis/target/release/trinity"
MAX_WAIT=120  # seconds to wait for longcat-sglang

echo "[Trinity Startup] Sequential load — waiting for Mistral 119B on port $LLAMA_PORT..."
echo "[Trinity Startup] Nothing else will start until the model is pinned in VRAM."

STARTED=false
for i in $(seq 1 $MAX_WAIT); do
    # Check both health endpoint AND that slots are available (model fully loaded)
    HEALTH=$(curl -sf "http://127.0.0.1:${LLAMA_PORT}/health" 2>/dev/null || echo "")
    if echo "$HEALTH" | grep -q '"status":"ok"' 2>/dev/null; then
        echo "[Trinity Startup] ✓ Mistral 119B healthy and pinned after ${i}s"
        STARTED=true
        break
    fi
    # Show progress every 10 seconds
    if [ $((i % 10)) -eq 0 ]; then
        echo "[Trinity Startup] Still waiting... ${i}/${MAX_WAIT}s"
    fi
    sleep 1
done

if [ "$STARTED" = false ]; then
    echo "[Trinity Startup] ✗ ERROR: longcat-sglang not healthy after ${MAX_WAIT}s"
    echo "[Trinity Startup] Check: journalctl -u trinity-llama -n 50"
    exit 1
fi

# Small grace period — let VRAM settle after model load
sleep 2

echo "[Trinity Startup] Launching Trinity OS backend..."
exec "$TRINITY_BIN"
