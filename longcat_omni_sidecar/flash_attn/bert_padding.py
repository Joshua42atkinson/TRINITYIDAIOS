import torch

def index_first_axis(x, indices):
    return x[indices]

def pad_input(hidden_states, indices, batch, seqlen):
    """Pad packed hidden states back to (batch, seqlen, dim)."""
    dim = hidden_states.shape[-1]
    output = torch.zeros(batch * seqlen, dim, device=hidden_states.device, dtype=hidden_states.dtype)
    output[indices] = hidden_states
    return output.view(batch, seqlen, dim)

def unpad_input(hidden_states, attention_mask):
    """Remove padding from hidden states."""
    seqlens = attention_mask.sum(dim=-1, dtype=torch.int32)
    indices = torch.nonzero(attention_mask.flatten(), as_tuple=False).flatten()
    max_seqlen = seqlens.max().item()
    cu_seqlens = torch.nn.functional.pad(
        torch.cumsum(seqlens, dim=0, dtype=torch.int32), (1, 0)
    )
    return hidden_states.view(-1, hidden_states.shape[-1])[indices], indices, cu_seqlens, max_seqlen
