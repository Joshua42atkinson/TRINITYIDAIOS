# NPU Model Download Instructions
## Llama-3.2-1B ONNX for AMD XDNA 2 NPU

**Status**: Ready to download (requires git-lfs)

---

## Prerequisites

Install git-lfs:
```bash
sudo apt install git-lfs
git lfs install
```

---

## Download Model

```bash
cd ~/ai_models/onnx
GIT_LFS_SKIP_SMUDGE=0 git clone https://huggingface.co/amd/Llama-3.2-1B-Instruct-onnx-ryzenai-npu
```

**Expected size**: ~1GB  
**Model files**: `model.onnx`, `model.onnx.data`

---

## Verification

```bash
ls -lh ~/ai_models/onnx/Llama-3.2-1B-Instruct-onnx-ryzenai-npu/
```

Should show:
- model.onnx (~500MB)
- model.onnx.data (~500MB)
- config files

---

## Next Steps

After download, test with Python ORT script:
```bash
python scripts/test_ort_npu.py
```

---

**Note**: This model is optimized for AMD Ryzen AI NPU (XDNA 2) with Vitis AI execution provider.
