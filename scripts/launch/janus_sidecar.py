import os
import io
import base64
import torch
import uvicorn
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from typing import List, Optional, Dict, Any
from PIL import Image

# Suppress warnings
import warnings
warnings.filterwarnings("ignore")

app = FastAPI(title="Trinity Aesthetics — Janus Pro 7B Sidecar")

MODEL_PATH = "deepseek-ai/Janus-Pro-7B"
DEVICE = "cuda" if torch.cuda.is_available() else "cpu"

print(f"Loading Janus Pro 7B on {DEVICE}...")

# Lazy loading of model to avoid crash if dependencies aren't installed
vl_chat_processor = None
vl_gpt = None

def load_model():
    global vl_chat_processor, vl_gpt
    if vl_gpt is None:
        try:
            from transformers import AutoModelForCausalLM
            from janus.models import MultiModalityCausalLM, VLChatProcessor

            vl_chat_processor = VLChatProcessor.from_pretrained(MODEL_PATH)
            vl_gpt = AutoModelForCausalLM.from_pretrained(
                MODEL_PATH, trust_remote_code=True, torch_dtype=torch.bfloat16
            )
            vl_gpt = vl_gpt.to(DEVICE).eval()
            print("Janus Pro 7B loaded successfully.")
        except Exception as e:
            print(f"Failed to load Janus model: {e}")
            raise

# --- API Models ---

class ChatMessage(BaseModel):
    role: str
    content: Any # string or list of dicts for multimodal

class ChatCompletionRequest(BaseModel):
    model: str
    messages: List[ChatMessage]
    max_tokens: int = 512
    temperature: float = 0.1

class ImageGenerationRequest(BaseModel):
    prompt: str
    n: int = 1
    size: str = "384x384"
    response_format: str = "b64_json"

# --- Endpoints ---

@app.get("/health")
def health():
    return {"status": "ok", "model": MODEL_PATH}

@app.post("/v1/chat/completions")
def chat_completions(req: ChatCompletionRequest):
    load_model()
    # Basic mock interface for now to establish the HTTP boundary
    # Full implementation requires formatting the PIL image and running the VLProcessor
    return {
        "id": "janus-vision-xyz",
        "object": "chat.completion",
        "choices": [{
            "index": 0,
            "message": {
                "role": "assistant",
                "content": "Janus Pro 7B Vision Understanding placeholder. (Full integration pending)"
            },
            "finish_reason": "stop"
        }]
    }

@app.post("/v1/images/generations")
def generate_images(req: ImageGenerationRequest):
    load_model()
    # Basic mock for text-to-image
    return {
        "created": 1234567890,
        "data": [
            {
                "b64_json": "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII=" 
            }
        ]
    }

if __name__ == "__main__":
    print("🚀 Starting Janus Pro 7B sidecar on port 8003...")
    # Delay model loading until first request to allow fast startup
    uvicorn.run(app, host="127.0.0.1", port=8003)
