"""
═══════════════════════════════════════════════════════════════════════════════
TRINITY ID AI OS — LongCat-Next Omni Sidecar (server.py)
═══════════════════════════════════════════════════════════════════════════════

PURPOSE: Serve LongCat-Next 74B MoE as the unified Omni-Brain for Trinity.
         Text, Image Generation, Image Understanding, TTS, Audio — one model.

ARCHITECTURE:
  - Loads model via transformers with 4-bit NF4 quantization (bitsandbytes)
  - Full bf16→NF4 compresses 151GB → ~38GB in-flight VRAM
  - All multimodal decoders (dNaViT vision, CosyVoice audio) remain intact
  - Serves OpenAI-compatible endpoints on port 8010

HARDWARE: AMD Strix Halo (APU) — gfx1151, 128GB unified LPDDR5x
═══════════════════════════════════════════════════════════════════════════════
"""

import os
import io
import base64
import json
import base64
import torch
import torchaudio
import soundfile as sf

# ── MONKEY PATCH TORCHAUDIO ──────────────────────────────────────────
# torchcodec fails to link its precompiled ffmpeg dependencies in Fedora.
# We intercept all torchaudio.save calls here and route them strictly 
# through the native python-soundfile library.
def patched_torchaudio_save(filepath, src, sample_rate, *args, **kwargs):
    if src.ndim == 2:
        src = src.t()  # Transpose from (Channels, Length) to (Length, Channels)
    sf.write(filepath, src.detach().cpu().numpy(), sample_rate)

torchaudio.save = patched_torchaudio_save
# ─────────────────────────────────────────────────────────────────────

import asyncio
import httpx
from fastapi import FastAPI, HTTPException, Request
from fastapi.responses import JSONResponse, Response, StreamingResponse
import uvicorn

app = FastAPI(title="LongCat-Next Omni Brain — Trinity ID AI OS")

MODEL_NAME = "meituan-longcat/LongCat-Next"
MODEL_PATH = os.path.expanduser("~/trinity-models/sglang/LongCat-Next")
PORT = 8010

# Global model state
model = None
tokenizer = None
processor = None
model_loaded = False


def load_model_4bit():
    """Load LongCat-Next with 4-bit NF4 quantization via bitsandbytes."""
    global model, tokenizer, processor, model_loaded

    try:
        from transformers import AutoModelForCausalLM, AutoTokenizer, BitsAndBytesConfig

        print(f"🧠 Loading LongCat-Next from {MODEL_PATH}")
        print(f"   Quantization: 4-bit NF4 (bitsandbytes)")
        print(f"   Target VRAM: ~38GB (down from 151GB bf16)")

        # 4-bit NF4 quantization config — preserves multimodal decoder precision
        bnb_config = BitsAndBytesConfig(
            load_in_4bit=True,
            bnb_4bit_quant_type="nf4",
            bnb_4bit_compute_dtype=torch.bfloat16,
            bnb_4bit_use_double_quant=True,  # nested quantization for extra savings
            llm_int8_skip_modules=["classifier", "router", "lm_head", "linear1", "linear2", "linear_1", "linear_2", "l_linear"]
        )

        tokenizer = AutoTokenizer.from_pretrained(
            MODEL_PATH,
            trust_remote_code=True
        )
        print("   ✅ Tokenizer loaded")

        model = AutoModelForCausalLM.from_pretrained(
            MODEL_PATH,
            quantization_config=bnb_config,
            device_map="auto",
            trust_remote_code=True,
            torch_dtype=torch.bfloat16,
        )
        model.eval()
        model.text_tokenizer = tokenizer
        model_loaded = True

        # Report VRAM usage after load
        if torch.cuda.is_available():
            vram_gb = torch.cuda.memory_allocated() / (1024**3)
            vram_total = torch.cuda.get_device_properties(0).total_memory / (1024**3)
            print(f"   ✅ Model loaded — {vram_gb:.1f}GB / {vram_total:.1f}GB VRAM")
        else:
            print("   ✅ Model loaded (CPU mode)")

        print("   🎯 DiNA Omni-Brain ONLINE — text/image/audio ready")

    except Exception as e:
        print(f"   ❌ Model load failed: {e}")
        import traceback
        traceback.print_exc()
        print("   ⚠️  Running in MOCK mode — endpoints return structural placeholders")


@app.on_event("startup")
async def startup_event():
    """Non-blocking startup — loads model in background thread."""
    print("═" * 60)
    print("  TRINITY ID AI OS — LongCat-Next Omni Sidecar")
    print("═" * 60)

    if not os.path.exists(MODEL_PATH):
        print(f"⚠️  Model directory not found at {MODEL_PATH}")
        print("   Running in MOCK mode until model is downloaded.")
        return

    # Load model in a background thread to not block FastAPI startup
    loop = asyncio.get_event_loop()
    loop.run_in_executor(None, load_model_4bit)


