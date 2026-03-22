#!/bin/bash
# =============================================================================
# LFM2.5-Audio-1.5B Benchmark Suite for Trinity
# AMD Ryzen AI Max+ 395 "Strix Halo" - 128GB Unified Memory
# =============================================================================
# Usage: ./benchmark_lfm_audio.sh [cpu|gpu]
#
# PATH A (CPU): Runs entirely on Zen 5 cores, leaves GPU 100% free for Mistral
# PATH B (GPU): Uses RDNA 3.5 compute units via ROCm/PyTorch
# =============================================================================

set -e

# Configuration
CKPT_DIR="$HOME/trinity-models/lfm2.5-audio-gguf"
INPUT_WAV="$HOME/trinity-models/lfm2.5-audio-gguf/test_input.wav"
OUTPUT_WAV="$HOME/trinity-models/lfm2.5-audio-gguf/test_output.wav"
THREADS=16  # 1:1 mapping to physical Zen 5 cores

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_header() {
    echo -e "${YELLOW}========================================${NC}"
    echo -e "${YELLOW}$1${NC}"
    echo -e "${YELLOW}========================================${NC}"
}

# =============================================================================
# PATH A: CPU Isolation (GGUF / llama.cpp with AVX-512)
# =============================================================================
run_cpu_path() {
    print_header "PATH A: CPU Isolation (AVX-512 / Zen 5)"
    
    CLI="$HOME/llama.cpp-lfm2.5/build/bin/llama-liquid-audio-cli"
    
    if [ ! -f "$CLI" ]; then
        echo -e "${RED}ERROR: llama-liquid-audio-cli not found${NC}"
        echo "Run: cd ~/llama.cpp-lfm2.5/build && cmake .. -DGGML_AVX512=ON -DGGML_AVX512_VNNI=ON -DGGML_AVX512_BF16=ON && make -j\$(nproc) llama-liquid-audio-cli"
        exit 1
    fi
    
    # Check for GGUF files
    for file in LFM2.5-Audio-1.5B-Q4_0.gguf mmproj-LFM2.5-Audio-1.5B-Q4_0.gguf vocoder-LFM2.5-Audio-1.5B-Q4_0.gguf tokenizer-LFM2.5-Audio-1.5B-Q4_0.gguf; do
        if [ ! -f "$CKPT_DIR/$file" ]; then
            echo -e "${RED}ERROR: Missing $file${NC}"
            exit 1
        fi
    done
    
    echo -e "${GREEN}CPU Configuration:${NC}"
    echo "  - Threads: $THREADS (physical cores only, no HT)"
    echo "  - AVX-512: Enabled (512-bit data path)"
    echo "  - AVX-512 VNNI: Enabled (vector neural network instructions)"
    echo "  - AVX-512 BF16: Enabled (bfloat16 native support)"
    echo ""
    
    # Create test audio if not exists (1 second of silence at 24kHz)
    if [ ! -f "$INPUT_WAV" ]; then
        echo "Creating test input audio (1s silence at 24kHz)..."
        ffmpeg -f lavfi -i anullsrc=r=24000:cl=mono -t 1 -y "$INPUT_WAV" 2>/dev/null
    fi
    
    echo -e "${GREEN}Running interleaved audio chat benchmark...${NC}"
    echo "Command:"
    echo "$CLI \\"
    echo "  -m $CKPT_DIR/LFM2.5-Audio-1.5B-Q4_0.gguf \\"
    echo "  -mm $CKPT_DIR/mmproj-LFM2.5-Audio-1.5B-Q4_0.gguf \\"
    echo "  -mv $CKPT_DIR/vocoder-LFM2.5-Audio-1.5B-Q4_0.gguf \\"
    echo "  --tts-speaker-file $CKPT_DIR/tokenizer-LFM2.5-Audio-1.5B-Q4_0.gguf \\"
    echo "  --threads $THREADS \\"
    echo "  -sys \"Respond with interleaved text and audio.\" \\"
    echo "  --audio $INPUT_WAV \\"
    echo "  --output $OUTPUT_WAV"
    echo ""
    
    # Time the execution
    START_TIME=$(date +%s.%N)
    
    "$CLI" \
        -m "$CKPT_DIR/LFM2.5-Audio-1.5B-Q4_0.gguf" \
        -mm "$CKPT_DIR/mmproj-LFM2.5-Audio-1.5B-Q4_0.gguf" \
        -mv "$CKPT_DIR/vocoder-LFM2.5-Audio-1.5B-Q4_0.gguf" \
        --tts-speaker-file "$CKPT_DIR/tokenizer-LFM2.5-Audio-1.5B-Q4_0.gguf" \
        --threads "$THREADS" \
        -sys "Respond with interleaved text and audio." \
        --audio "$INPUT_WAV" \
        --output "$OUTPUT_WAV"
    
    END_TIME=$(date +%s.%N)
    ELAPSED=$(echo "$END_TIME - $START_TIME" | bc)
    
    echo ""
    echo -e "${GREEN}=== CPU Path Results ===${NC}"
    echo "  Total time: ${ELAPSED}s"
    if [ -f "$OUTPUT_WAV" ]; then
        DURATION=$(ffprobe -i "$OUTPUT_WAV" -show_entries format=duration -v quiet -of csv="p=0")
        echo "  Output audio duration: ${DURATION}s"
        echo "  Real-time factor: $(echo "scale=2; $ELAPSED / $DURATION" | bc)x"
    fi
    echo ""
    echo -e "${GREEN}GPU remains 100% free for Mistral Small 4${NC}"
}

# =============================================================================
# PATH B: GPU Compute (ROCm / PyTorch)
# =============================================================================
run_gpu_path() {
    print_header "PATH B: GPU Compute (ROCm / PyTorch)"
    
    # Set ROCm environment
    export HSA_OVERRIDE_GFX_VERSION=11.0.0
    export HIP_VISIBLE_DEVICES=0
    
    echo -e "${GREEN}GPU Configuration:${NC}"
    echo "  - Device: ROCm (maps to 'cuda' in PyTorch)"
    echo "  - Model: LFM2.5-Audio-1.5B (~3GB VRAM)"
    echo "  - Remaining GPU memory: ~125GB for Mistral Small 4"
    echo ""
    
    # Run Python benchmark
    source "$HOME/trinity-vllm-env/bin/activate"
    python3 "$HOME/Workflow/desktop_trinity/trinity-genesis/scripts/benchmark/benchmark_lfm_gpu.py"
}

# =============================================================================
# Main
# =============================================================================
case "${1:-}" in
    cpu)
        run_cpu_path
        ;;
    gpu)
        run_gpu_path
        ;;
    both)
        run_cpu_path
        echo ""
        run_gpu_path
        ;;
    *)
        echo "Usage: $0 [cpu|gpu|both]"
        echo ""
        echo "  cpu  - Run CPU-only path (AVX-512, leaves GPU free)"
        echo "  gpu  - Run GPU path (ROCm/PyTorch)"
        echo "  both - Run both paths for comparison"
        exit 1
        ;;
esac
