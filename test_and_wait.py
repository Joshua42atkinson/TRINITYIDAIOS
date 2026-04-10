import time
import requests
import json
import subprocess
import sys

# Start the server
process = subprocess.Popen(["bash", "launch_amd_sidecar.sh"], cwd="/home/joshua/Workflow/desktop_trinity/trinity-genesis/longcat_amd_sidecar", stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)

print("Started server, waiting for model to load (can take ~2-3 minutes)..")

loaded = False
start_time = time.time()
while time.time() - start_time < 300: # 5 min timeout
    try:
        resp = requests.get("http://127.0.0.1:8010/health")
        if resp.status_code == 200:
            data = resp.json()
            if data.get("loaded") == True:
                loaded = True
                print("\n✅ Model loaded successfully!")
                break
    except:
        pass
    print(".", end="", flush=True)
    time.sleep(5)

if not loaded:
    print("\n❌ Failed to load within 5 minutes.")
    process.terminate()
    sys.exit(1)

print("Running test generation...")
try:
    resp = requests.post("http://127.0.0.1:8010/v1/chat/completions", json={
        "messages": [{"role": "user", "content": "Hello, LongCat!"}],
        "max_tokens": 50,
        "temperature": 0.7
    }, timeout=60)
    print(json.dumps(resp.json(), indent=2))
    print("✅ Generation successful!")
except Exception as e:
    print(f"❌ Generation failed: {e}")

process.terminate()
print("Killed server. Done.")
