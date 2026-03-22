---
base_model: Tesslate/OmniCoder-9B
tags:
  - llama-cpp
  - gguf
  - qwen3.5
  - omnicoder
  - tesslate
  - code
  - agent
license: apache-2.0
---

<div align="center">

<img src="https://huggingface.co/Tesslate/OmniCoder-9B/resolve/main/omnicoder-banner.png" alt="OmniCoder" width="720">

# OmniCoder-9B-GGUF

### GGUF quantizations of [OmniCoder-9B](https://huggingface.co/Tesslate/OmniCoder-9B)

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Full Weights](https://img.shields.io/badge/Full_Weights-OmniCoder--9B-purple)](https://huggingface.co/Tesslate/OmniCoder-9B)

</div>

---

## Available Quantizations

| Quantization | Size | Use Case |
|:---|---:|:---|
| `Q2_K` | ~3.8 GB | Extreme compression, lowest quality |
| `Q3_K_S` | ~4.3 GB | Small footprint |
| `Q3_K_M` | ~4.6 GB | Small footprint, balanced |
| `Q3_K_L` | ~4.9 GB | Small footprint, higher quality |
| `Q4_0` | ~5.3 GB | Good balance |
| `Q4_K_S` | ~5.4 GB | Good balance |
| **`Q4_K_M`** | **~5.7 GB** | **Recommended for most users** |
| `Q5_0` | ~6.3 GB | High quality |
| `Q5_K_S` | ~6.3 GB | High quality |
| `Q5_K_M` | ~6.5 GB | High quality, balanced |
| `Q6_K` | ~7.4 GB | Near-lossless |
| `Q8_0` | ~9.5 GB | Highest quality quantization |
| `BF16` | ~17.9 GB | Full precision |

## Usage

```bash
# Install llama.cpp
brew install llama.cpp  # macOS
# or build from source: https://github.com/ggml-org/llama.cpp

# Interactive chat
llama-cli --hf-repo Tesslate/OmniCoder-9B-GGUF --hf-file omnicoder-9b-q4_k_m.gguf -p "Your prompt" -c 8192

# Server mode (OpenAI-compatible API)
llama-server --hf-repo Tesslate/OmniCoder-9B-GGUF --hf-file omnicoder-9b-q4_k_m.gguf -c 8192
```

---

<div align="center">

**Built by [Tesslate](https://tesslate.com)** | See full model card: [OmniCoder-9B](https://huggingface.co/Tesslate/OmniCoder-9B)

</div>
