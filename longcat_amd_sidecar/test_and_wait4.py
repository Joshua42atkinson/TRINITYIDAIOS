import time
import requests
import json
import sys

print("Polling http://127.0.0.1:8010/health until loaded is True...")

start = time.time()
while time.time() - start < 300:
    try:
        res = requests.get('http://127.0.0.1:8010/health', timeout=5)
        if res.json().get('loaded'):
            print('✅ Loaded!')
            print(res.json())
            
            # Now test completion
            print("\nTesting Completion...")
            x = requests.post('http://127.0.0.1:8010/v1/chat/completions', json={'messages': [{'role':'user', 'content':'hello'}], 'max_tokens':50})
            print(json.dumps(x.json(), indent=2))
            
            if 'Internal Server Error' not in x.text and 'error' not in x.json():
                print("🏁 ALL SUCCESS")
                sys.exit(0)
            else:
                print("❌ FAILED")
                sys.exit(1)
    except:
        pass
    print(".", end="", flush=True)
    time.sleep(5)
print('TIMEOUT or SERVER NOT UP')
sys.exit(1)
