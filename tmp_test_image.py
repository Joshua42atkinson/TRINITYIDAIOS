import requests
import base64
import time

url = "http://127.0.0.1:8004/v1/images/generations"
payload = {
    "prompt": "a beautiful steampunk mechanical golem drinking tea, concept art, glowing green lights",
    "n": 1,
    "size": "1024x1024",
    "response_format": "b64_json"
}

try:
    print("Sending request to HunyuanImage on vLLM (port 8004)...")
    start = time.time()
    response = requests.post(url, json=payload, timeout=300)
    response.raise_for_status()
    data = response.json()
    b64 = data["data"][0]["b64_json"]
    
    with open("/home/joshua/Workflow/desktop_trinity/trinity-genesis/proof_of_quality.png", "wb") as f:
        f.write(base64.b64decode(b64))
        
    print(f"Success! Image saved to proof_of_quality.png in {time.time()-start:.2f}s")
except Exception as e:
    print(f"Failed: {e}")
