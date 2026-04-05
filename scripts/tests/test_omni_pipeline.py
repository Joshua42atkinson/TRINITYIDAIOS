import requests
import time
import json
import socket
import sys

NODES = {
    "Router": {"port": 8000, "type": "chat"},
    "Great_Recycler_31B": {"port": 8001, "type": "chat", "model": "Great_Recycler"},
    "Programmer_Pete_26B": {"port": 8002, "type": "chat", "model": "Programmer_Pete"},
    "Tempo_Engine_E4B": {"port": 8003, "type": "chat", "model": "Tempo_Engine"},
    "Embeddings_Node": {"port": 8005, "type": "embed", "model": "nomic-embed"},
}

def print_header(title):
    print(f"\n{'='*50}\n🔹 {title}\n{'='*50}")

def check_socket(port):
    """Check if the port is even listening on 127.0.0.1"""
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.settimeout(1)
    result = sock.connect_ex(('127.0.0.1', port))
    sock.close()
    return result == 0

def test_chat_inference(port, model_name):
    url = f"http://127.0.0.1:{port}/v1/chat/completions"
    payload = {
        "model": model_name,
        "messages": [{"role": "user", "content": "Hello. Testing logic gate. Respond with exactly one word: 'ONLINE'."}],
        "max_tokens": 10,
        "stream": False
    }
    
    start_time = time.time()
    try:
        response = requests.post(url, json=payload, timeout=20)
        ttft = time.time() - start_time
        
        if response.status_code == 200:
            data = response.json()
            content = data['choices'][0]['message']['content'].strip()
            print(f"✅ PASS | '{model_name}' Responded: {content} | TTFT: {ttft:.2f}s")
            return True
        else:
            print(f"❌ FAIL | HTTP {response.status_code} | {response.text}")
            return False
    except requests.exceptions.Timeout:
        print(f"❌ FAIL | Connection timed out after 20s. Node may be hung.")
        return False
    except requests.exceptions.ConnectionError:
        print(f"❌ FAIL | Connection refused. Service is not running or port is blocked.")
        return False

def run_diagnostics():
    print_header("TRINITY AI OS - OMNI PIPELINE DIAGNOSTICS")
    
    overall_health = True
    
    print("\nPhase 1: TCP Port Binding Checks (127.0.0.1)")
    for name, info in NODES.items():
        if check_socket(info['port']):
            print(f"✅ Port {info['port']} ({name}) is OPEN")
        else:
            print(f"❌ Port {info['port']} ({name}) is CLOSED - Host network cannot reach the Distrobox/vLLM!")
            overall_health = False

    if not overall_health:
        print("\n⚠️ FATAL ERROR: TCP Ports are not bound to 127.0.0.1. Verify firewall, Distrobox networking, or server crash states.")
        sys.exit(1)

    print("\nPhase 2: Local Node Inference Tests")
    for name, info in NODES.items():
        if info['type'] == 'chat' and 'model' in info:
            print(f"> Testing {name}...")
            if not test_chat_inference(info['port'], info['model']):
                overall_health = False

    print_header("TEST SUMMARY")
    if overall_health:
        print("🟢 SYSTEM GREEN: ALL NODES ONLINE AND RESPONDING.")
    else:
        print("🔴 SYSTEM RED: PIPELINE IS FRACTURED. CHECK LOGS.")
        sys.exit(1)

if __name__ == "__main__":
    run_diagnostics()
