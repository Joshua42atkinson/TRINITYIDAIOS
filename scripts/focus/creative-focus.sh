#!/bin/bash
# Creative Focus Mode — Trinity P + A running, IDEs killed
# Use when making art, stories, voice, video, 3D
# Kills: Windsurf, Antigravity, VS Code, Zed, Void
# Keeps: Trinity (:3000), DiffusionGemma (:8000), ComfyUI (:8188)

set -e

echo "🎨 CREATIVE FOCUS MODE"
echo "========================"
echo "Killing IDE agents to free VRAM for Trinity..."
echo ""

# Kill Windsurf
pkill -f "windsurf" 2>/dev/null && echo "  ✅ Killed Windsurf" || echo "  ⬜ Windsurf not running"

# Kill Antigravity
pkill -f "antigravity" 2>/dev/null && echo "  ✅ Killed Antigravity" || echo "  ⬜ Antigravity not running"

# Kill VS Code
pkill -f "code-server\|vscode" 2>/dev/null && echo "  ✅ Killed VS Code" || echo "  ⬜ VS Code not running"

# Kill Zed
pkill -f "zed" 2>/dev/null && echo "  ✅ Killed Zed" || echo "  ⬜ Zed not running"

# Kill Void
pkill -f "void" 2>/dev/null && echo "  ✅ Killed Void" || echo "  ⬜ Void not running"

# Kill any lingering language servers from IDEs
pkill -f "language_server_linux" 2>/dev/null && echo "  ✅ Killed language servers" || echo "  ⬜ No language servers running"

echo ""
sleep 2

# Check if Trinity is running
if curl -s http://localhost:3000/api/health >/dev/null 2>&1; then
    echo "  ✅ Trinity :3000 is running"
else
    echo "  ⚠️  Trinity :3000 not detected — start it:"
    echo "     ~/Workflow/TRINITYIDAIOS/target/release/trinity --headless &"
fi

# Check if P (DiffusionGemma) is running
if curl -s http://localhost:8000/v1/models >/dev/null 2>&1; then
    echo "  ✅ P (DiffusionGemma) :8000 is running"
else
    echo "  ⚠️  P not detected — start it:"
    echo "     bash ~/trinity-models/start-diffusiongemma.sh"
fi

# Check if A (ComfyUI) is running
if curl -s http://localhost:8188/system_stats >/dev/null 2>&1; then
    echo "  ✅ A (ComfyUI) :8188 is running"
else
    echo "  🔨 A not detected — launching via hotel studio..."
    curl -s -X POST http://localhost:3000/api/inference/hotel/studio 2>/dev/null && echo "  ✅ Hotel studio launched" || echo "  ⚠️  Hotel studio failed — start ComfyUI manually"
fi

echo ""
echo "🎨 Creative Focus active."
echo "   P (story/code)  → http://localhost:3000"
echo "   A (art/voice)   → http://localhost:8188"
echo ""
echo "   IDEs are OFF. Full VRAM available for creative pipeline."
echo "   To switch back: scripts/focus/code-focus.sh"
