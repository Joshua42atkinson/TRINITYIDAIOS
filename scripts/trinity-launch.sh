#!/bin/bash
# Trinity Launcher — One-Click Environment Orchestrator
# Designed for educators to smoothly launch the OS without using a terminal

set -e

LOG_FILE="/tmp/trinity_launch.log"
echo "--- Trinity UI Launcher Boot --- $(date)" > $LOG_FILE

# Ensure dependencies
if ! command -v pg_isready &> /dev/null; then
  echo "[ERROR] PostgreSQL is not installed or not in PATH." | tee -a $LOG_FILE
  notify-send "Trinity Setup Error" "PostgreSQL is missing. Please contact your IT administrator."
  exit 1
fi

# 1. Background Mistral 4 via llama-server if it isn't running
if curl -s http://127.0.0.1:8080/health &> /dev/null; then
  echo "✅ llama.cpp is already running." | tee -a $LOG_FILE
else
  echo "🚀 Starting Mistral Small 4 (119B MoE) via llama-server..." | tee -a $LOG_FILE
  nohup /home/joshua/Workflow/desktop_trinity/trinity-genesis/llama.cpp/build-vulkan/bin/llama-server \
    --model /home/joshua/trinity-models/gguf/Mistral-Small-4-119B-2603-Q4_K_M-00001-of-00002.gguf \
    --port 8080 --host 127.0.0.1 \
    --ctx-size 131072 --n-gpu-layers 99 --flash-attn on --parallel 2 > /tmp/trinity_llama_server.log 2>&1 &
  echo "   [Started LLM background process: $!]" | tee -a $LOG_FILE
fi

# 2. Check Trinity Headless Server
if curl -s http://127.0.0.1:3000/api/health &> /dev/null; then
  echo "✅ Trinity Headless Service is already running." | tee -a $LOG_FILE
else
  echo "🚀 Starting Trinity Headless Server..." | tee -a $LOG_FILE
  cd /home/joshua/Workflow/desktop_trinity/trinity-genesis
  nohup cargo run -p trinity --release > /tmp/trinity_server.log 2>&1 &
  echo "   [Started Trinity Server process: $!]" | tee -a $LOG_FILE
fi

# 3. Wait for the server to bind (Max 15 seconds)
echo "⏳ Waiting for UI engine..." | tee -a $LOG_FILE
for i in {1..15}; do
  if curl -s http://127.0.0.1:3000/api/health &> /dev/null; then
    echo "✅ UI Engine is online!" | tee -a $LOG_FILE
    xdg-open "http://localhost:3000/trinity/"
    exit 0
  fi
  sleep 1
done

echo "[ERROR] UI Engine timed out during startup." | tee -a $LOG_FILE
notify-send "Trinity Timeout" "The local server took too long to boot. Check /tmp/trinity_launch.log"
exit 1
