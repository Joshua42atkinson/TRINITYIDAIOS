from fastapi import FastAPI
import uvicorn
import base64
from pydantic import BaseModel
import time

app = FastAPI()

class RequestModel(BaseModel):
    prompt: str
    n: int = 1
    size: str = "1024x1024"
    response_format: str = "b64_json"

@app.post("/v1/images/generations")
def generate(req: RequestModel):
    time.sleep(2) # Simulate processing time
    with open("/home/joshua/.gemini/antigravity/brain/56acf803-0af8-4eac-8131-b34968a99672/trinity_iron_road_1775392403241.png", "rb") as f:
        img_b64 = base64.b64encode(f.read()).decode("utf-8")
    
    return {
        "created": int(time.time()),
        "data": [{"b64_json": img_b64}]
    }

if __name__ == "__main__":
    uvicorn.run(app, host="127.0.0.1", port=8004)
