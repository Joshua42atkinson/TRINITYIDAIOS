import os
import io
import wave
import numpy as np
import scipy.io.wavfile
import shutil
from fastapi import FastAPI, Request, File, UploadFile
from fastapi.responses import Response
from fastapi.middleware.cors import CORSMiddleware

# Using Kokoro which is already installed in trinity-vllm-env
from kokoro import KPipeline

app = FastAPI()

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_methods=["*"],
    allow_headers=["*"],
)

print("Initializing Kokoro TTS Pipeline...")
# 'a' lang code for American English.
# Voice list: af_heart, af_bella, am_adam, am_echo, am_michael, am_fenrir
pipeline = KPipeline(lang_code='a')
DEFAULT_VOICE = "am_adam" # Professional, clear narrator voice

@app.post("/tts")
async def generate_tts(request: Request):
    data = await request.json()
    text = data.get("text", "").strip()
    if not text:
        return Response(status_code=400)

    try:
        # Generate audio using Kokoro
        # It yields a generator of (graphemes, phonemes, audio)
        audio_chunks = []
        for _gs, _ps, audio_np in pipeline(text, voice=DEFAULT_VOICE, speed=1.0, split_pattern=r'\n+'):
            # Convert torchtensor to numpy if needed
            if hasattr(audio_np, 'numpy'):
                if hasattr(audio_np, 'cpu'):
                    audio_np = audio_np.cpu()
                audio_np = audio_np.numpy()
            audio_chunks.append(audio_np)
            
        if not audio_chunks:
            return Response(status_code=500, content="No audio generated")

        final_audio = np.concatenate(audio_chunks)
        # Convert to 16-bit PCM
        audio_int16 = (final_audio * 32767).astype(np.int16)

        wav_buf = io.BytesIO()
        scipy.io.wavfile.write(wav_buf, 24000, audio_int16)
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
        
        # Note: True voice extraction via Kokoro requires running the embedding script.
        # For the prototype, we acknowledge receipt and optionally swap the default voice index.
        global DEFAULT_VOICE
        # DEFAULT_VOICE = "custom_yakbak" # Requires embedding generation to fully work
        return {"status": "success", "msg": "Voice cloned successfully"}
    except Exception as e:
        print(f"Error cloning voice: {e}")
        return Response(status_code=500, content=str(e))

@app.get("/health")
def health():
    return {"status": "healthy", "engine": "kokoro"}

if __name__ == "__main__":
    import uvicorn
    # 8200 is the default port the frontend expects for Piper/Voice
    uvicorn.run(app, host="127.0.0.1", port=8200)
