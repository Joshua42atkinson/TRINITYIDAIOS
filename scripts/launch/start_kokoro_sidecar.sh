#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# TRINITY ID AI OS — Kokoro TTS Sidecar (Replaces Piper/Moshi)
# Expressive Narrator for Zen Mode
# ═══════════════════════════════════════════════════════════════════════════════
set -e

VENV="${HOME}/trinity-vllm-env"
PORT="8200"

if [ ! -d "$VENV" ]; then
    echo "ERROR: Python venv not found at $VENV"
    echo "Please create it and 'pip install kokoro soundfile numpy fastapi uvicorn'"
    exit 1
fi

source "$VENV/bin/activate"

echo "═══════════════════════════════════════"
echo "  Kokoro Voice Sidecar (Narrator)"
echo "  Port: $PORT"
echo "  Engine: Kokoro (Chatterbox Architecture)"
echo "  Status: http://localhost:$PORT/health"
echo "═══════════════════════════════════════"

exec python "$HOME/Workflow/desktop_trinity/trinity-genesis/scripts/launch/kokoro_sidecar.py"