@app.get("/health")
async def health():
    """Health check endpoint — reports model status."""
    return {
        "status": "ok",
        "model": MODEL_NAME,
        "loaded": model_loaded,
        "mode": "4bit_nf4" if model_loaded else "mock",
        "port": PORT,
        "capabilities": ["text", "image_generation", "image_understanding",
                         "tts", "audio_understanding", "voice_cloning"]
    }


@app.post("/v1/chat/completions")
async def chat_completions(request: Request):
    """OpenAI-compatible chat completions — text generation."""
    data = await request.json()
    messages = data.get("messages", [])
    max_tokens = data.get("max_tokens", 2048)
    temperature = data.get("temperature", 0.7)
    do_sample = temperature > 0

    if not model_loaded:
        return JSONResponse({
            "choices": [{
                "message": {
                    "role": "assistant",
                    "content": "[LongCat-Next not yet loaded — mock response]"
                }
            }]
        })

    try:
        text_input = tokenizer.apply_chat_template(
            messages, tokenize=False, add_generation_prompt=True
        )
        inputs = tokenizer(text_input, return_tensors="pt").to(model.device)

        with torch.no_grad():
            outputs = model.generate(
                **inputs,
                max_new_tokens=max_tokens,
                do_sample=do_sample,
                temperature=temperature if do_sample else None,
                top_p=0.85 if do_sample else None,
            )

        # Decode only new tokens
        new_tokens = outputs[0][inputs["input_ids"].shape[1]:]
        response_text = tokenizer.decode(new_tokens, skip_special_tokens=True)

        return JSONResponse({
            "choices": [{
                "message": {
                    "role": "assistant",
                    "content": response_text
                }
            }],
            "model": MODEL_NAME,
            "usage": {"total_tokens": len(outputs[0])}
        })
    except Exception as e:
        import traceback
        traceback.print_exc()
        raise HTTPException(status_code=500, detail=f"Generation failed: {e}")


