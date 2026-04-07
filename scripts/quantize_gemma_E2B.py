import os
from transformers import AutoModelForCausalLM
from llmcompressor.transformers import oneshot
from llmcompressor.modifiers.quantization import QuantizationModifier

MODEL = "/home/joshua/trinity-models/vllm/gemma-4-E2B-it"
OUTPUT = "/home/joshua/trinity-models/vllm/gemma-4-E2B-it-AWQ-4bit"

# Structurally correct for protecting the vision towers
recipe = [
    QuantizationModifier(
        targets=["Linear"],
        scheme="W4A16",
        ignore=["lm_head", "vision_tower", "vision_proj", "multi_modal_projector"] 
    )
]

# 1. Create a swap directory
os.makedirs("/tmp/gemma_offload", exist_ok=True)

# 2. Map the model to disk before quantization
print(f"Loading {MODEL} with disk offload...")
model = AutoModelForCausalLM.from_pretrained(
    MODEL, 
    device_map="auto", 
    offload_folder="/tmp/gemma_offload", 
    low_cpu_mem_usage=True,
    trust_remote_code=True
)

# 3. Pass the mapped model object
print("Starting W4A16 oneshot quantization...")
oneshot(
    model=model, 
    dataset="HuggingFaceH4/ultrachat_200k",
    recipe=recipe,
    output_dir=OUTPUT,
    max_seq_length=2048,           # Halved from 4096. Saves ~15GB RAM.
    num_calibration_samples=256,   # Halved from 512. Mathematically sufficient for AWQ.
    clear_model_cache=True         # Forces PyTorch garbage collection per layer
)

print(f"Quantization complete. Saved to {OUTPUT}")
