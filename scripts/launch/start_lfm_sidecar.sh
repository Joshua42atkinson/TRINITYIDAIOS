#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# TRINITY ID AI OS — LFM2.5-Audio Voice Sidecar
# Speech-to-speech conversation manager for Trinity
# Isolated process — crashes don't affect Trinity core (Rust)
# ═══════════════════════════════════════════════════════════════════════════════
# Usage:
#   ./start_lfm_sidecar.sh [cpu|gpu]
#
# CPU path: Uses AVX-512 optimized llama.cpp - leaves GPU 100% free for Mistral
# GPU path: Uses ROCm/PyTorch - faster but shares GPU memory
# ═══════════════════════════════════════════════════════════════════════════════
set -e

MODE="${1:-cpu}"
VENV="${HOME}/trinity-vllm-env"
CKPT_DIR="${HOME}/trinity-models/lfm2.5-audio-gguf"
CLI="${HOME}/llama.cpp-lfm2.5/build/bin/llama-liquid-audio-cli"
SERVER="${HOME}/llama.cpp-lfm2.5/build/bin/llama-liquid-audio-server"
PORT="${LFM_PORT:-7861}"
THREADS=16  # 1:1 mapping to physical Zen 5 cores

# ROCm environment (for GPU path)
export HSA_OVERRIDE_GFX_VERSION=11.0.0
export HIP_VISIBLE_DEVICES=0
export ROCBLAS_USE_HIPBLASLT=1

echo "═══════════════════════════════════════"
echo "  LFM2.5-Audio Voice Sidecar"
echo "  Mode: $MODE"
echo "  Port: $PORT"
echo "═══════════════════════════════════════"

# Check for GGUF files
for file in LFM2.5-Audio-1.5B-Q4_0.gguf mmproj-LFM2.5-Audio-1.5B-Q4_0.gguf vocoder-LFM2.5-Audio-1.5B-Q4_0.gguf tokenizer-LFM2.5-Audio-1.5B-Q4_0.gguf; do
    if [ ! -f "$CKPT_DIR/$file" ]; then
        echo "ERROR: Missing $file"
        echo "Run: huggingface-cli download LiquidAI/LFM2.5-Audio-1.5B-GGUF $file --local-dir $CKPT_DIR"
        exit 1
    fi
done

case "$MODE" in
    cpu)
        # ═════════════════════════════════════════════════════════════════════════
        # CPU PATH: AVX-512 optimized llama.cpp
        # - Leaves GPU 100% free for Mistral Small 4
        # - ~1.7s response time, 138 tok/s
        # - Uses 16 physical Zen 5 cores
        # ═════════════════════════════════════════════════════════════════════════
        if [ ! -f "$SERVER" ]; then
            echo "ERROR: llama-liquid-audio-server not found"
            echo "Build with: cd ~/llama.cpp-lfm2.5/build && make -j\$(nproc) llama-liquid-audio-server"
            exit 1
        fi
        
        echo "  Path: CPU (AVX-512 / Zen 5)"
        echo "  Threads: $THREADS"
        echo "  GPU: FREE for Mistral Small 4"
        echo "  API: http://localhost:$PORT"
        echo "═══════════════════════════════════════"
        
        # Run HTTP server for audio chat API
        # System prompt for Trinity voice conversation
        SYS_PROMPT="You are Trinity, an AI instructional design assistant. Respond naturally with interleaved text and audio. Keep responses concise and helpful for K-12 teachers."
        
        exec "$SERVER" \
            -m "$CKPT_DIR/LFM2.5-Audio-1.5B-Q4_0.gguf" \
            -mm "$CKPT_DIR/mmproj-LFM2.5-Audio-1.5B-Q4_0.gguf" \
            -mv "$CKPT_DIR/vocoder-LFM2.5-Audio-1.5B-Q4_0.gguf" \
            --tts-speaker-file "$CKPT_DIR/tokenizer-LFM2.5-Audio-1.5B-Q4_0.gguf" \
            --threads "$THREADS" \
            --host 0.0.0.0 \
            --port "$PORT" \
            -sys "$SYS_PROMPT"
        ;;
    
    gpu)
        # ═════════════════════════════════════════════════════════════════════════
        # GPU PATH: ROCm/PyTorch via liquid-audio
        # - Faster inference but shares GPU memory (~3GB)
        # - Uses Gradio web UI for testing
        # ═════════════════════════════════════════════════════════════════════════
        if [ ! -d "$VENV" ]; then
            echo "ERROR: Python venv not found at $VENV"
            exit 1
        fi
        
        source "$VENV/bin/activate"
        
        echo "  Path: GPU (ROCm / PyTorch)"
        echo "  Memory: ~3GB GPU for LFM, ~125GB for Mistral"
        echo "  UI: http://localhost:$PORT"
        echo "═══════════════════════════════════════"
        
        # Run Gradio demo
        exec python -c "
from liquid_audio.demo.chat import demo
demo.launch(server_name='0.0.0.0', server_port=$PORT, share=False)
"
        ;;
    
    *)
        echo "Usage: $0 [cpu|gpu]"
        echo "  cpu - Run on CPU (default, leaves GPU free for Mistral)"
        echo "  gpu - Run on GPU (faster but shares GPU memory)"
        exit 1
        ;;
esac
