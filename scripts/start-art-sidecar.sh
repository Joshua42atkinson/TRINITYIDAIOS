#!/bin/bash
# ════════════════════════════════════════════════════════════════════
# ART SIDECAR — Creative Studio for Bevy Developers
# ════════════════════════════════════════════════════════════════════
# A complete AI-powered creative studio for game development
# 
# Components:
#   - Brain (Crow-9B):     Reasoning, planning, prompts
#   - Hands (Qwen-25B):    Code generation, Bevy expertise  
#   - Images (ComfyUI):    SDXL Turbo diffusion
#   - 3D (Trellis):        Image → Mesh generation
#
# Ports:
#   8081 — Brain LLM
#   8083 — Hands LLM  
#   8188 — ComfyUI
#   8189 — Trellis 3D (optional)
#
# Memory: ~35GB GPU + context
# ════════════════════════════════════════════════════════════════════

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Paths
MODEL_DIR="$HOME/trinity-models/gguf"
LLAMA_BIN="$HOME/Workflow/desktop_trinity/trinity-genesis/llama.cpp/build-vulkan/bin/llama-server"
COMFYUI_DIR="$HOME/ComfyUI"
TRELLIS_DIR="$HOME/trinity-models/safetensors/trellis"

# Models
BRAIN="Crow-9B-Opus-4.6-Distill-Heretic_Qwen3.5.i1-Q4_K_M.gguf"
HANDS="Qwen3-Coder-REAP-25B-A3B-Rust-Q4_K_M.gguf"

# Parse arguments
START_COMFYUI=true
START_TRELLIS=false
VERBOSE=false

for arg in "$@"; do
    case $arg in
        --no-comfy) START_COMFYUI=false ;;
        --with-3d) START_TRELLIS=true ;;
        --verbose) VERBOSE=true ;;
        --help)
            echo "Usage: $0 [options]"
            echo ""
            echo "Options:"
            echo "  --no-comfy    Skip ComfyUI startup"
            echo "  --with-3d     Include Trellis 3D mesh generator"
            echo "  --verbose     Show detailed logs"
            echo "  --help        Show this help"
            exit 0
            ;;
    esac
done

# Kill existing processes
echo -e "${YELLOW}Stopping existing services...${NC}"
pkill -f "llama-server.*--port 808" 2>/dev/null || true
pkill -f "python.*main.py.*--port 8188" 2>/dev/null || true
sleep 2

# Banner
echo ""
echo -e "${PURPLE}╔══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${PURPLE}║${NC}  ${CYAN}🎨 ART SIDECAR — Creative Studio for Bevy Developers${NC}      ${PURPLE}║${NC}"
echo -e "${PURPLE}╠══════════════════════════════════════════════════════════════╣${NC}"
echo -e "${PURPLE}║${NC}  ${GREEN}Brain${NC}:  Crow-9B Opus (6GB)     → :8081  Reasoning    ${PURPLE}║${NC}"
echo -e "${PURPLE}║${NC}  ${GREEN}Hands${NC}:  Qwen-25B Rust (15GB)  → :8083  Code Gen    ${PURPLE}║${NC}"
if [ "$START_COMFYUI" = true ]; then
echo -e "${PURPLE}║${NC}  ${GREEN}Images${NC}: SDXL Turbo (7GB)     → :8188  Diffusion   ${PURPLE}║${NC}"
fi
if [ "$START_TRELLIS" = true ]; then
echo -e "${PURPLE}║${NC}  ${GREEN}3D${NC}:     Trellis-4B (16GB)     → :8189  Mesh Gen    ${PURPLE}║${NC}"
fi
echo -e "${PURPLE}╚══════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Verify models
echo -e "${BLUE}Verifying models...${NC}"
for model in "$BRAIN" "$HANDS"; do
    if [ ! -f "$MODEL_DIR/$model" ]; then
        echo -e "${RED}❌ Model not found: $MODEL_DIR/$model${NC}"
        exit 1
    fi
    echo -e "${GREEN}✓${NC} $model"
done

if [ ! -f "$LLAMA_BIN" ]; then
    echo -e "${RED}❌ llama-server not found: $LLAMA_BIN${NC}"
    echo -e "${YELLOW}   Build with: cd llama.cpp && cmake -B build-vulkan -DGGML_VULKAN=ON && cmake --build build-vulkan${NC}"
    exit 1
fi

# Start Brain (Primary)
echo ""
echo -e "${CYAN}🧠 Starting Brain (Crow-9B) on :8081...${NC}"
nohup "$LLAMA_BIN" \
    -m "$MODEL_DIR/$BRAIN" \
    -c 16384 \
    --port 8081 \
    -ngl 99 \
    --threads 8 \
    --temp 0.7 \
    > /tmp/art-brain.log 2>&1 &
