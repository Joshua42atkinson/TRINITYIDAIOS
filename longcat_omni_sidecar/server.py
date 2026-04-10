"""
═══════════════════════════════════════════════════════════════════════════════
TRINITY ID AI OS — LongCat-Next Omni Sidecar (server.py)
═══════════════════════════════════════════════════════════════════════════════

PURPOSE: Serve LongCat-Next 74B MoE as the unified Omni-Brain for Trinity.
         Text, Image Generation, Image Understanding, TTS, Audio — one model.

ARCHITECTURE:
  - Dual-Process Omni-Brain Pattern
  - SGLang Engine loads model running RadixAttention and MLA.
  - SGLang handles token processing via FluentLLM Backend.
  - Python FastAPI processes VQ-VAE for audio and images, and sends
    Continuous Embeddings to the SGLang Engine.

HARDWARE: AMD Strix Halo (APU) — gfx1151, 128GB unified LPDDR5x
═══════════════════════════════════════════════════════════════════════════════
"""

import os
import sys
import json
import uuid
import base64
import torch
import asyncio
from fastapi import FastAPI, HTTPException, Request
from fastapi.responses import JSONResponse, Response, StreamingResponse
import uvicorn
import yaml

# Inject Meituan SGLang inference framework
INFERENCE_FRAMEWORK_DIR = os.path.expanduser("~/Workflow/desktop_trinity/trinity-genesis/longcat_sglang_sidecar/inference_framework")
sys.path.append(INFERENCE_FRAMEWORK_DIR)

try:
    import torchaudio
    import soundfile as sf
    def patched_torchaudio_save(filepath, src, sample_rate, *args, **kwargs):
        if src.ndim == 2: src = src.t()
        sf.write(filepath, src.detach().cpu().numpy(), sample_rate)
    torchaudio.save = patched_torchaudio_save
except Exception as e:
    pass

app = FastAPI(title="LongCat-Next Omni Brain — SGLang Accelerated")

MODEL_NAME = "meituan-longcat/LongCat-Next"
MODEL_PATH = os.path.expanduser("~/trinity-models/sglang/LongCat-Next")
PORT = 8010

model_loaded = False
ensemble = None

@app.on_event("startup")
async def startup_event():
    global ensemble, model_loaded
    print("═" * 60)
    print("  TRINITY ID AI OS — LongCat-Next SGLang Setup")
    print("═" * 60)
    
    if not os.path.exists(MODEL_PATH):
        print(f"⚠️  Model directory not found at {MODEL_PATH}")
        return

    try:
        from demo import NmmEnsemble
        os.environ["NMM_INFER_MODEL_ROOT"] = MODEL_PATH
        yaml_path = os.path.join(INFERENCE_FRAMEWORK_DIR, 'nmm_pf.yaml')
        
        with open(yaml_path, "r", encoding="utf8") as f:
            configs = yaml.safe_load(f)

        # Assign explicit ports to prevent collision
        configs["backend_params"]["port"] = 8011
        
        print("🧠 Initializing SGLang NvmEnsemble Backend...")
        ensemble = NmmEnsemble(configs)
        model_loaded = True
        print("✅ Dual-Process Omni-Brain ONLINE")
    except Exception as e:
        print(f"❌ Failed to load SGLang framework: {e}")
        import traceback
        traceback.print_exc()

def _mock_image_b64():
    return "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg=="

def _mock_wav_bytes():
    return b"RIFF\x24\x00\x00\x00WAVEfmt \x10\x00\x00\x00\x01\x00\x01\x00DE\x00\x00\x88\x8a\x00\x00\x02\x00\x10\x00data\x00\x00\x00\x00"

@app.get("/health")
async def health():
    return {
        "status": "ok",
        "model": MODEL_NAME,
        "loaded": model_loaded,
        "mode": "sglang_native" if model_loaded else "mock",
        "port": PORT
    }

