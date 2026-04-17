from transformers import AutoModelForCausalLM, BitsAndBytesConfig
import torch
import os

print("Loading model...")
bnb_config = BitsAndBytesConfig(
    load_in_4bit=True,
    bnb_4bit_quant_type="nf4",
    bnb_4bit_compute_dtype=torch.bfloat16,
    bnb_4bit_use_double_quant=True,
    llm_int8_enable_fp32_cpu_offload=True,
    llm_int8_skip_modules=[
        "classifier", "router", "lm_head", "linear1", "linear2", "linear_1", "linear_2", "l_linear",
        "visual_head", "audio_head", "visual_tokenizer", "audio_tokenizer"
    ]
)
model = AutoModelForCausalLM.from_pretrained(
    "/home/joshua/trinity-models/sglang/LongCat-Next",
    quantization_config=bnb_config,
    device_map="auto",
    max_memory={0: "90GB", "cpu": "10GB"},
    trust_remote_code=True,
    torch_dtype=torch.bfloat16,
    attn_implementation="sdpa"
)
print("Finding meta parameters:")
for n, p in model.named_parameters():
    if getattr(p, 'device', None) is not None and p.device.type == 'meta':
        print(f"META: {n}")
