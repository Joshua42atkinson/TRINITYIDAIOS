import time
import requests

start = time.time()
while time.time() - start < 300:
    try:
        res = requests.get('http://127.0.0.1:8010/health')
        if res.json().get('loaded'):
            print('Loaded!')
            x = requests.post('http://127.0.0.1:8010/v1/chat/completions', json={'messages': [{'role':'user', 'content':'hello'}], 'max_tokens':10})
            print(x.text)
            exit(0)
    except:
        pass
    time.sleep(5)
print('Timeout')