@app.post("/v1/chat/completions")
async def chat_completions(request: Request):
    data = await request.json()
    messages = data.get("messages", [])
    max_tokens = data.get("max_tokens", 2048)
    temperature = data.get("temperature", 0.7)
    stream = data.get("stream", False)
    request_id = str(uuid.uuid4())

    question = ""
    for msg in messages:
        if isinstance(msg.get("content"), list):
            for item in msg["content"]:
                if item.get("type") == "text":
                    question += item.get("text", "")
                elif item.get("type") == "audio_url":
                    audio_url = item.get("audio_url", {}).get("url", "")
                    if audio_url.startswith("data:audio/wav;base64,"):
                        bbytes = base64.b64decode(audio_url.replace("data:audio/wav;base64,", ""))
                        path = f"/tmp/stt_{uuid.uuid4().hex[:8]}.wav"
                        with open(path, "wb") as f: f.write(bbytes)
                        question += f"<longcat_audio_start>{path}<longcat_audio_end>"
        else:
            question += str(msg.get("content", ""))

    if not model_loaded or not ensemble:
        return JSONResponse({"choices": [{"message": {"role": "assistant", "content": "[Mock - SGLang Loading]"}}]})

    raw_input = {"question": question, "request_id": request_id}
    
    # Run the Meituan PreProcessor to compute the multi-modal embeddings
    emb, input_ids = ensemble.encoder.process_from_raw_input_new(raw_input)
    emb = emb.reshape(-1, emb.shape[-1])
    orig_input_ids = ensemble.encoder.tokenizer.encode(question, add_special_tokens=False, return_tensors='pt')[0]
    gen_audio, gen_image = ensemble.need_to_generate_multi(input_ids=input_ids)
    input_ids_list = input_ids.view(-1).tolist()

    sampling_params = {"temperature": temperature, "max_new_tokens": max_tokens, "top_p": 0.85, "top_k": 5}
    input_extra_infos = {
        "gen_image": gen_image, "gen_audio": gen_audio, 
        "orig_input_ids": orig_input_ids, "delay": 0, 
        "multi_sampling_params": {"temperature": 0.2, "top_p": 0.85}, 
        "token_w": None
    }

    if stream:
        async def token_stream():
            try:
                async for res in ensemble.lm.generate(request_id, input_ids_list, input_tensor_dict={"input_embedding": emb}, sampling_params=sampling_params, input_extra_infos=input_extra_infos, stream=True):
                    if isinstance(res, dict) and "text" in res:
                        token_text = res.get("text", "")
                        if token_text:
                            chunk = json.dumps({"choices": [{"delta": {"content": token_text}, "index": 0}], "model": MODEL_NAME})
                            yield f"data: {chunk}\n\n"
                    elif isinstance(res, str):
                        # Catch error messages
                        print(f"Stream error chunk: {res}")
            except Exception as e:
                print(f"Stream aborted: {e}")
            finally:
                yield "data: [DONE]\n\n"
        return StreamingResponse(token_stream(), media_type="text/event-stream")

    output_ids = []
    async for res in ensemble.lm.generate(request_id, input_ids_list, input_tensor_dict={"input_embedding": emb}, sampling_params=sampling_params, input_extra_infos=input_extra_infos, stream=True):
        if isinstance(res, dict) and "output_ids" in res:
            output_ids.extend(res["output_ids"])
             
    text = ensemble.encoder.tokenizer.decode(output_ids[:-1], skip_special_tokens=True)
    return JSONResponse({"choices": [{"message": {"role": "assistant", "content": text}}]})

