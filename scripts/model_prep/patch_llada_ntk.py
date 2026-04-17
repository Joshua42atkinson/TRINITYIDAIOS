import os
from transformers import AutoConfig, AutoModelForCausalLM, AutoTokenizer
import torch

def patch_llada_model():
    print("🚀 Trinity Model Prep: Patching LLaDA2.1-mini for 256K Dynamic NTK...")
    
    model_id = "inclusionAI/LLaDA2.1-mini"
    home_dir = os.environ.get("HOME", "/tmp")
    save_path = os.path.join(home_dir, "trinity-models", "LLaDA2.1-mini-256k-dynamic-ntk")
    
    os.makedirs(save_path, exist_ok=True)
    
    print(f"Loading config from {model_id}...")
    config = AutoConfig.from_pretrained(
        model_id,
        trust_remote_code=True,
    )
    
    # 262144 / 32768 = 8.0 base scaling
    print("Applying Dynamic NTK Rope Scaling (factor 8.0) for 256K context limit...")
    config.rope_scaling = {
        "type": "dynamic",
        "factor": 8.0,                    
    }
    config.max_position_embeddings = 262144
    
    print("Loading base model weights (bfloat16)...")
    model = AutoModelForCausalLM.from_pretrained(
        model_id,
        config=config,
        trust_remote_code=True,
        torch_dtype=torch.bfloat16,
        device_map="auto",
    )
    
    print(f"Pre-saving model with baked-in 256K config to: {save_path}...")
    model.save_pretrained(save_path)
    
    print("Downloading and saving tokenizer...")
    tokenizer = AutoTokenizer.from_pretrained(model_id)
    tokenizer.save_pretrained(save_path)
    
    print("✅ Model successfully saved!")
    print(f"You can now load it in vLLM or Trinity using the path: {save_path}")

if __name__ == "__main__":
    patch_llada_model()
