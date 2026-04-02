---
base_model:
- nvidia/NVIDIA-Nemotron-3-Super-120B-A12B-BF16
---

## Updates
### 03/12/2026
I uploaded the wrong splits for Q4_K_M / Q5_K_M and have corrected that now with the changes mentioned in the 03/11 update. Also added an IQ3_S quant now that there is a PR from @bartowski to fix the IQ4_NL quantization crash.

### 03/11/2026
I've adjusted the Q4_K_M and Q5_K_M to use Q5_0 for the `ffn_down_exps` tensor, which brings the Q5_K_M quant size down substantially.

## Description

This repo contains specialized MoE-quants for NVIDIA-Nemotron-3-Super-120B-A12B-BF16. The idea being that given the huge size of the FFN tensors compared to the rest of the tensors in the model, it should be possible to achieve a better quality while keeping the overall size of the entire model smaller compared to a similar naive quantization. To that end, the quantization type default is kept in high quality and the FFN UP + FFN GATE tensors are quanted down along with the FFN DOWN tensors.

## Notes

This model is a little weird, architecturally. There isn't a `ffn_gate_exps` tensor in it, and the `ffn_down_exps` tensor has `2688` elements in it which means that it is not compatible with most Q*_K quantizations.

So you may notice that the `ffn_down_exps` here is a little odd, and producing an actual IQ3_S-sized quant like I normally do is tricky since the IQ4_NL quantization type is also not behaving well.

I've chosen to upload these 3 quants for now and hope that there will be some improvements soon.

| Quant  | Size                 | Mixture                 | PPL                 | 1-(Mean PPL(Q)/PPL(base)) | KLD                 |
| :----- | :------------------- | :---------------------- | :------------------ | :------------------------ | :------------------ |
| Q5_K_M | 80.27 GiB (5.71 BPW) | Q8_0 / Q5_K  / X / Q5_0 | 4.590127 ± 0.027865 | +0.0817% | 0.007533 ± 0.000042 |
| Q4_K_M | 73.70 GiB (5.25 BPW) | Q8_0 / Q4_K  / X / Q5_0 | 4.600659 ± 0.027947 | +0.3113% | 0.010532 ± 0.000072 |
| IQ4_XS | 63.45 GiB (4.52 BPW) | Q8_0 / IQ3_S / X / Q4_1 | 4.647848 ± 0.028308 | +1.3402% | 0.022996 ± 0.000191 |
| IQ3_S  | 52.66 GiB (3.75 BPW) | Q6_K / IQ2_S / X / IQ4_NL | 4.787999 ± 0.029268 | +4.3960% | 0.059260 ± 0.000528 |

![kld_graph](kld_data/01_kld_vs_filesize.png "Chart showing Pareto KLD analysis of quants")
![ppl_graph](kld_data/02_ppl_vs_filesize.png "Chart showing Pareto PPL analysis of quants")