import torch
from transformers import AutoModelForCausalLM, BitsAndBytesConfig

print("Starting trace...")

MODEL_PATH = "/home/joshua/trinity-models/sglang/LongCat-Next"

bnb_config = BitsAndBytesConfig(
    load_in_4bit=True,
    bnb_4bit_quant_type="nf4",
    bnb_4bit_compute_dtype=torch.bfloat16,
    bnb_4bit_use_double_quant=True,
    llm_int8_skip_modules=["classifier", "router", "lm_head", "linear1", "linear2", "linear_1", "linear_2", "l_linear"] # Using original skip modules
)

print("Loading model...")
model = AutoModelForCausalLM.from_pretrained(
    MODEL_PATH,
    quantization_config=bnb_config,
    device_map="auto",
    trust_remote_code=True,
    torch_dtype=torch.bfloat16,
)

print("Model loaded successfully!")
