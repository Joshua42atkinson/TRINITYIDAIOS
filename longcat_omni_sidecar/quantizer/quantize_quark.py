import torch
import transformers.utils
transformers.utils.FLAX_WEIGHTS_NAME = "flax_model.msgpack"
from transformers import AutoModelForCausalLM, AutoTokenizer
from quark.torch.quantization.api import ModelQuantizer
from quark.torch.quantization.config.config import Config, QConfig, QLayerConfig, AWQConfig, Int4PerGroupSpec
from datasets import load_dataset
import os

MODEL_ID = "/home/joshua/trinity-models/sglang/LongCat-Next"
OUTPUT_DIR = os.path.expanduser("~/trinity-models/vllm/LongCat-Next-AWQ-4bit")

print(f"Loading {MODEL_ID} into CPU cache...")
# Using device=cpu explicitly ensures we don't accidentally overload the GPU during static model load.
tokenizer = AutoTokenizer.from_pretrained(MODEL_ID)
model = AutoModelForCausalLM.from_pretrained(
    MODEL_ID, 
    device_map="cpu", 
    torch_dtype=torch.bfloat16, 
    trust_remote_code=True
)

print("Defining strict DiNA Visual bypass schema for AMD Quark v0.11...")
# The official AWQ 4-bit (w4a16) configuration explicitly mapped for ROCm inference deployment
quant_schema = QLayerConfig(weight=[Int4PerGroupSpec(ch_axis=1, group_size=128).to_quantization_spec()])
quant_config = QConfig(
    global_quant_config=quant_schema,
    algo_config=[AWQConfig(model_decoder_layers="model.layers")],
    exclude=["router", "classifier", "lm_head", "linear1", "linear2"]
)

quantizer = ModelQuantizer(quant_config)

print("Loading calibration set (Wikitext)...")
calib_dataset = load_dataset("wikitext", "wikitext-2-raw-v1", split="train").select(range(64))

if tokenizer.pad_token is None:
    tokenizer.pad_token = tokenizer.eos_token

def calib_dataloader(dataset, tokenizer, max_length=512):
    for example in dataset:
        inputs = tokenizer(
            example["text"], 
            return_tensors="pt", 
            max_length=max_length, 
            truncation=True, 
            padding="max_length"
        )
        # Avoid passing empty tensors
        if inputs["input_ids"].size(1) > 0:
            yield inputs["input_ids"]

dataloader = calib_dataloader(calib_dataset, tokenizer)

print("Kicking off AMD Quark Calibration. This targets native CPU RAM chunks recursively...")
quantized_model = quantizer.quantize_model(model, dataloader)

print(f"Saving explicitly to {OUTPUT_DIR} for vLLM deployment...")
os.makedirs(OUTPUT_DIR, exist_ok=True)
quantized_model.save_pretrained(OUTPUT_DIR)
tokenizer.save_pretrained(OUTPUT_DIR)

print("AMD Quark Quantization Complete. Mission Accomplished.")
