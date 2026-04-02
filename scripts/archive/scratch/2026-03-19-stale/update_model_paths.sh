#!/bin/bash

# Update subagent model paths to use real models

echo "🔄 Updating subagent model paths to use real models..."

# Define real model paths
MINIMAX_PATH="/home/joshua/.lmstudio/models/RemySkye/MiniMax-M2.5-REAP-50-GGUF/MiniMax-M2-5-REAP-50-Q4_K_M.gguf"
QWEN35B_PATH="/home/joshua/.lmstudio/models/lmstudio-community/Qwen3.5-35B-A3B-GGUF/Qwen3.5-35B-A3B-Q4_K_M.gguf"

# Update conductor to use MiniMax
echo "📌 Conductor -> MiniMax (66GB)"
sed -i "s|/models/conductor/personaplex-7b-v1-Q4_K_M.gguf|$MINIMAX_PATH|g" crates/trinity-subagents/trinity-conductor/src/real_brain.rs

# Update dispatcher to use MiniMax (good for coding)
echo "📌 Dispatcher -> MiniMax (66GB)"
sed -i "s|/models/dispatcher/Fortytwo_Strand-Rust-Coder-14B-v1-Q4_K_M.gguf|$MINIMAX_PATH|g" crates/trinity-subagents/trinity-dispatcher/src/real_brain.rs

# Update draftsman to use Qwen3.5-35B (good for creative)
echo "📌 Draftsman -> Qwen3.5-35B (20GB)"
sed -i "s|/models/draftsman/Creative-Design-35B-v1-Q4_K_M.gguf|$QWEN35B_PATH|g" crates/trinity-subagents/trinity-draftsman/src/real_brain.rs

# Update other subagents to use MiniMax for now
SUBAGENTS=("engineer" "yardmaster" "brakeman" "diffusion" "nitrogen" "omni")
for subagent in "${SUBAGENTS[@]}"; do
    echo "📌 $subagent -> MiniMax (66GB)"
    sed -i "s|/models/$subagent/.*\.gguf|$MINIMAX_PATH|g" "crates/trinity-subagents/trinity-$subagent/src/real_brain.rs"
done

echo "✅ Model paths updated!"
echo ""
echo "📊 Model Summary:"
echo "  - MiniMax-M2.5-REAP-50: 66GB (General purpose, coding)"
echo "  - Qwen3.5-35B-A3B: 20GB (Multimodal, creative)"
echo ""
echo "🚀 Ready to load real models!"
