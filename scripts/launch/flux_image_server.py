import argparse
import base64
import io
import time
import uvicorn
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
import torch
from diffusers import FluxPipeline
from transformers import BitsAndBytesConfig

app = FastAPI()

class ImageRequest(BaseModel):
    prompt: str
    n: int = 1
    size: str = "1024x1024"
    response_format: str = "b64_json"
    model: str = ""

pipe = None

def init_model(model_id: str):
    global pipe
    print(f"Loading FLUX.1 [schnell] from {model_id} with Q4 on ROCm PyTorch...")
    
    # Enforce Q4 for Strix Halo to avoid FP8 OOM/Crashes
    q4_config = BitsAndBytesConfig(
        load_in_4bit=True,
        bnb_4bit_quant_type="nf4",
        bnb_4bit_compute_dtype=torch.bfloat16
    )
    
    # Text encoder typically accepts BitsAndBytes, and newer diffusers accepts it for Transformer too.
    pipe = FluxPipeline.from_pretrained(
        model_id, 
        torch_dtype=torch.bfloat16,
        quantization_config=q4_config
    ).to("cuda")
    print("Model loaded successfully!")

@app.post("/v1/images/generations")
def generate_image(req: ImageRequest):
    if not pipe:
        raise HTTPException(status_code=500, detail="Pipeline not loaded")

    # Assuming size format "1024x1024"
    width, height = 1024, 1024
    if "x" in req.size:
        parts = req.size.split("x")
        try:
            width = int(parts[0])
            height = int(parts[1])
        except ValueError:
            pass

    # Ensure divisible by 16 for flux
    width = (width // 16) * 16
    height = (height // 16) * 16

    try:
        print(f"Generating image for prompt: {req.prompt}")
        start = time.time()
        # Flux Schnell natively requires 4 inference steps and guidance_scale=0.0
        image = pipe(
            prompt=req.prompt,
            output_type="pil",
            num_inference_steps=4,
            guidance_scale=0.0,
            max_sequence_length=256,
            generator=torch.Generator("cuda").manual_seed(int(time.time()))
        ).images[0]
        
        buffered = io.BytesIO()
        image.save(buffered, format="PNG")
        img_b64 = base64.b64encode(buffered.getvalue()).decode("utf-8")
        
        print(f"Image generated in {time.time()-start:.2f}s")

        return {
            "created": int(time.time()),
            "data": [{"b64_json": img_b64}]
        }
    except Exception as e:
        print(f"Error generating image: {e}")
        raise HTTPException(status_code=500, detail=str(e))

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--model_id", type=str, required=True, help="Path to FLUX.1-schnell model")
    parser.add_argument("--port", type=int, default=8004, help="Port to bind to")
    args = parser.parse_args()

    init_model(args.model_id)

    uvicorn.run(app, host="0.0.0.0", port=args.port)
