"""
═══════════════════════════════════════════════════════════════════════════════
TRINITY ID AI OS — A.R.T.Y. Hub Reverse Proxy (vllm_router.py)
═══════════════════════════════════════════════════════════════════════════════

PURPOSE: FastAPI reverse proxy on port 8000 that routes OpenAI-compatible
         requests to the correct downstream vLLM instances by model name.

ARCHITECTURE (P.A.R.T.Y. Framework — April 2026):
  This router IS the A.R.T.Y. Hub. It handles everything Pete (SGLang :8010)
  can't do. Pete/Great Recycler runs on the LongCat sidecar and is NOT routed
  through here.

  Downstream backends:
    Port 8005 — R: nomic-embed-text-v1.5-AWQ (embeddings for RAG)
    Port 8009 — Y: Yardmaster/Qwen3-REAP (coding subagent — optional)
    Port 8004 — A: FLUX.1-schnell (image gen — optional)
    Port 8006 — A: CogVideoX-2b (video gen — optional)
    Port 8007 — A: TripoSR (3D mesh gen — optional)
    Port 8008 — T: ACE-Step (music gen — optional)

HEALTH: /health returns honest status of which downstream backends are live.
═══════════════════════════════════════════════════════════════════════════════
"""

import uvicorn
from fastapi import FastAPI, Request, Response
import httpx
import json
import asyncio

app = FastAPI(title="A.R.T.Y. Hub — Trinity ID AI OS")

# ─── Model → Backend Routing Table ──────────────────────────────────────────
# Map requested model names to local backend ports.
# Multiple names can map to the same backend for caller convenience.
MODEL_ROUTING = {
    # R — Research (Embeddings)
    "nomic-embed-text-v1.5-AWQ": "http://127.0.0.1:8005",
    "nomic-embed":               "http://127.0.0.1:8005",
    "nomic-embed-text-v1.5":     "http://127.0.0.1:8005",

    # A — Aesthetics
    "FLUX.1-schnell":  "http://127.0.0.1:8004",
    "CogVideoX-2b":   "http://127.0.0.1:8006",
    "TripoSR":         "http://127.0.0.1:8007",

    # T — Tempo
    "ACE-Step":        "http://127.0.0.1:8008",

    # Y — Yardmaster
    "Qwen3-REAP":     "http://127.0.0.1:8009",
    "Yardmaster":      "http://127.0.0.1:8009",
}

# Endpoints that should route to embeddings by default (not Yardmaster)
EMBEDDING_PATHS = {"/v1/embeddings"}

client = httpx.AsyncClient(timeout=300.0)


async def proxy_request(request: Request, path: str):
    """Route request to the correct downstream backend based on model name."""
    body = await request.body()
    try:
        data = json.loads(body)
        model = data.get("model", "")
    except Exception:
        model = ""

    # Determine target based on model name
    if model in MODEL_ROUTING:
        target_base = MODEL_ROUTING[model]
    elif f"/{path}" in EMBEDDING_PATHS:
        # Embedding requests default to nomic-embed, not Yardmaster
        target_base = "http://127.0.0.1:8005"
    else:
        # Default: route to Yardmaster for chat/completions
        target_base = "http://127.0.0.1:8009"

    url = f"{target_base}/{path}"
    headers = dict(request.headers)
    headers.pop("host", None)

    try:
        response = await client.request(
            method=request.method,
            url=url,
            content=body,
            headers=headers,
        )
        return Response(
            content=response.content,
            status_code=response.status_code,
            headers=dict(response.headers),
        )
    except httpx.ConnectError:
        return Response(
            content=json.dumps({
                "error": {
                    "message": f"Backend unavailable for model '{model}' at {target_base}",
                    "type": "backend_unavailable",
                }
            }),
            status_code=503,
            media_type="application/json",
        )


# ─── OpenAI-compatible Proxy Endpoints ──────────────────────────────────────

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


# ─── Health & Discovery ─────────────────────────────────────────────────────

async def _check_backend(name: str, url: str) -> dict:
    """Probe a downstream backend and return its status."""
    try:
        resp = await client.get(f"{url}/health", timeout=3.0)
        alive = resp.status_code == 200
    except Exception:
        alive = False
    return {"name": name, "url": url, "healthy": alive}


@app.get("/health")
async def health():
    """Honest health check — probes all downstream backends."""
    # Check the critical backends in parallel
    checks = await asyncio.gather(
        _check_backend("nomic-embed (R)", "http://127.0.0.1:8005"),
        _check_backend("yardmaster (Y)", "http://127.0.0.1:8009"),
        _check_backend("flux-schnell (A)", "http://127.0.0.1:8004"),
        _check_backend("cogvideo (A)", "http://127.0.0.1:8006"),
        _check_backend("triposr (A)", "http://127.0.0.1:8007"),
        _check_backend("ace-step (T)", "http://127.0.0.1:8008"),
    )

    any_healthy = any(c["healthy"] for c in checks)
    healthy_count = sum(1 for c in checks if c["healthy"])

    return {
        "status": "ok" if any_healthy else "degraded",
        "hub": "A.R.T.Y.",
        "port": 8000,
        "backends": checks,
        "healthy_count": healthy_count,
        "total_count": len(checks),
    }


@app.get("/v1/models")
async def models():
    """Return list of models — only those whose backends are actually alive."""
    alive_models = []
    # Quick parallel health check of unique backend URLs
    unique_backends = {}
    for model_name, url in MODEL_ROUTING.items():
        if url not in unique_backends:
            unique_backends[url] = []
        unique_backends[url].append(model_name)

    checks = await asyncio.gather(
        *[_check_backend(url, url) for url in unique_backends.keys()]
    )

    alive_urls = {c["url"] for c in checks if c["healthy"]}

    for url, model_names in unique_backends.items():
        if url in alive_urls:
            for name in model_names:
                alive_models.append({"id": name, "object": "model"})

    return {"object": "list", "data": alive_models}


if __name__ == "__main__":
    print("═══════════════════════════════════════════════")
    print("  A.R.T.Y. Hub — Trinity ID AI OS")
    print("  Port 8000 — Reverse proxy for vLLM backends")
    print("═══════════════════════════════════════════════")
    uvicorn.run(app, host="127.0.0.1", port=8000)
