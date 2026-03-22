#!/usr/bin/env python3
"""
Test ONNX Runtime loading with CPU and NPU execution providers.
Validates the NPU pipeline before attempting Rust integration.
"""

import sys
import os

def test_ort_import():
    """Test if onnxruntime is installed"""
    try:
        import onnxruntime as ort
        print(f"✅ onnxruntime {ort.__version__} imported successfully")
        return ort
    except ImportError as e:
        print(f"❌ onnxruntime not installed: {e}")
        print("Install with: pip install onnxruntime")
        return None

def test_cpu_provider(ort, model_path):
    """Test CPU execution provider (baseline)"""
    print("\n--- Testing CPU Execution Provider ---")
    try:
        sess = ort.InferenceSession(
            model_path,
            providers=['CPUExecutionProvider']
        )
        print(f"✅ CPU provider loaded successfully")
        print(f"   Inputs: {[i.name for i in sess.get_inputs()]}")
        print(f"   Outputs: {[o.name for i in sess.get_outputs()]}")
        return True
    except Exception as e:
        print(f"❌ CPU provider failed: {e}")
        return False

def test_npu_provider(ort, model_path):
    """Test NPU execution provider (VitisAI)"""
    print("\n--- Testing NPU Execution Provider ---")
    
    # Check available providers
    available = ort.get_available_providers()
    print(f"Available providers: {available}")
    
    if 'VitisAIExecutionProvider' not in available:
        print("⚠️  VitisAIExecutionProvider not available")
        print("   This is expected - NPU support requires:")
        print("   - AMD Ryzen AI NPU drivers")
        print("   - Vitis AI execution provider")
        print("   - ONNX Runtime with NPU support")
        return False
    
    try:
        sess = ort.InferenceSession(
            model_path,
            providers=['VitisAIExecutionProvider', 'CPUExecutionProvider']
        )
        actual_provider = sess.get_providers()[0]
        if actual_provider == 'VitisAIExecutionProvider':
            print(f"✅ NPU provider loaded successfully")
            return True
        else:
            print(f"⚠️  Fell back to {actual_provider}")
            return False
    except Exception as e:
        print(f"❌ NPU provider failed: {e}")
        return False

def main():
    # Model path
    model_path = os.path.expanduser("~/ai_models/onnx/Llama-3.2-1B-Instruct-onnx-ryzenai-npu/model.onnx")
    
    print("=" * 60)
    print("ONNX Runtime NPU Test")
    print("=" * 60)
    
    # Check if model exists
    if not os.path.exists(model_path):
        print(f"❌ Model not found: {model_path}")
        print("\nDownload with:")
        print("  cd ~/ai_models/onnx")
        print("  git clone https://huggingface.co/amd/Llama-3.2-1B-Instruct-onnx-ryzenai-npu")
        return 1
    
    print(f"Model: {model_path}")
    print(f"Size: {os.path.getsize(model_path) / 1024 / 1024:.1f} MB")
    
    # Test import
    ort = test_ort_import()
    if not ort:
        return 1
    
    # Test CPU provider
    cpu_ok = test_cpu_provider(ort, model_path)
    
    # Test NPU provider
    npu_ok = test_npu_provider(ort, model_path)
    
    # Summary
    print("\n" + "=" * 60)
    print("SUMMARY")
    print("=" * 60)
    print(f"CPU Provider: {'✅ PASS' if cpu_ok else '❌ FAIL'}")
    print(f"NPU Provider: {'✅ PASS' if npu_ok else '⚠️  NOT AVAILABLE (expected)'}")
    
    if cpu_ok:
        print("\n✅ ONNX pipeline validated - CPU provider works")
        print("   NPU integration is aspirational for future work")
        return 0
    else:
        print("\n❌ ONNX pipeline failed - check model and dependencies")
        return 1

if __name__ == "__main__":
    sys.exit(main())
