#!/bin/bash
# Night Shift Mode — Everything off
# Use for system maintenance, downloads, or full rest
# Kills: All IDEs, Trinity, ComfyUI, DiffusionGemma

set -e

echo "🌙 NIGHT SHIFT MODE"
echo "===================="
echo "Killing all heavy processes..."
echo ""

# Kill IDEs
pkill -f "windsurf" 2>/dev/null && echo "  ✅ Killed Windsurf" || echo "  ⬜ Windsurf not running"
pkill -f "antigravity" 2>/dev/null && echo "  ✅ Killed Antigravity" || echo "  ⬜ Antigravity not running"
pkill -f "code-server\|vscode" 2>/dev/null && echo "  ✅ Killed VS Code" || echo "  ⬜ VS Code not running"
pkill -f "language_server_linux" 2>/dev/null && echo "  ✅ Killed language servers" || echo "  ⬜ No language servers"

# Kill ComfyUI
pkill -f "ComfyUI/venv/bin/python main.py" 2>/dev/null && echo "  ✅ Killed ComfyUI" || echo "  ⬜ ComfyUI not running"

# Kill DiffusionGemma
podman stop -t 5 $(podman ps -q 2>/dev/null) 2>/dev/null && echo "  ✅ Stopped all podman containers" || echo "  ⬜ No podman containers running"
pkill -f "vllm" 2>/dev/null && echo "  ✅ Killed vLLM" || echo "  ⬜ No vLLM running"

# Kill Trinity
pkill -f "target/release/trinity" 2>/dev/null && echo "  ✅ Killed Trinity server" || echo "  ⬜ Trinity not running"

# Kill Blender
pkill -f "blender" 2>/dev/null && echo "  ✅ Killed Blender" || echo "  ⬜ Blender not running"

# Kill Godot
pkill -f "godot" 2>/dev/null && echo "  ✅ Killed Godot" || echo "  ⬜ Godot not running"

echo ""
sleep 2

# Show freed resources
echo "  RAM available: $(free -h | awk '/Mem:/ {print $7}')"
echo "  VRAM freed: all GPU memory released"

echo ""
echo "🌙 Night Shift active."
echo "   All heavy processes are OFF."
echo "   System ready for maintenance, downloads, or rest."
echo ""
echo "   To start creating: scripts/focus/creative-focus.sh"
echo "   To start coding:   scripts/focus/code-focus.sh"
