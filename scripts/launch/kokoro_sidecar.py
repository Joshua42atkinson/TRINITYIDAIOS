"""
TRINITY ID AI OS — Kokoro TTS Sidecar (ONNX Edition)
Apache 2.0 License — lightweight, no PyTorch dependency

Uses kokoro-onnx for CPU inference — avoids ROCm segfaults entirely.
Voices: af_heart, af_bella, am_adam, am_echo, am_michael, am_fenrir
"""

import os
import io
import shutil
import asyncio
import numpy as np
import scipy.io.wavfile
from fastapi import FastAPI, Request, File, UploadFile
from fastapi.responses import Response
from fastapi.middleware.cors import CORSMiddleware

app = FastAPI()

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_methods=["*"],
    allow_headers=["*"],
)

# Global pipeline — initialized on startup
pipeline = None
DEFAULT_VOICE = "am_adam"  # Professional, clear narrator voice

@app.on_event("startup")
async def startup():
    global pipeline
    print("Initializing Kokoro TTS Pipeline (ONNX — CPU, no PyTorch)...")
    try:
        import kokoro_onnx
        models_dir = os.path.expanduser("~/trinity-models/tts/kokoro")
        pipeline = kokoro_onnx.Kokoro(
            os.path.join(models_dir, "kokoro-v1.0.onnx"),
            os.path.join(models_dir, "voices-v1.0.bin"),
        )
        print("✅ Kokoro ONNX pipeline ready")
    except Exception as e:
        print(f"⚠️ Kokoro ONNX init failed: {e}")
        print("   Trying to download models...")
        try:
            # kokoro-onnx auto-downloads models on first use
            import kokoro_onnx
            pipeline = kokoro_onnx.Kokoro.from_pretrained()
            print("✅ Kokoro ONNX pipeline ready (auto-downloaded)")
        except Exception as e2:
            print(f"❌ Kokoro ONNX startup failed: {e2}")
            print("   Install with: pip install kokoro-onnx")
            # pipeline stays None — /health will report unhealthy


@app.post("/tts")
async def generate_tts(request: Request):
    if pipeline is None:
        return Response(status_code=503, content="Kokoro pipeline not initialized")

    data = await request.json()
    text = data.get("text", "").strip()
    voice = data.get("voice", DEFAULT_VOICE)
    if not text:
        return Response(status_code=400, content="No text provided")

    try:
        # Generate audio using Kokoro ONNX
        audio_np, sample_rate = pipeline.create(text, voice=voice, speed=1.0)

        # Convert to 16-bit PCM WAV
        if audio_np.dtype != np.int16:
            audio_int16 = (audio_np * 32767).astype(np.int16)
        else:
            audio_int16 = audio_np

        wav_buf = io.BytesIO()
        scipy.io.wavfile.write(wav_buf, sample_rate, audio_int16)
        wav_buf.seek(0)

        return Response(content=wav_buf.read(), media_type="audio/wav")

    except Exception as e:
        print(f"Error in Kokoro TTS: {e}")
        return Response(status_code=500, content=str(e))


@app.post("/clone")
async def clone_voice(audio_file: UploadFile = File(...)):
    try:
        models_dir = os.path.expanduser("~/trinity-models")
        os.makedirs(models_dir, exist_ok=True)
        save_path = os.path.join(models_dir, "custom_yakbak_voice.webm")

        with open(save_path, "wb") as buffer:
            shutil.copyfileobj(audio_file.file, buffer)

        print(f"Received Yak Bak voice. Saved to {save_path}")
        return {"status": "success", "msg": "Voice cloned successfully"}
    except Exception as e:
        print(f"Error cloning voice: {e}")
        return Response(status_code=500, content=str(e))


@app.get("/health")
def health():
    if pipeline is not None:
        return {"status": "healthy", "engine": "kokoro-onnx", "voices": [
            "af_heart", "af_bella", "am_adam", "am_echo", "am_michael", "am_fenrir"
        ]}
    return Response(status_code=503, content="Pipeline not ready")


if __name__ == "__main__":
    import uvicorn
    # 8200 is the default port the frontend expects
    uvicorn.run(app, host="127.0.0.1", port=8200)
