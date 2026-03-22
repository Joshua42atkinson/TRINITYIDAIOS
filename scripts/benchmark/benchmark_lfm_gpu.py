#!/usr/bin/env python3
"""
LFM2.5-Audio-1.5B GPU Benchmark for Trinity
AMD Ryzen AI Max+ 395 "Strix Halo" - ROCm GPU Path

Benchmarks Time-To-First-Audio (TTFA) and throughput using
the liquid-audio pip package with PyTorch/ROCm.
"""

import os
import time
import torch
import numpy as np
from datetime import datetime

# Set ROCm environment before imports
os.environ["HSA_OVERRIDE_GFX_VERSION"] = "11.0.0"
os.environ["HIP_VISIBLE_DEVICES"] = "0"

def benchmark_gpu_path():
    """Run GPU benchmark for LFM2.5-Audio-1.5B"""
    
    print("=" * 60)
    print("LFM2.5-Audio-1.5B GPU Benchmark (ROCm)")
    print("=" * 60)
    
    # Check GPU availability
    print(f"\nPyTorch version: {torch.__version__}")
    print(f"CUDA available: {torch.cuda.is_available()}")
    if torch.cuda.is_available():
        print(f"Device count: {torch.cuda.device_count()}")
        print(f"Device name: {torch.cuda.get_device_name(0)}")
        print(f"Device memory: {torch.cuda.get_device_properties(0).total_memory / 1e9:.1f} GB")
    
    device = "cuda" if torch.cuda.is_available() else "cpu"
    print(f"\nUsing device: {device}")
    
    # Import liquid-audio
    print("\nLoading LFM2.5-Audio-1.5B model...")
    from liquid_audio import LFM2AudioModel, LFM2AudioProcessor, ChatState
    
    load_start = time.time()
    
    # Load processor and model
    processor = LFM2AudioProcessor.from_pretrained("LiquidAI/LFM2.5-Audio-1.5B")
    model = LFM2AudioModel.from_pretrained("LiquidAI/LFM2.5-Audio-1.5B").eval()
    
    # Move to GPU
    model = model.to(device)
    processor = processor.to(device)
    
    load_time = time.time() - load_start
    print(f"Model load time: {load_time:.2f}s")
    
    # Get model size
    param_count = sum(p.numel() for p in model.parameters()) / 1e9
    print(f"Model parameters: {param_count:.2f}B")
    
    # Memory usage
    if device == "cuda":
        allocated = torch.cuda.memory_allocated() / 1e9
        reserved = torch.cuda.memory_reserved() / 1e9
        print(f"GPU memory allocated: {allocated:.2f} GB")
        print(f"GPU memory reserved: {reserved:.2f} GB")
    
    # Create test audio (1 second of silence at 24kHz)
    print("\nCreating test audio input...")
    sample_rate = 24000
    duration = 1.0
    silence = torch.zeros(int(sample_rate * duration), dtype=torch.float32, device=device)
    
    # Initialize chat state
    chat = ChatState(processor)
    
    # Setup conversation turn
    chat.new_turn("system")
    chat.add_text("Respond with interleaved text and audio.")
    chat.end_turn()
    
    chat.new_turn("user")
    chat.add_audio(silence, sample_rate)
    chat.end_turn()
    
    chat.new_turn("assistant")
    
    # Benchmark generation
    print("\nRunning interleaved generation benchmark...")
    print("-" * 40)
    
    gen_start = time.time()
    first_audio_time = None
    token_count = 0
    audio_chunks = 0
    
    with torch.no_grad(), processor.mimi.streaming(1):
        for token in model.generate_interleaved(
            **chat,
            max_new_tokens=256,
            audio_temperature=1.0,
            audio_top_k=4,
        ):
            token_count += 1
            
            # Track TTFA (first audio token has 8 entries)
            if token.numel() == 8:
                if first_audio_time is None:
                    first_audio_time = time.time() - gen_start
                    print(f"Time to first audio token: {first_audio_time:.3f}s")
                audio_chunks += 1
            
            # Decode audio for output
            if token.numel() == 1920:  # Decoded audio chunk
                pass  # Audio would be streamed here
    
    total_gen_time = time.time() - gen_start
    
    print("-" * 40)
    print(f"\n{'='*60}")
    print("GPU Path Results")
    print("=" * 60)
    print(f"  Total generation time: {total_gen_time:.2f}s")
    print(f"  Time to first audio: {first_audio_time:.3f}s" if first_audio_time else "  No audio generated")
    print(f"  Total tokens: {token_count}")
    print(f"  Audio chunks: {audio_chunks}")
    print(f"  Tokens/second: {token_count / total_gen_time:.1f}")
    
    # Memory footprint
    if device == "cuda":
        final_allocated = torch.cuda.memory_allocated() / 1e9
        final_reserved = torch.cuda.memory_reserved() / 1e9
        print(f"\n  Final GPU memory allocated: {final_allocated:.2f} GB")
        print(f"  Final GPU memory reserved: {final_reserved:.2f} GB")
        print(f"  Available for Mistral Small 4: ~{128 - final_reserved:.1f} GB")
    
    print("\n" + "=" * 60)
    print("GPU Path Complete")
    print("=" * 60)
    
    return {
        "load_time": load_time,
        "gen_time": total_gen_time,
        "ttfa": first_audio_time,
        "tokens": token_count,
        "memory_gb": final_allocated if device == "cuda" else 0,
    }


if __name__ == "__main__":
    benchmark_gpu_path()