BRAIN_PID=$!
echo -e "${GREEN}   PID: $BRAIN_PID${NC}"

sleep 3

# Start Hands (Code Generator)
echo -e "${CYAN}🔧 Starting Hands (Qwen-25B Rust) on :8083...${NC}"
nohup "$LLAMA_BIN" \
    -m "$MODEL_DIR/$HANDS" \
    -c 8192 \
    --port 8083 \
    -ngl 99 \
    --threads 8 \
    --temp 0.3 \
    > /tmp/art-hands.log 2>&1 &
HANDS_PID=$!
echo -e "${GREEN}   PID: $HANDS_PID${NC}"

# Start ComfyUI
if [ "$START_COMFYUI" = true ]; then
    sleep 3
    echo -e "${CYAN}🖼️  Starting ComfyUI (SDXL Turbo) on :8188...${NC}"
    if [ -d "$COMFYUI_DIR" ]; then
        cd "$COMFYUI_DIR"
        nohup python3 main.py --port 8188 --listen > /tmp/art-comfyui.log 2>&1 &
        COMFY_PID=$!
        echo -e "${GREEN}   PID: $COMFY_PID${NC}"
        cd - > /dev/null
    else
        echo -e "${YELLOW}   ⚠️ ComfyUI not found at $COMFYUI_DIR${NC}"
    fi
fi

# Start Trellis 3D (optional)
if [ "$START_TRELLIS" = true ]; then
    sleep 3
    echo -e "${CYAN}🎲 Starting Trellis 3D on :8189...${NC}"
    if [ -d "$TRELLIS_DIR" ]; then
        # Trellis requires Python server - placeholder for now
        echo -e "${YELLOW}   ⚠️ Trellis server not yet implemented${NC}"
        echo -e "${YELLOW}   Models available at: $TRELLIS_DIR${NC}"
    else
        echo -e "${YELLOW}   ⚠️ Trellis not found at $TRELLIS_DIR${NC}"
    fi
fi

# Wait for services
echo ""
echo -e "${BLUE}Waiting for services to initialize...${NC}"
sleep 8

# Verify endpoints
echo ""
echo -e "${BLUE}Verifying endpoints...${NC}"

check_endpoint() {
    local port=$1
    local name=$2
    local url="http://localhost:$port/v1/models"
    
    if curl -s --max-time 5 "$url" | grep -q "object"; then
        echo -e "${GREEN}✅ $name${NC} (:$port) — Ready"
        return 0
    else
        echo -e "${YELLOW}⏳ $name${NC} (:$port) — Loading..."
        return 1
    fi
}

check_comfy() {
    local url="http://localhost:8188/system_stats"
    
    if curl -s --max-time 5 "$url" | grep -q "system"; then
        echo -e "${GREEN}✅ ComfyUI${NC} (:8188) — Ready"
        return 0
    else
        echo -e "${YELLOW}⏳ ComfyUI${NC} (:8188) — Loading..."
        return 1
    fi
}

check_endpoint 8081 "Brain"
check_endpoint 8083 "Hands"
if [ "$START_COMFYUI" = true ]; then
    check_comfy
fi

# Summary
echo ""
echo -e "${PURPLE}══════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}ART SIDECAR READY${NC}"
echo ""
echo -e "Endpoints:"
echo -e "  ${CYAN}Brain${NC}:  http://localhost:8081/v1/chat/completions"
echo -e "  ${CYAN}Hands${NC}:  http://localhost:8083/v1/chat/completions"
if [ "$START_COMFYUI" = true ]; then
echo -e "  ${CYAN}Images${NC}: http://localhost:8188/api"
fi
echo ""
echo -e "Logs:"
echo -e "  ${YELLOW}/tmp/art-brain.log${NC}"
echo -e "  ${YELLOW}/tmp/art-hands.log${NC}"
if [ "$START_COMFYUI" = true ]; then
echo -e "  ${YELLOW}/tmp/art-comfyui.log${NC}"
fi
echo -e "${PURPLE}══════════════════════════════════════════════════════════════${NC}"

# Quick test
echo ""
echo -e "${BLUE}Quick test (Brain):${NC}"
echo 'curl -s http://localhost:8081/v1/chat/completions -H "Content-Type: application/json" -d '"'"'{"model":"crow","messages":[{"role":"user","content":"Hello"}],"max_tokens":20}'"'"' | jq -r .choices[0].message.content'
