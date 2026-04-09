"""
ROCm Flash Attention Shim - drop-in replacement using PyTorch SDPA
"""
__version__ = "2.7.0"
print("🔧 FAKE FLASH_ATTN MODULE LOADED FROM DIRECTORY!")

import torch

def flash_attn_varlen_func(q, k, v, cu_seqlens_q, cu_seqlens_k,
                           max_seqlen_q, max_seqlen_k, **kwargs):
    """
    Drop-in replacement for flash_attn.flash_attn_varlen_func using PyTorch SDPA.
    """
    causal = kwargs.get("causal", False)
    dropout_p = kwargs.get("dropout_p", 0.0)
    batch_size = cu_seqlens_q.shape[0] - 1

    if batch_size == 1:
        # Single-sequence fast path
        q_4d = q.unsqueeze(0).transpose(1, 2)
        k_4d = k.unsqueeze(0).transpose(1, 2)
        v_4d = v.unsqueeze(0).transpose(1, 2)
        out = torch.nn.functional.scaled_dot_product_attention(
            q_4d, k_4d, v_4d, is_causal=causal, dropout_p=dropout_p
        )
        return out.transpose(1, 2).squeeze(0)
    else:
        # Multi-sequence
        outputs = []
        for i in range(batch_size):
            start_q = cu_seqlens_q[i].item()
            end_q = cu_seqlens_q[i + 1].item()
            start_k = cu_seqlens_k[i].item()
            end_k = cu_seqlens_k[i + 1].item()

            qi = q[start_q:end_q].unsqueeze(0).transpose(1, 2)
            ki = k[start_k:end_k].unsqueeze(0).transpose(1, 2)
            vi = v[start_k:end_k].unsqueeze(0).transpose(1, 2)

            oi = torch.nn.functional.scaled_dot_product_attention(
                qi, ki, vi, is_causal=causal, dropout_p=dropout_p
            )
            outputs.append(oi.transpose(1, 2).squeeze(0))
        return torch.cat(outputs, dim=0)

def flash_attn_func(q, k, v, **kwargs):
    """Simple flash_attn_func stub using SDPA."""
    causal = kwargs.get("causal", False)
    dropout_p = kwargs.get("dropout_p", 0.0)
    out = torch.nn.functional.scaled_dot_product_attention(
        q.transpose(1, 2), k.transpose(1, 2), v.transpose(1, 2),
        is_causal=causal, dropout_p=dropout_p
    )
    return out.transpose(1, 2)
