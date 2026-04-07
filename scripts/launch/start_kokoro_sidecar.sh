#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# TRINITY ID AI OS — Kokoro TTS Sidecar (Apache 2.0)
# Expressive AI Narrator — port 8200
# Voices: af_heart, af_bella, am_adam, am_echo, am_michael, am_fenrir
# ═══════════════════════════════════════════════════════════════════════════════

PORT="8200"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
SIDECAR="${SCRIPT_DIR}/kokoro_sidecar.py"

# Pick whichever venv has kokoro-onnx installed
for VENV in "${HOME}/trinity-vllm-env" "${HOME}/trinity-ai-env" "${HOME}/trinity-voice-env"; do
    if [ -d "$VENV" ] && "$VENV/bin/python3" -c "import kokoro_onnx" 2>/dev/null; then
        PYTHON="$VENV/bin/python3"
        echo "✅ Using venv: $VENV"
        break
    fi
done

if [ -z "$PYTHON" ]; then
    echo "ERROR: Could not find a venv with kokoro-onnx installed."
    echo "Run: pip install kokoro-onnx scipy fastapi uvicorn"
    exit 1
fi

echo "═══════════════════════════════════════════════════"
echo "  🎤 Kokoro TTS Sidecar (Apache 2.0)"
echo "  Port:    $PORT"
echo "  Models:  ~/trinity-models/tts/kokoro/"
echo "  Voices:  af_heart, af_bella, am_adam, am_echo, am_michael, am_fenrir"
echo "  Health:  http://localhost:$PORT/health"
echo "═══════════════════════════════════════════════════"

exec "$PYTHON" "$SIDECAR"
