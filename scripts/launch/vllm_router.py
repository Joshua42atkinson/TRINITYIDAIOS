import uvicorn
from fastapi import FastAPI, Request, Response
import httpx
import json

app = FastAPI()

# Map requested models to local vLLM backend ports
MODEL_ROUTING = {
    "Great_Recycler": "http://127.0.0.1:8001",
    "Programmer_Pete": "http://127.0.0.1:8002",
    "Tempo_Engine": "http://127.0.0.1:8003",
    "FLUX.1-schnell": "http://127.0.0.1:8004",
    "nomic-embed": "http://127.0.0.1:8005",
    "CogVideoX-2b": "http://127.0.0.1:8006",
    "TripoSR": "http://127.0.0.1:8007",
    "ACE-Step": "http://127.0.0.1:8008",
}

client = httpx.AsyncClient(timeout=300.0)

async def proxy_request(request: Request, path: str):
    body = await request.body()
    try:
        data = json.loads(body)
        model = data.get("model", "")
    except Exception:
        model = ""
    
    # Default to 31B if not specified or unknown
    target_base = MODEL_ROUTING.get(model, "http://127.0.0.1:8001")
    
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
    url = f"http://127.0.0.1:8001/v1/models"
    try:
        response = await client.get(url)
        return Response(content=response.content, status_code=response.status_code, headers=dict(response.headers))
    except Exception:
        return {"object": "list", "data": []}

if __name__ == "__main__":
    uvicorn.run(app, host="127.0.0.1", port=8000)
