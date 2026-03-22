#!/bin/bash
# Trinity Voice Server — Launch Script
# Usage: ./start_voice_pipeline.sh [voice]
#
# Web UI on http://localhost:7777 — swap voices, send text, start/stop pipeline
# Voices: am_adam, am_echo, am_eric, am_fenrir, am_liam, am_michael, am_onyx, am_puck
# Default: am_adam

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
VENV_DIR="$HOME/trinity-vllm-env"

source "$VENV_DIR/bin/activate"

export CUDA_VISIBLE_DEVICES=""
export TRINITY_VOICE="${1:-am_adam}"
export TRINITY_LLM_URL="${TRINITY_LLM_URL:-http://localhost:8080}"
export TRINITY_WAKE_WORD="${TRINITY_WAKE_WORD:-hey_jarvis}"
export TRINITY_WHISPER_MODEL="${TRINITY_WHISPER_MODEL:-base.en}"
export TRINITY_VOICE_PORT="${TRINITY_VOICE_PORT:-7777}"

echo "═══════════════════════════════════════════"
echo "  Trinity Voice Server"
echo "  Web UI: http://localhost:$TRINITY_VOICE_PORT"
echo "  Voice:  $TRINITY_VOICE"
echo "  Brain:  $TRINITY_LLM_URL"
echo "  Wake:   $TRINITY_WAKE_WORD"
echo "═══════════════════════════════════════════"

python3 "$SCRIPT_DIR/trinity_voice_server.py"
