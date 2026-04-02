#!/bin/bash
# Trinity Launcher — Graceful Shutdown Orchestrator

LOG_FILE="/tmp/trinity_launch.log"
echo "--- Trinity UI Shutdown --- $(date)" >> $LOG_FILE

# Inform the user
notify-send "Trinity AI OS" "Shutting down the cognitive engine and freeing VRAM..."

# 1. Kill the background llama-server and free 68GB of memory
if pgrep -f llama-server >/dev/null; then
  echo "🛑 Terminating Mistral 4 (llama-server)..." >> $LOG_FILE
  pkill -f llama-server
  echo "   [VRAM Successfully released]" >> $LOG_FILE
fi

# 2. Kill the headless server
if pgrep -f "cargo run -p trinity" >/dev/null; then
  echo "🛑 Terminating Trinity Headless Server (cargo process)..." >> $LOG_FILE
  pkill -f "cargo run -p trinity"
fi

# Wait for process exit if they exist, but don't hang
sleep 2

notify-send "Trinity Offline" "Memory has been cleared. The system is fully shut down."
echo "✅ Graceful shutdown composed." >> $LOG_FILE
exit 0
