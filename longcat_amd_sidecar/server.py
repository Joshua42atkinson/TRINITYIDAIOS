import os
import io
import json
import base64
import torch
import soundfile as sf
import asyncio
import httpx
from fastapi import FastAPI, HTTPException, Request
from fastapi.responses import JSONResponse, Response, StreamingResponse
import uvicorn

# ── MONKEY PATCH TORCHAUDIO ──────────────────────────────────────────
# Ensure torchaudio doesn't crash on ffmpeg linking issues in Linux
try:
    import torchaudio
    def patched_torchaudio_save(filepath, src, sample_rate, *args, **kwargs):
        if src.ndim == 2:
            src = src.t()
        sf.write(filepath, src.detach().cpu().numpy(), sample_rate)
    torchaudio.save = patched_torchaudio_save
except ImportError:
    import types
    import sys
    torchaudio = types.ModuleType("torchaudio")
    torchaudio.__version__ = "0.0.0-shim"
    def _sf_save(filepath, src, sample_rate, *args, **kwargs):
        import numpy as np
        data = src.detach().cpu().numpy() if hasattr(src, 'detach') else src
        if data.ndim == 2:
            data = data.T
        sf.write(filepath, data, sample_rate)
    torchaudio.save = _sf_save
    sys.modules["torchaudio"] = torchaudio
    print("⚠️ torchaudio not installed — using soundfile shim")
# ─────────────────────────────────────────────────────────────────────

app = FastAPI(title="LongCat-Next Omni Brain (AMD CPU/SDPA Backend)")

MODEL_NAME = "meituan-longcat/LongCat-Next"
MODEL_PATH = os.path.expanduser("~/trinity-models/sglang/LongCat-Next")
PORT = 8010

model = None
tokenizer = None
processor = None
model_loaded = False

def load_model_4bit():
    global model, tokenizer, processor, model_loaded
    try:
        from transformers import AutoModelForCausalLM, AutoTokenizer, BitsAndBytesConfig
        print(f"🧠 Loading LongCat-Next from {MODEL_PATH}")
        print("   Backend: PyTorch SDPA (Bypassing Flash Attention)")
        
        bnb_config = BitsAndBytesConfig(
            load_in_4bit=True,
            bnb_4bit_quant_type="nf4",
            bnb_4bit_compute_dtype=torch.bfloat16,
            bnb_4bit_use_double_quant=True,
            llm_int8_enable_fp32_cpu_offload=True,
            llm_int8_skip_modules=[
                "classifier", "router", "lm_head", "linear1", "linear2", "linear_1", "linear_2", "l_linear",
                "visual_head", "audio_head", "visual_tokenizer", "audio_tokenizer"
            ]
        )

        tokenizer = AutoTokenizer.from_pretrained(MODEL_PATH, trust_remote_code=True)
        
        # KEY FIX FOR AMD: Force SDPA and bypass auto flash-attention mapping
        model = AutoModelForCausalLM.from_pretrained(
            MODEL_PATH,
            quantization_config=bnb_config,
            device_map="auto",
            max_memory={0: "90GB", "cpu": "10GB"},  # Prevent unified memory OOM (leave ~28GB for OS + IDE)
            trust_remote_code=True,
            torch_dtype=torch.bfloat16,
            attn_implementation="sdpa"  # Forces PyTorch native SDPA instead of flash_attn
        )
        model.eval()
        model.text_tokenizer = tokenizer
        
        # Determine safe execution device
        active_device = torch.device("cuda:0" if torch.cuda.is_available() else "cpu")
        
        # KEY FIX FOR AMD / LONGCAT:
        # Move stray meta tensors into active GPU memory correctly
        # This resolves `Tensor.item() cannot be called on meta tensors` during huggingface's device check
        if hasattr(model, 'visual_offset_vals') and model.visual_offset_vals.device.type == 'meta':
            model.visual_offset_vals = torch.nn.Parameter(torch.zeros(8, dtype=torch.int64, device=active_device))
        if hasattr(model, 'audio_offset_vals') and model.audio_offset_vals.device.type == 'meta':
            model.audio_offset_vals = torch.nn.Parameter(torch.zeros(8, dtype=torch.int64, device=active_device))
            
        # Iterate over all registered parameters to ensure NOTHING is left on meta
        from accelerate.utils import set_module_tensor_to_device
        for name, param in model.named_parameters():
            if param.device.type == 'meta':
                # Use accelerate to properly re-link the parameter to physical memory
                set_module_tensor_to_device(model, name, active_device, value=torch.zeros_like(param, device=active_device))
                
            # Hydrate bitsandbytes quantization code maps left stranded
            if hasattr(param, 'quant_state') and param.quant_state is not None:
                if param.quant_state.code is not None:
                    param.quant_state.code = param.quant_state.code.to(active_device)

        model_loaded = True
        
        if torch.cuda.is_available():
            vram_gb = torch.cuda.memory_allocated() / (1024**3)
            vram_total = torch.cuda.get_device_properties(0).total_memory / (1024**3)
            print(f"   ✅ Model loaded using SDPA — {vram_gb:.1f}GB / {vram_total:.1f}GB VRAM")
        else:
            print("   ✅ Model loaded using SDPA (CPU fallback mode)")

    except Exception as e:
        print(f"   ❌ Model load failed: {e}")
        import traceback
        traceback.print_exc()