@app.post("/v1/images/generations")
async def generate_image(request: Request):
    """
    Image generation via DiNA visual tokens.
    Uses <longcat_img_start> trigger to force visual token generation.
    The model's built-in image decoder (dNaViT + FLUX VAE) renders the output.
    """
    data = await request.json()
    prompt = data.get("prompt", "")
    size = data.get("size", "512x512")

    if not model_loaded:
        print(f"MOCK: Generating image for prompt: {prompt}")
        return JSONResponse({"data": [{"b64_json": _mock_image_b64()}]})

    try:
        # Parse target dimensions from size string
        try:
            w, h = [int(x) for x in size.split("x")]
        except:
            w, h = 512, 512

        # Calculate token dimensions (14px per token, /2 for spatial merge)
        token_h = max(1, h // 14 // 2)
        token_w = max(1, w // 14 // 2)

        # Force image generation with <longcat_img_start> suffix
        img_size_prefix = f"<longcat_img_token_size>{token_h} {token_w}</longcat_img_token_size>"
        messages = [
            {"role": "system", "content": "You are Trinity's creative visual engine."},
            {"role": "user", "content": f"{img_size_prefix}{prompt}<longcat_img_start>"}
        ]

        text_input = tokenizer.apply_chat_template(
            messages, tokenize=False, add_generation_prompt=True
        )
        inputs = tokenizer(text_input, return_tensors="pt").to(model.device)

        with torch.no_grad():
            outputs = model.generate(
                **inputs,
                return_dict_in_generate=True,
                max_new_tokens=2048,
                do_sample=False,
                # Visual generation config from model's recommended params
            )

        # Check for visual token output
        output_visual_ids = getattr(outputs, 'visual_ids', None)
        if output_visual_ids is not None and output_visual_ids.size(0) > 0:
            save_prefix = "/tmp/longcat_img"
            # Use the model's built-in visual decoder
            gen_config = getattr(model.generation_config, 'visual_generation_config', {})
            custom_params = gen_config.get("custom_params", {
                "cfg_scale": 3,
                "token_h": token_h,
                "token_w": token_w,
                "anyres_prefix": f"<longcat_img_token_size>{token_h} {token_w}</longcat_img_token_size>"
            })

            image_path_list = model.model.decode_visual_ids_and_save(
                output_visual_ids,
                save_prefix=save_prefix,
                **custom_params
            )

            if image_path_list:
                with open(image_path_list[0], "rb") as f:
                    encoded = base64.b64encode(f.read()).decode("utf-8")
                return JSONResponse({"data": [{"b64_json": encoded}]})

        # Fallback: model generated text instead of visual tokens
        print("⚠️ Model returned text instead of visual tokens")
        return JSONResponse({"data": [{"b64_json": _mock_image_b64()}]})

    except Exception as e:
        print(f"❌ Image generation failed: {e}")
        import traceback
        traceback.print_exc()
        return JSONResponse({"data": [{"b64_json": _mock_image_b64()}]})


@app.post("/tts")
async def text_to_speech(request: Request):
    """
    Text-to-Speech via DiNA audio tokens.
    Uses <longcat_audiogen_start> trigger for speech synthesis.
    The model's built-in CosyVoice vocoder renders the audio.
    """
    data = await request.json()
    text = data.get("text", "")
    voice = data.get("voice", "am_adam")

    # Try to find a voice reference file for cloning
    voice_file = f"./assets/voices/{voice}.wav"
    if not os.path.exists(voice_file):
        voice_file = os.path.join(MODEL_PATH, "assets", "system_audio.wav")

    if not model_loaded:
        print(f"MOCK: TTS for voice {voice}: {text}")
        return Response(content=_mock_wav_bytes(), media_type="audio/wav")

    try:
        # Build messages with voice reference for cloning
        if os.path.exists(voice_file):
            system_prompt = f"Replicate the voice in the audio clip to formulate an answer:<longcat_audio_start>{voice_file}<longcat_audio_end>"
        else:
            system_prompt = "You are a helpful voice assistant."

        messages = [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": f"{text}<longcat_audiogen_start>"}
        ]

        text_input = tokenizer.apply_chat_template(
            messages, tokenize=False, add_generation_prompt=True
        )
        inputs = tokenizer(text_input, return_tensors="pt").to(model.device)

        with torch.no_grad():
            outputs = model.generate(
                **inputs,
                return_dict_in_generate=True,
                max_new_tokens=2048,
                do_sample=True,
                temperature=0.2,
                top_k=20,
                top_p=0.85,
                repetition_penalty=1.1,
            )

        # Check for audio token output
        output_audio_ids = getattr(outputs, 'audio_ids', None)
        if output_audio_ids is not None and output_audio_ids.size(0) > 0:
            save_prefix = "/tmp/longcat_audio"
            gen_config = getattr(model.generation_config, 'audio_generation_config', {})
            custom_params = gen_config.get("custom_params", {
                "sampling_rate": 24000,
                "wave_concat_overlap": 1200
            })

            audio_path_list = model.model.decode_audio_ids_and_save(
                output_audio_ids,
                save_prefix=save_prefix,
                **custom_params
            )

            if audio_path_list:
                with open(audio_path_list[0], "rb") as f:
                    return Response(content=f.read(), media_type="audio/wav")

        print("⚠️ Model returned text instead of audio tokens")
        return Response(content=_mock_wav_bytes(), media_type="audio/wav")

    except Exception as e:
        print(f"❌ TTS generation failed: {e}")
        import traceback
        traceback.print_exc()
        return Response(content=_mock_wav_bytes(), media_type="audio/wav")


@app.post("/v1/audio/transcriptions")
async def audio_to_text(request: Request):
    """Audio understanding — transcription and comprehension."""
    data = await request.json()
    audio_path = data.get("file", "")

    if not model_loaded:
        return JSONResponse({"text": "[LongCat-Next not loaded — mock transcription]"})

    try:
        messages = [
            {"role": "user", "content": f"<longcat_audio_start>{audio_path}<longcat_audio_end>"}
        ]

        text_input = tokenizer.apply_chat_template(
            messages, tokenize=False, add_generation_prompt=True
        )
        inputs = tokenizer(text_input, return_tensors="pt").to(model.device)

        with torch.no_grad():
            outputs = model.generate(
                **inputs,
                max_new_tokens=1024,
                do_sample=True,
                temperature=0.2,
                top_k=20,
                top_p=0.85,
                repetition_penalty=1.1,
            )

        new_tokens = outputs[0][inputs["input_ids"].shape[1]:]
        transcription = tokenizer.decode(new_tokens, skip_special_tokens=True)
        return JSONResponse({"text": transcription})

    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Transcription failed: {e}")


# ─── Utility Functions ────────────────────────────────────────────────────────

def _mock_image_b64():
    """1x1 transparent PNG for structural mock testing."""
    return "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg=="

def _mock_wav_bytes():
    """Minimal valid WAV header for structural mock testing."""
    return b"RIFF\x24\x00\x00\x00WAVEfmt \x10\x00\x00\x00\x01\x00\x01\x00DE\x00\x00\x88\x8a\x00\x00\x02\x00\x10\x00data\x00\x00\x00\x00"


if __name__ == "__main__":
    print(f"Starting LongCat-Next Omni Sidecar on port {PORT}")
    uvicorn.run(app, host="127.0.0.1", port=PORT)
