import os
from huggingface_hub import snapshot_download

repo_id = "Akicou/LLaDA2.1-mini-256k-dynamic-ntk"
local_dir = "/home/joshua/trinity-models/omni/LLaDA2.1-mini-256k"

print(f"Downloading {repo_id}...")
print(f"Target directory: {local_dir}")

os.makedirs(local_dir, exist_ok=True)

try:
    snapshot_download(
        repo_id=repo_id,
        local_dir=local_dir,
        local_dir_use_symlinks=False, 
        resume_download=True,
        max_workers=8
    )
    print("Download completed successfully!")
except Exception as e:
    print(f"Error during download: {e}")
