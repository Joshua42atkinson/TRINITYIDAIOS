import requests
import time
import json

SGLANG_API_URL = "http://127.0.0.1:8011/v1/chat/completions"

# For LongCat-Next, predicting an image requires a dense sequence of visual tokens.
# We explicitly request an image to force the model to spin up the token sequence.
MESSAGE_PAYLOAD = {
    "model": "default",
    "messages": [
        {"role": "user", "content": "Generate a high-resolution image of a majestic cat sitting on a futuristic server rack."}
    ],
    "max_tokens": 1024,
    "temperature": 0.5,
    "stream": False
}

print("=================================================")
print(" SGLang Offline Image Generation Test")
print(f" Target: {SGLANG_API_URL}")
print("=================================================")
print("\nInitiating token sequence generation...")

try:
    start_time = time.time()
    
    response = requests.post(
        SGLANG_API_URL, 
        headers={"Content-Type": "application/json"},
        json=MESSAGE_PAYLOAD
    )
    
    end_time = time.time()
    total_time = end_time - start_time
    
    if response.status_code == 200:
        data = response.json()
        generation = data['choices'][0]['message']['content']
        tokens_used = data['usage']['completion_tokens']
        
        print("\n✅ Generation Complete!")
        print(f"Latency: {total_time:.2f} seconds")
        print(f"Tokens Generated: {tokens_used}")
        print(f"Tokens/Sec: {tokens_used / total_time:.2f}")
        print("\n--- Output Preview ---")
        print(generation[:300] + "..." if len(generation) > 300 else generation)
        print("----------------------")
        
        if "<|image|>" in generation or "[IMAGE" in generation:
            print("\n🔍 Visual decode tokens successfully resolved in output string.")
    else:
        print(f"❌ API Error: {response.status_code}")
        print(response.text)
        
except Exception as e:
    print(f"\n❌ Critical Request Failure: {e}")
    print("Ensure the SGLang sidecar (launch_sglang.sh) is fully booted and listening on Port 8011.")
