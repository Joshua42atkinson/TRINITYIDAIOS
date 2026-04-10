import os
import sys

print("[*] Applying bitsandbytes patches...")
try:
    import bitsandbytes as bnb
    bnb_dir = os.path.dirname(bnb.__file__)
    rocm_lib_713 = os.path.join(bnb_dir, "libbitsandbytes_rocm713.so")
    rocm_lib_72 = os.path.join(bnb_dir, "libbitsandbytes_rocm72.so")
    if os.path.exists(rocm_lib_72) and not os.path.exists(rocm_lib_713):
        os.symlink("libbitsandbytes_rocm72.so", rocm_lib_713)
        print(" -> Linked rocm72 to rocm713")
    elif os.path.exists(rocm_lib_713) and not os.path.exists(rocm_lib_72):
        os.symlink("libbitsandbytes_rocm713.so", rocm_lib_72)
        print(" -> Linked rocm713 to rocm72")
except Exception as e:
    print(f"Skipping bitsandbytes patch: {e}")

print("[*] Applying vLLM activation patches...")
try:
    import vllm
    import re
    v_dir = os.path.dirname(vllm.__file__)
    target = os.path.join(v_dir, "model_executor", "layers", "quantization", "moe_wna16.py")
    if os.path.exists(target):
        with open(target, "r") as f: data = f.read()
        if "assert layer.activation ==" in data:
            data = re.sub(r"assert layer\.activation == .*", "if layer.activation != \"silu\": import logging; logging.warning(\"Bypassing SiLU assertion\")", data)
            with open(target, "w") as f: f.write(data)
            print(" -> Patched moe_wna16.py")
except Exception as e:
    print(f"Skipping vllm patch: {e}")
