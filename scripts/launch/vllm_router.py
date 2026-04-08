import uvicorn
from fastapi import FastAPI, Request, Response
import httpx
import json

app = FastAPI()

# Map requested models to local backend ports.
# This router is the A.R.T.Y. Hub — it handles everything Pete (SGLang :8010) can't do.
# Pete/Great Recycler runs on SGLang port 8010 and is NOT routed through here.
#
# P.A.R.T.Y. Framework (April 2026):
#   A = Aesthetics (FLUX, CogVideoX, TripoSR)
#   R = Research (nomic-embed for RAG)
#   T = Tempo (ACE-Step music generation)
#   Y = Yardmaster (Qwen3-REAP coding subagent)
MODEL_ROUTING = {
    # A — Aesthetics
    "FLUX.1-schnell": "http://127.0.0.1:8004",
    "CogVideoX-2b": "http://127.0.0.1:8006",
    "TripoSR": "http://127.0.0.1:8007",
    # R — Research
    "nomic-embed": "http://127.0.0.1:8005",
    # T — Tempo
    "ACE-Step": "http://127.0.0.1:8008",
    # Y — Yardmaster
    "Qwen3-REAP": "http://127.0.0.1:8009",
}

client = httpx.AsyncClient(timeout=300.0)

async def proxy_request(request: Request, path: str):
    body = await request.body()
    try:
        data = json.loads(body)
        model = data.get("model", "")
    except Exception:
        model = ""
    
    # Default to Yardmaster REAP for unrecognized models
    target_base = MODEL_ROUTING.get(model, "http://127.0.0.1:8009")
    
    url = f"{target_base}/{path}"
    headers = dict(request.headers)
    headers.pop("host", None)
    
    response = await client.post(url, content=body, headers=headers)
    return Response(content=response.content, status_code=response.status_code, headers=dict(response.headers))

@app.post("/v1/chat/completions")
async def chat_completions(request: Request):
    return await proxy_request(request, "v1/chat/completions")

@app.post("/v1/completions")
async def completions(request: Request):
    return await proxy_request(request, "v1/completions")

@app.post("/v1/images/generations")
async def images_generations(request: Request):
    return await proxy_request(request, "v1/images/generations")

@app.post("/v1/audio/generations")
async def audio_generations(request: Request):
    return await proxy_request(request, "v1/audio/generations")

@app.post("/v1/video/generations")
async def video_generations(request: Request):
    return await proxy_request(request, "v1/video/generations")

@app.post("/v1/embeddings")
async def embeddings(request: Request):
    return await proxy_request(request, "v1/embeddings")

@app.post("/v1/3d/generations")
async def mesh_generations(request: Request):
    return await proxy_request(request, "v1/3d/generations")

@app.get("/health")
async def health():
    return {"status": "ok"}

@app.get("/v1/models")
async def models(request: Request):
    # Return model list from routing table (no single upstream to query)
    model_list = [{"id": name, "object": "model"} for name in MODEL_ROUTING.keys()]
    return {"object": "list", "data": model_list}

if __name__ == "__main__":
    uvicorn.run(app, host="127.0.0.1", port=8000)