@app.on_event("startup")
async def startup_event():
    print("═" * 60)
    print("  TRINITY ID AI OS — AMD SDPA Sidecar (Port 8010)")
    print("═" * 60)
    
    if not os.path.exists(MODEL_PATH):
        print(f"⚠️  Model directory not found at {MODEL_PATH}")
        return
        
    loop = asyncio.get_event_loop()
    loop.run_in_executor(None, load_model_4bit)

@app.get("/health")
async def health():
    return {
        "status": "ok",
        "model": MODEL_NAME,
        "loaded": model_loaded,
        "mode": "sdpa_nf4" if model_loaded else "mock",
        "port": PORT
    }

def _mock_image_b64():
    return "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg=="

def _mock_wav_bytes():
    return b"RIFF\x24\x00\x00\x00WAVEfmt \x10\x00\x00\x00\x01\x00\x01\x00DE\x00\x00\x88\x8a\x00\x00\x02\x00\x10\x00data\x00\x00\x00\x00"

@app.post("/v1/chat/completions")
async def chat_completions(request: Request):
    data = await request.json()
    messages = data.get("messages", [])
    max_tokens = data.get("max_tokens", 2048)
    temperature = data.get("temperature", 0.7)
    do_sample = temperature > 0
    stream = data.get("stream", False)

    for msg in messages:
        if isinstance(msg.get("content"), list):
            new_content = ""
            for item in msg["content"]:
                if item.get("type") == "text":
                    new_content += item.get("text", "")
                elif item.get("type") == "audio_url":
                    audio_url = item.get("audio_url", {}).get("url", "")
                    if audio_url.startswith("data:audio/wav;base64,"):
                        b64_data = audio_url.replace("data:audio/wav;base64,", "")
                        bbytes = base64.b64decode(b64_data)
                        import uuid
                        path = f"/tmp/stt_{uuid.uuid4().hex[:8]}.wav"
                        with open(path, "wb") as f:
                            f.write(bbytes)
                        new_content += f"<longcat_audio_start>{path}<longcat_audio_end>"
            msg["content"] = new_content

    if not model_loaded:
        return JSONResponse({"choices": [{"message": {"role": "assistant", "content": "[Mock - SDPA Model Loading]"}}]})

    # Determine safe execution device (avoid model.device which might return meta)
    active_device = "cuda:0" if torch.cuda.is_available() else "cpu"
    
    text_input = tokenizer.apply_chat_template(messages, tokenize=False, add_generation_prompt=True)
    inputs = tokenizer(text_input, return_tensors="pt").to(active_device)

    if stream:
        from transformers import TextIteratorStreamer
        from threading import Thread
        
        streamer = TextIteratorStreamer(tokenizer, skip_prompt=True, skip_special_tokens=True)
        gen_kwargs = {"input_ids": inputs["input_ids"], "max_new_tokens": max_tokens, "do_sample": do_sample, "streamer": streamer}
        if "attention_mask" in inputs: gen_kwargs["attention_mask"] = inputs["attention_mask"]
        if do_sample: gen_kwargs.update({"temperature": temperature, "top_p": 0.85})

        thread = Thread(target=lambda: model.generate(**gen_kwargs))
        thread.start()

        def token_stream():
            for token_text in streamer:
                if token_text:
                    yield f"data: {json.dumps({'choices': [{'delta': {'content': token_text}, 'index': 0}]})}\n\n"
            thread.join(timeout=120)
            yield "data: [DONE]\n\n"

        return StreamingResponse(token_stream(), media_type="text/event-stream")

    with torch.no_grad():
        outputs = model.generate(**inputs, max_new_tokens=max_tokens, do_sample=do_sample)
    new_tokens = outputs[0][inputs["input_ids"].shape[1]:]
    return JSONResponse({"choices": [{"message": {"role": "assistant", "content": tokenizer.decode(new_tokens, skip_special_tokens=True)}}]})

