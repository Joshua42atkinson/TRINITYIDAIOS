#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════════════
# Trinity Voice Sidecar — Setup Script
#
# Installs Chatterbox TTS + faster-whisper into trinity-voice-env
#
# Usage:
#   ./scripts/setup_chatterbox.sh
# ═══════════════════════════════════════════════════════════════════════
set -euo pipefail

VENV_DIR="$HOME/trinity-voice-env"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║     TRINITY VOICE — Chatterbox TTS Setup                   ║"
echo "╚══════════════════════════════════════════════════════════════╝"

# Create venv if needed
if [ ! -d "$VENV_DIR" ]; then
    echo "Creating Python venv at $VENV_DIR..."
    python3 -m venv "$VENV_DIR"
fi

source "$VENV_DIR/bin/activate"

echo "Installing PyTorch with ROCm 6.2 support..."
pip install --no-cache-dir torch torchaudio --index-url https://download.pytorch.org/whl/rocm6.2

echo "Installing Chatterbox TTS..."
pip install --no-cache-dir chatterbox-tts

echo "Installing voice sidecar dependencies..."
pip install --no-cache-dir faster-whisper flask requests soundfile

echo ""
echo "✅ Setup complete!"
echo ""
echo "To start the voice sidecar:"
echo "  source $VENV_DIR/bin/activate"
echo "  python scripts/voice_sidecar.py"
echo ""
echo "To use Piper fallback instead:"
echo "  TTS_BACKEND=piper python scripts/voice_sidecar.py"
