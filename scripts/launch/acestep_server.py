import argparse
import base64
import time
import io
import os
import uvicorn
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
import torch
import torchaudio

app = FastAPI()

class MusicRequest(BaseModel):
    prompt: str
    lyrics: str = ""
    duration: float = 60.0
    model: str = ""
    response_format: str = "b64_json"

pipe = None

def init_model(model_id: str):
    global pipe
    print(f"Loading ACE-Step from {model_id}...")
    
    from acestep.pipeline_ace_step import ACEStepPipeline
    
    pipe = ACEStepPipeline.from_pretrained(model_id, torch_dtype=torch.bfloat16)
    pipe.to("cuda")
    print("ACE-Step Music Model loaded successfully!")

@app.post("/v1/audio/generations")
def generate_music(req: MusicRequest):
    if not pipe:
        raise HTTPException(status_code=500, detail="Pipeline not loaded")

    try:
        print(f"Generating music for prompt: {req.prompt[:80]}...")
        start = time.time()
        
        # Generate audio
        audio = pipe(
            prompt=req.prompt,
            lyrics=req.lyrics if req.lyrics else "[instrumental]",
            duration=req.duration,
            num_inference_steps=100,
        )
        
        # Convert to WAV bytes
        waveform = audio.audios[0]  # shape: [channels, samples]
        if isinstance(waveform, torch.Tensor):
            waveform_tensor = waveform
        else:
            import numpy as np
            waveform_tensor = torch.from_numpy(np.array(waveform))
        
        if waveform_tensor.dim() == 1:
            waveform_tensor = waveform_tensor.unsqueeze(0)
        
        buffer = io.BytesIO()
        torchaudio.save(buffer, waveform_tensor.cpu(), sample_rate=44100, format="wav")
        audio_b64 = base64.b64encode(buffer.getvalue()).decode("utf-8")
        
        print(f"Music generated in {time.time()-start:.2f}s")

        return {
            "created": int(time.time()),
            "data": [{"b64_json": audio_b64, "format": "wav"}]
        }
    except Exception as e:
        print(f"Error generating music: {e}")
        raise HTTPException(status_code=500, detail=str(e))

@app.get("/health")
def health():
    return {"status": "ok", "model": "ACE-Step-v1-3.5B"}

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--model_id", type=str, required=True, help="Path to ACE-Step model")
    parser.add_argument("--port", type=int, default=8008, help="Port to bind to")
    args = parser.parse_args()

    init_model(args.model_id)
    uvicorn.run(app, host="0.0.0.0", port=args.port)
