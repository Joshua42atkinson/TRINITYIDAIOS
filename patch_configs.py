import json
import glob

files = glob.glob("/home/joshua/trinity-models/vllm/*/config.json")
for f in files:
    with open(f, 'r') as file:
        data = json.load(file)
    
    changed = False
    
    # Change architectures
    if data.get("model_type") == "gemma4":
        data["model_type"] = "gemma"
        changed = True
        
    if data.get("model_type") == "hunyuan":
        data["model_type"] = "stable-diffusion-xl"  # Standard placeholder for diffusion
        changed = True
        
    if "text_config" in data and data["text_config"].get("model_type") == "gemma4_text":
        data["text_config"]["model_type"] = "gemma"
        changed = True
        
    if changed:
        with open(f, 'w') as file:
            json.dump(data, file, indent=2)
        print(f"Patched {f}")

