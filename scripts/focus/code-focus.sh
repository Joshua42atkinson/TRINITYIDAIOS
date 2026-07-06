#!/bin/bash
# Code Focus Mode — IDEs running, Trinity models killed
# Use when coding with Windsurf, Antigravity, or other IDE agents
# Kills: DiffusionGemma (podman), ComfyUI, Trinity server
# Keeps: IDE agents (Windsurf, Antigravity, etc.)

set -e

echo "💻 CODE FOCUS MODE"
echo "==================="
echo "Killing Trinity models to free VRAM for IDE agents..."
echo ""

# Kill ComfyUI (A)
pkill -f "ComfyUI/venv/bin/python main.py" 2>/dev/null && echo "  ✅ Killed ComfyUI (A)" || echo "  ⬜ ComfyUI not running"

# Kill DiffusionGemma (P) via podman
podman stop -t 5 $(podman ps -q --filter ancestor=vllm/vllm-openai:latest 2>/dev/null) 2>/dev/null && echo "  ✅ Stopped DiffusionGemma container (P)" || echo "  ⬜ DiffusionGemma container not running"

# Also try killing any vllm processes directly
pkill -f "vllm" 2>/dev/null && echo "  ✅ Killed vLLM process" || echo "  ⬜ No vLLM process running"

# Kill Trinity server (optional — keep if you want API access)
if [ "$1" != "--keep-trinity" ]; then
    pkill -f "target/release/trinity" 2>/dev/null && echo "  ✅ Killed Trinity server" || echo "  ⬜ Trinity server not running"
else
    echo "  ⬜ Trinity server kept running (--keep-trinity flag)"
fi

echo ""
sleep 2

# Verify IDEs are running
if pgrep -f "windsurf" >/dev/null 2>&1; then
    echo "  ✅ Windsurf is running"
else
    echo "  ⚠️  Windsurf not detected — start it manually"
fi

if pgrep -f "antigravity" >/dev/null 2>&1; then
    echo "  ✅ Antigravity is running"
else
    echo "  ⬜ Antigravity not detected"
fi

echo ""
echo "💻 Code Focus active."
echo "   IDE agents have full VRAM + CPU."
echo "   Trinity models are OFF."
echo ""
echo "   To switch back: scripts/focus/creative-focus.sh"
echo "   To kill everything: scripts/focus/night-shift.sh"
