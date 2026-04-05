import argparse
import base64
import time
import os
import uvicorn
import io
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
import torch
from PIL import Image

# Requires TripoSR installed: `pip install tsr`
try:
    from tsr.system import TSR
except ImportError:
    TSR = None

app = FastAPI()

class MeshRequest(BaseModel):
    image_base64: str
    prompt: str = ""

model = None

def init_model(model_dir: str):
    global model
    if TSR is None:
        print("Warning: tsr library not found. TripoSR endpoint will start but may fail generating.")
        return
        
    print(f"Loading TripoSR from {model_dir} on ROCm PyTorch...")
    model = TSR.from_pretrained(
        model_dir,
        config_name="config.yaml",
        weight_name="model.ckpt",
    )
    model.renderer.set_chunk_size(131072)
    model.to("cuda")
    print("3D Model loaded successfully!")

@app.post("/v1/3d/generations")
def generate_3d(req: MeshRequest):
    if not model:
        raise HTTPException(status_code=500, detail="TripoSR Model not loaded or tsr missing")

    try:
        print(f"Generating 3D mesh...")
        start = time.time()
        
        # Decode image
        img_bytes = base64.b64decode(req.image_base64)
        image = Image.open(io.BytesIO(img_bytes)).convert("RGB")
        
        if image.size != (800, 800):
            image = image.resize((800, 800)) # recommended by TripoSR
            
        with torch.no_grad():
            scene_codes = model(image, device="cuda")
            temp_path = f"/tmp/triposr_mesh_{int(time.time())}.obj"
            mesh = model.extract_mesh(scene_codes)[0]
            mesh.export(temp_path)
        
        with open(temp_path, "r") as f:
            v_b64 = base64.b64encode(f.read().encode("utf-8")).decode("utf-8")
            
        # Cleanup
        os.remove(temp_path)
        
        print(f"3D mesh generated in {time.time()-start:.2f}s")

        return {
            "created": int(time.time()),
            "data": [{"obj_base64": v_b64}]
        }
    except Exception as e:
        print(f"Error generating 3D: {e}")
        raise HTTPException(status_code=500, detail=str(e))

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--model_id", type=str, required=True, help="Path to TripoSR model")
    parser.add_argument("--port", type=int, default=8007, help="Port to bind to")
    args = parser.parse_args()

    init_model(args.model_id)
    uvicorn.run(app, host="0.0.0.0", port=args.port)
