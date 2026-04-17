#!/usr/bin/env python3
"""Install gptqmodel shim into the container's site-packages"""
import os

SITE = "/opt/venv/lib64/python3.12/site-packages"

dirs = [
    f"{SITE}/gptqmodel",
    f"{SITE}/gptqmodel/quantization",
    f"{SITE}/gptqmodel/quantization/awq",
    f"{SITE}/gptqmodel/quantization/awq/modules",
    f"{SITE}/gptqmodel/utils",
]
for d in dirs:
    os.makedirs(d, exist_ok=True)

files = {
    f"{SITE}/gptqmodel/__init__.py": '"""gptqmodel shim for AWQ"""',
    
    f"{SITE}/gptqmodel/quantization/__init__.py": """
from enum import Enum
class METHOD(Enum):
    AWQ = "awq"
    GPTQ = "gptq"
""",
    
    f"{SITE}/gptqmodel/quantization/awq/__init__.py": "",
    f"{SITE}/gptqmodel/quantization/awq/modules/__init__.py": "",
    
    f"{SITE}/gptqmodel/quantization/awq/modules/act.py": """
import torch
import torch.nn as nn

class ScaledActivation(nn.Module):
    def __init__(self, module, scales):
        super().__init__()
        self.act = module
        self.scales = nn.Parameter(scales.data)
    def forward(self, x):
        return self.act(x) / self.scales.view(1, 1, -1).to(x.device)
""",
    
    f"{SITE}/gptqmodel/utils/__init__.py": "",
    
    f"{SITE}/gptqmodel/utils/importer.py": """
import torch
import torch.nn as nn

class AWQLinearShim(nn.Module):
    def __init__(self, bits=4, sym=False, desc_act=False, group_size=128,
                 in_features=1, out_features=1, bias=False, dev="cpu",
                 register_buffers=True, **kwargs):
        super().__init__()
        self.in_features = in_features
        self.out_features = out_features
        self.w_bit = bits
        self.group_size = group_size
        pack_factor = 32 // bits
        self.register_buffer("qweight", torch.zeros((in_features, out_features // pack_factor), dtype=torch.int32, device=dev))
        self.register_buffer("qzeros", torch.zeros((in_features // group_size, out_features // pack_factor), dtype=torch.int32, device=dev))
        self.register_buffer("scales", torch.zeros((in_features // group_size, out_features), dtype=torch.float16, device=dev))
        if bias:
            self.register_buffer("bias", torch.zeros(out_features, dtype=torch.float16, device=dev))
        else:
            self.bias = None
    def forward(self, x):
        raise NotImplementedError("Forward should use loaded AWQ weights")

def hf_select_quant_linear_v2(bits, group_size, desc_act, sym, format=None,
                                backend=None, device_map=None, quant_method=None,
                                zero_point=None, pack=False, **kwargs):
    return AWQLinearShim
""",
    
    f"{SITE}/gptqmodel/utils/model.py": """
def hf_gptqmodel_post_init(model, use_act_order=False):
    pass
""",
}

for path, content in files.items():
    with open(path, "w") as f:
        f.write(content)

print("gptqmodel shim installed successfully")