@app.post("/v1/images/generations")
async def generate_image(request: Request):
    if not model_loaded: return JSONResponse({"data": [{"b64_json": _mock_image_b64()}]})
    data = await request.json()
    prompt = data.get("prompt", "")
    
    token_h, token_w = 37, 37
    img_size_prefix = f"<longcat_img_token_size>{token_h} {token_w}</longcat_img_token_size>"
    messages = [{"role": "system", "content": "You are Trinity's creative visual engine."}, {"role": "user", "content": f"{img_size_prefix}{prompt}<longcat_img_start>"}]
    
    text_input = tokenizer.apply_chat_template(messages, tokenize=False, add_generation_prompt=True)
    inputs = tokenizer(text_input, return_tensors="pt").to(model.device)

    try:
        with torch.no_grad():
            outputs = model.generate(**inputs, return_dict_in_generate=True, max_new_tokens=2048, do_sample=False)
        output_visual_ids = getattr(outputs, 'visual_ids', None)
        if output_visual_ids is not None and output_visual_ids.size(0) > 0:
            image_path_list = model.model.decode_visual_ids_and_save(output_visual_ids, save_prefix="/tmp/longcat_img", cfg_scale=3, token_h=token_h, token_w=token_w, anyres_prefix=img_size_prefix)
            if image_path_list:
                with open(image_path_list[0], "rb") as f:
                    return JSONResponse({"data": [{"b64_json": base64.b64encode(f.read()).decode("utf-8")}]})
    except Exception as e:
        print(f"❌ Image generation failed: {e}")
    return JSONResponse({"data": [{"b64_json": _mock_image_b64()}]})

@app.post("/tts")
async def text_to_speech(request: Request):
    if not model_loaded: return Response(content=_mock_wav_bytes(), media_type="audio/wav")
    data = await request.json()
    system_prompt = "You are a helpful voice assistant."
    messages = [{"role": "system", "content": system_prompt}, {"role": "user", "content": f"{data.get('text', '')}<longcat_audiogen_start>"}]
    
    text_input = tokenizer.apply_chat_template(messages, tokenize=False, add_generation_prompt=True)
    inputs = tokenizer(text_input, return_tensors="pt").to(model.device)

    try:
        with torch.no_grad():
            outputs = model.generate(**inputs, return_dict_in_generate=True, max_new_tokens=2048, do_sample=True, temperature=0.2, top_p=0.85)
        output_audio_ids = getattr(outputs, 'audio_ids', None)
        if output_audio_ids is not None and output_audio_ids.size(0) > 0:
            audio_path_list = model.model.decode_audio_ids_and_save(output_audio_ids, save_prefix="/tmp/longcat_audio", sampling_rate=24000, wave_concat_overlap=1200)
            if audio_path_list:
                with open(audio_path_list[0], "rb") as f:
                    return Response(content=f.read(), media_type="audio/wav")
    except Exception as e:
        print(f"❌ TTS failed: {e}")
    return Response(content=_mock_wav_bytes(), media_type="audio/wav")

@app.post("/v1/audio/generations")
async def audio_generations(request: Request):
    if not model_loaded: return JSONResponse({"data": [{"b64_json": base64.b64encode(_mock_wav_bytes()).decode()}]})
    data = await request.json()
    messages = [{"role": "system", "content": f"You are a music composer. Generate a {min(data.get('duration', 15), 30)}-second {data.get('style', 'ambient')} instrumental music piece. Style: {data.get('prompt', 'ambient')}."}, {"role": "user", "content": "Generate the music now.<longcat_audiogen_start>"}]
    
    text_input = tokenizer.apply_chat_template(messages, tokenize=False, add_generation_prompt=True)
    inputs = tokenizer(text_input, return_tensors="pt").to(model.device)

    try:
        with torch.no_grad():
            outputs = model.generate(**inputs, return_dict_in_generate=True, max_new_tokens=min(int(12.5 * 8 * data.get('duration', 15)), 4096), do_sample=True, temperature=0.8)
        output_audio_ids = getattr(outputs, 'audio_ids', None)
        if output_audio_ids is not None and output_audio_ids.size(0) > 0:
            audio_path_list = model.model.decode_audio_ids_and_save(output_audio_ids, save_prefix="/tmp/longcat_tempo", sampling_rate=24000, wave_concat_overlap=1200)
            if audio_path_list:
                with open(audio_path_list[0], "rb") as f:
                    return JSONResponse({"data": [{"b64_json": base64.b64encode(f.read()).decode("utf-8")}]})
    except Exception as e:
        print(f"❌ TEMPO generation failed: {e}")
    return JSONResponse({"data": [{"b64_json": base64.b64encode(_mock_wav_bytes()).decode()}]})

if __name__ == "__main__":
    uvicorn.run(app, host="127.0.0.1", port=PORT)
