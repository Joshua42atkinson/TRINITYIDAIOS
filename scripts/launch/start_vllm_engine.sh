#!/bin/bash
echo "Starting vLLM engine for Trinity Conductor/ART Batcher..."
echo "Model: Qwen/Qwen2.5-7B-Instruct (placeholder for Ming/Conductor)"

# Assuming the user has vllm installed via pip
python3 -m vllm.entrypoints.openai.api_server \
    --model Qwen/Qwen2.5-7B-Instruct \
    --port 8080 \
    --max-model-len 4096 \
    --gpu-memory-utilization 0.9 &

echo $! > /tmp/vllm_pid
echo "vLLM started on port 8080. PID: $(cat /tmp/vllm_pid)"