@app.post("/v1/images/generations")
async def generate_image(request: Request):
    data = await request.json()
    prompt = data.get("prompt", "")
    size = data.get("size", "512x512")
    if not model_loaded: return JSONResponse({"data": [{"b64_json": _mock_image_b64()}]})
    
    try:
        w, h = [int(x) for x in size.split("x")]
        token_h = max(1, h // 14 // 2)
        token_w = max(1, w // 14 // 2)
        img_size_prefix = f"<longcat_img_token_size>{token_h} {token_w}</longcat_img_token_size>"
        question = f"{img_size_prefix}{prompt}<longcat_img_start>"

        request_id = str(uuid.uuid4())
        raw_input = {"question": question, "request_id": request_id}
        file_path = f"/tmp/{request_id}"
        
        text, output_multi_ids, res, generated_file = await ensemble.generate_new(
            raw_input=raw_input, request_id=request_id, token_w=token_w, file_path=file_path, enable_gen_multi=True
        )
        
        if generated_file and os.path.exists(generated_file):
            with open(generated_file, "rb") as f:
                encoded = base64.b64encode(f.read()).decode("utf-8")
            return JSONResponse({"data": [{"b64_json": encoded}]})
    except Exception as e:
        print(f"❌ Image generation failed: {e}")
        import traceback
        traceback.print_exc()

    return JSONResponse({"data": [{"b64_json": _mock_image_b64()}]})

@app.post("/tts")
async def text_to_speech(request: Request):
    data = await request.json()
    text = data.get("text", "")
    voice = data.get("voice", "am_adam")
    voice_file = f"./assets/voices/{voice}.wav"
    
    if os.path.exists(voice_file):
        question = f"<longcat_audio_start>{voice_file}<longcat_audio_end>{text}<longcat_audiogen_start>"
    else:
        question = f"{text}<longcat_audiogen_start>"
        
    request_id = str(uuid.uuid4())
    raw_input = {"question": question, "request_id": request_id}
    file_path = f"/tmp/audio_{request_id}"
    
    if not model_loaded: return Response(content=_mock_wav_bytes(), media_type="audio/wav")
    
    try:
        txt, output_multi_ids, res, generated_file = await ensemble.generate_new(
            raw_input=raw_input, request_id=request_id, delay=0, file_path=file_path, enable_gen_multi=True
        )
        if generated_file and os.path.exists(generated_file):
            with open(generated_file, "rb") as f:
                return Response(content=f.read(), media_type="audio/wav")
    except Exception as e:
        print(f"❌ TTS failed: {e}")

    return Response(content=_mock_wav_bytes(), media_type="audio/wav")

@app.post("/v1/audio/generations")
async def audio_generations(request: Request):
    data = await request.json()
    prompt = data.get("prompt", "ambient background music")
    duration = min(data.get("duration", 15), 30)
    
    question = f"Generate a {duration}-second instrumental piece. Style: {prompt}<longcat_audiogen_start>"
    request_id = str(uuid.uuid4())
    file_path = f"/tmp/tempo_{request_id}"
    
    if not model_loaded: return JSONResponse({"data": [{"b64_json": base64.b64encode(_mock_wav_bytes()).decode()}]})
    
    try:
        txt, output_multi_ids, res, generated_file = await ensemble.generate_new(
            raw_input={"question": question, "request_id": request_id}, 
            request_id=request_id, 
            file_path=file_path, 
            enable_gen_multi=True
        )
        if generated_file and os.path.exists(generated_file):
            with open(generated_file, "rb") as f:
                encoded = base64.b64encode(f.read()).decode("utf-8")
            return JSONResponse({"data": [{"b64_json": encoded}]})
    except Exception as e:
        print(f"❌ TEMPO generation failed: {e}")

    return JSONResponse({"data": [{"b64_json": base64.b64encode(_mock_wav_bytes()).decode()}]})

@app.post("/v1/audio/transcriptions")
async def audio_to_text(request: Request):
    data = await request.json()
    audio_path = data.get("file", "")
    if not model_loaded: return JSONResponse({"text": ""})
    
    question = f"<longcat_audio_start>{audio_path}<longcat_audio_end>"
    request_id = str(uuid.uuid4())
    raw_input = {"question": question, "request_id": request_id}
    
    try:
        text, output_multi_ids, res, generated_file = await ensemble.generate_new(
            raw_input=raw_input, request_id=request_id, delay=0, file_path=f"/tmp/stt_{request_id}", enable_gen_multi=False
        )
        return JSONResponse({"text": text})
    except Exception as e:
        return JSONResponse({"text": ""})

if __name__ == "__main__":
    import shutil
    # Delete test temp outputs
    for f in os.listdir('/tmp/'):
        if f.startswith('stt_') or f.startswith('audio_') or f.startswith('tempo_'):
            try: os.remove(os.path.join('/tmp', f))
            except: pass
            
    print(f"Starting LongCat-Next Omni Sidecar on port {PORT}")
    uvicorn.run("server:app", host="127.0.0.1", port=PORT)
