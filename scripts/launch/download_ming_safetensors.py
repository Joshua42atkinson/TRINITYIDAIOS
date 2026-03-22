#!/usr/bin/env python3
"""
Ming-Flash-Omni 2.0 Safetensors Downloader

Because the Evo X2 (Strix Halo) has 128GB of RAM but 492GB of free NVMe space,
we can safely download the full 200GB FP16 model to disk. 

We will then use SGLang/vLLM's `load_in_4bit` or `--quantization bitsandbytes` 
flags to stream the safetensor chunks from disk into RAM, quantizing them 
on the fly. This bypasses the 128GB RAM limit while avoiding the need for 
a cloud node.
"""

import os
from huggingface_hub import snapshot_download

# Target directory on the NVMe drive
target_dir = os.path.expanduser("~/trinity-models/safetensors/Ming-flash-omni-2.0")
os.makedirs(target_dir, exist_ok=True)

print(f"Starting download of inclusionAI/Ming-flash-omni-2.0 to {target_dir}")
print("This is a ~200GB download. Grab a coffee.")

# We exclude PyTorch/bin files to save space, only pulling the safetensors.
try:
    snapshot_download(
        repo_id="inclusionAI/Ming-flash-omni-2.0",
        local_dir=target_dir,
        local_dir_use_symlinks=False,
        ignore_patterns=["*.pt", "*.bin"], # Force safetensors only
        resume_download=True,
        max_workers=8 # Speed up chunked downloading
    )
    print("Download Complete!")
except Exception as e:
    print(f"Error during download: {e}")
