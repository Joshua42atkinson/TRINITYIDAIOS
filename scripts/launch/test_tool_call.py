import requests
import json
import sys

PORT = 8000
URL = f"http://127.0.0.1:{PORT}/v1/chat/completions"

# A simple mock tool definition matching OpenAI spec
tools = [
    {
        "type": "function",
        "function": {
            "name": "generate_image",
            "description": "Generate an image based on a prompt.",
            "parameters": {
                "type": "object",
                "properties": {
                    "prompt": {
                        "type": "string",
                        "description": "The detailed prompt describing the image to generate."
                    }
                },
                "required": ["prompt"]
            }
        }
    }
]

payload = {
    "model": "Pete_Coder_26B",
    "messages": [
        {"role": "user", "content": "Please generate a picture of a futuristic cyberpunk city with neon lights."}
    ],
    "tools": tools,
    "temperature": 0.1
}

headers = {"Content-Type": "application/json"}

print(f"Sending request to {URL}...")
try:
    response = requests.post(URL, json=payload, headers=headers)
    response.raise_for_status()
    data = response.json()
    
    print("\n--- RESPONSE JSON ---")
    print(json.dumps(data, indent=2))
    
    choice = data["choices"][0]
    message = choice.get("message", {})
    tool_calls = message.get("tool_calls")
    
    print("\n--- TOOL CALL PARSING ---")
    if tool_calls:
        print("✅ Success: Tool call detected!")
        for tc in tool_calls:
            print(f"Name: {tc['function']['name']}")
            print(f"Arguments: {tc['function']['arguments']}")
    else:
        print("❌ Failed: No tool_calls found. Model output raw text:")
        print(message.get("content", ""))
        
except Exception as e:
    print(f"Error: {e}")
    if 'response' in locals():
        print(response.text)
