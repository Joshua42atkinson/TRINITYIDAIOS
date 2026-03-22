#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════════════
# Trinity Engineer Sidecar — Sword & Shield Launcher
#
# Starts the dual-model Engineer agent:
#   Shield (Opus 27B): Thinker, planner, reviewer  — port 8081
#   Sword  (REAP 25B): Fast Rust code generator    — port 8082
#   API server:                                     — port 8090
#
# Usage:
#   ./scripts/launch/start_engineer.sh              # Normal start
#   ./scripts/launch/start_engineer.sh --autonomous # Start + auto work loop
# ═══════════════════════════════════════════════════════════════════════
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$PROJECT_ROOT"

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║     TRINITY ENGINEER SIDECAR — Sword & Shield Agent        ║"
echo "╚══════════════════════════════════════════════════════════════╝"

# Kill any stale engineer processes
echo "Cleaning up stale processes..."
pkill -f "llama-server.*8081" 2>/dev/null || true
pkill -f "llama-server.*8082" 2>/dev/null || true
pkill -f "trinity-sidecar-engineer" 2>/dev/null || true
sleep 1

# Verify models exist
OPUS_MODEL="models/engineer/Qwen3.5-27B-Claude-4.6-Opus-Reasoning-Distilled.i1-Q6_K.gguf"
REAP_MODEL="models/engineer/Qwen3-Coder-REAP-25B-A3B-Rust-Q4_K_M.gguf"

if [ ! -f "$OPUS_MODEL" ]; then
    echo "ERROR: Opus model not found: $OPUS_MODEL"
    exit 1
fi
if [ ! -f "$REAP_MODEL" ]; then
    echo "ERROR: REAP model not found: $REAP_MODEL"
    exit 1
fi

echo "Opus model: $(du -h "$OPUS_MODEL" | cut -f1)"
echo "REAP model: $(du -h "$REAP_MODEL" | cut -f1)"

# Ensure quest directories exist
mkdir -p quests/board quests/active quests/complete quests/failed

# Build the sidecar
echo ""
echo "Building Engineer sidecar..."
cargo build --release -p trinity-sidecar-engineer 2>&1 | tail -5

# Run the sidecar
echo ""
echo "Starting Engineer sidecar..."
echo "  Shield (Opus): http://127.0.0.1:8081"
echo "  Sword  (REAP): http://127.0.0.1:8082"
echo "  API:           http://127.0.0.1:8090"
echo ""

# Run in foreground (logs to terminal)
cargo run --release -p trinity-sidecar-engineer &
SIDECAR_PID=$!

echo "Sidecar PID: $SIDECAR_PID"

# Wait for API to be ready
echo "Waiting for API to be ready..."
for i in $(seq 1 180); do
    if curl -s http://127.0.0.1:8090/status > /dev/null 2>&1; then
        echo "Engineer sidecar is READY!"
        break
    fi
    sleep 1
done

# Start autonomous mode if requested
if [ "${1:-}" = "--autonomous" ]; then
    echo ""
    echo "Starting autonomous work loop..."
    curl -s -X POST http://127.0.0.1:8090/autonomous/start
    echo ""
    echo "Autonomous mode ACTIVE — Engineer will pick up quests from quests/board/"
fi

echo ""
echo "Engineer sidecar is running. Endpoints:"
echo "  GET  http://127.0.0.1:8090/status"
echo "  GET  http://127.0.0.1:8090/quests"
echo "  POST http://127.0.0.1:8090/autonomous/start"
echo "  POST http://127.0.0.1:8090/autonomous/stop"
echo "  POST http://127.0.0.1:8090/think   {\"prompt\": \"...\"}"
echo "  POST http://127.0.0.1:8090/code    {\"prompt\": \"...\"}"
echo ""
echo "Drop quest JSON files in quests/board/ for the Engineer to pick up."
echo "Press Ctrl+C to stop."

wait $SIDECAR_PID
