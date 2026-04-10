#!/bin/bash
source /etc/profile.d/rocm-sdk.sh 2>/dev/null || true
source /etc/profile.d/01-rocm-env-for-triton.sh 2>/dev/null || true
export HSA_ENABLE_SDMA=0

/opt/venv/bin/python3 -c "
import torch
print(f'PyTorch: {torch.__version__}')
print(f'GPU: {torch.cuda.get_device_name(0)}')
print(f'Arch: {torch.cuda.get_device_properties(0).gcnArchName}')
t = torch.randn(4, 4, device='cuda', dtype=torch.bfloat16)
print(f'BF16 Tensor: OK ({t.sum().item():.4f})')
c = torch.mm(t, t.T)
print(f'MatMul: OK ({c.sum().item():.4f})')
print()
print('ALL GPU TESTS PASSED')
"
