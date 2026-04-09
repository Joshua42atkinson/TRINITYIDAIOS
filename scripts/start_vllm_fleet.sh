#!/bin/bash
# ═══════════════════════════════════════════════════════════════
# Trinity ID AI OS — vLLM Fleet Launcher (DEPRECATED)
# ═══════════════════════════════════════════════════════════════
# This script is replaced by the A.R.T.Y. Hub launcher.
# Use: ./scripts/launch/launch_arty_hub.sh
# ═══════════════════════════════════════════════════════════════

echo "⚠️  start_vllm_fleet.sh is deprecated."
echo "   Use the A.R.T.Y. Hub launcher instead:"
echo ""
echo "   ./scripts/launch/launch_arty_hub.sh"
echo ""
echo "   This launches:"
echo "     • nomic-embed (port 8005) — RAG semantic embeddings"
echo "     • A.R.T.Y. Hub proxy (port 8000) — routes all vLLM requests"
echo ""

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ARTY_LAUNCHER="$SCRIPT_DIR/launch/launch_arty_hub.sh"

if [ -f "$ARTY_LAUNCHER" ]; then
    echo "Redirecting to launch_arty_hub.sh..."
    exec "$ARTY_LAUNCHER"
else
    echo "❌ launch_arty_hub.sh not found at $ARTY_LAUNCHER"
    exit 1
fi
