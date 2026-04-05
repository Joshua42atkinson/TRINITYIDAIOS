import argparse
import base64
import time
import os
import uvicorn
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
import torch
from diffusers import AutoPipelineForText2Video
from diffusers.utils import export_to_video
from transformers import BitsAndBytesConfig

app = FastAPI()

class VideoRequest(BaseModel):
    prompt: str
    duration: int = 6
    fps: int = 8
    model: str = ""
    response_format: str = "b64_json"

pipe = None

def init_model(model_id: str):
    global pipe
    print(f"Loading CogVideoX-2b from {model_id} on ROCm PyTorch with 4-bit quantization...")
    
    nf4_config = BitsAndBytesConfig(
        load_in_4bit=True,
        bnb_4bit_quant_type="nf4",
        bnb_4bit_compute_dtype=torch.float16
    )
    
    from diffusers import CogVideoXTransformer3DModel
    transformer = CogVideoXTransformer3DModel.from_pretrained(
        model_id,
        subfolder="transformer",
        quantization_config=nf4_config,
        torch_dtype=torch.float16
    )
    
    pipe = AutoPipelineForText2Video.from_pretrained(
        model_id,
        transformer=transformer,
        torch_dtype=torch.float16
    ).to("cuda")
    print("Video Model loaded successfully in 4-bit NF4!")

@app.post("/v1/video/generations")
def generate_video(req: VideoRequest):
    if not pipe:
        raise HTTPException(status_code=500, detail="Pipeline not loaded")

    try:
        print(f"Generating video for prompt: {req.prompt}")
        start = time.time()
        
        # CogVideoX-2B natively generates 49 frames (~6 seconds at 8 fps)
        video_frames = pipe(
            prompt=req.prompt,
            num_inference_steps=50,
            num_frames=49,
            guidance_scale=6.0,
            generator=torch.Generator("cuda").manual_seed(int(time.time()))
        ).frames[0]
        
        # Save locally to temp file to read bytes
        temp_path = f"/tmp/cogvideo_{int(time.time())}.mp4"
        export_to_video(video_frames, temp_path, fps=req.fps)
        
        with open(temp_path, "rb") as f:
            v_b64 = base64.b64encode(f.read()).decode("utf-8")
            
        # Cleanup
        os.remove(temp_path)
        
        print(f"Video generated in {time.time()-start:.2f}s")

        return {
            "created": int(time.time()),
            "data": [{"b64_json": v_b64}]
        }
    except Exception as e:
        print(f"Error generating video: {e}")
        raise HTTPException(status_code=500, detail=str(e))

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--model_id", type=str, required=True, help="Path to CogVideoX-2b model")
    parser.add_argument("--port", type=int, default=8006, help="Port to bind to")
    args = parser.parse_args()

    init_model(args.model_id)
    uvicorn.run(app, host="0.0.0.0", port=args.port)
