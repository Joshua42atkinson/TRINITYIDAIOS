#!/bin/bash
# Investigate NVIDIA Audio Model
echo "🔍 Investigating NVIDIA Audio Model: ng.pt"
echo "=========================================="

MODEL_PATH="models-consolidated/specialized/ng.pt"

if [ ! -f "$MODEL_PATH" ]; then
    echo "❌ Model not found at: $MODEL_PATH"
    exit 1
fi

echo "📁 Model Information:"
ls -lh "$MODEL_PATH"

echo ""
echo "🔧 File Analysis:"
file "$MODEL_PATH"

echo ""
echo "📊 Model Size:"
du -sh "$MODEL_PATH"

echo ""
echo "🧠 PyTorch Investigation:"
python3 << 'PYTHON'
import torch
import sys
import os

model_path = "models-consolidated/specialized/ng.pt"

try:
    print("🔍 Loading PyTorch model...")
    model = torch.load(model_path, map_location='cpu')
    
    print("✅ Model loaded successfully!")
    print(f"📋 Model type: {type(model)}")
    
    if isinstance(model, dict):
        print(f"🔑 Dictionary keys: {list(model.keys())}")
        
        # Check for common model components
        common_keys = ['state_dict', 'model', 'config', 'params', 'weight', 'bias']
        found_keys = [k for k in common_keys if k in model.keys()]
        if found_keys:
            print(f"🎯 Found common keys: {found_keys}")
        
        # Analyze each key
        for key, value in model.items():
            print(f"\n📦 Key: {key}")
            print(f"   Type: {type(value)}")
            
            if hasattr(value, 'shape'):
                print(f"   Shape: {value.shape}")
                print(f"   Dtype: {value.dtype}")
                total_params = value.numel() if hasattr(value, 'numel') else 'N/A'
                print(f"   Parameters: {total_params}")
            
            if isinstance(value, dict):
                print(f"   Sub-keys: {list(value.keys())[:10]}{'...' if len(value) > 10 else ''}")
                
                # Look for parameter counts
                for sub_key, sub_value in value.items():
                    if hasattr(sub_value, 'shape') and hasattr(sub_value, 'numel'):
                        if sub_key.lower() in ['weight', 'bias', 'embeddings']:
                            print(f"     {sub_key}: {sub_value.shape} ({sub_value.numel():,} params)")
    
    elif hasattr(model, 'state_dict'):
        print("🎯 Model with state_dict found")
        state_dict = model.state_dict()
        total_params = sum(p.numel() for p in state_dict.values())
        print(f"📊 Total parameters: {total_params:,}")
        
        # Check for audio-related layers
        audio_layers = [k for k in state_dict.keys() if any(word in k.lower() for word in ['audio', 'sound', 'wave', 'spectrogram', 'mel', 'encoder', 'decoder'])]
        if audio_layers:
            print(f"🎵 Audio-related layers found: {len(audio_layers)}")
            for layer in audio_layers[:5]:
                print(f"   {layer}: {state_dict[layer].shape}")
        
        # Check transformer layers
        transformer_layers = [k for k in state_dict.keys() if any(word in k.lower() for word in ['transformer', 'attention', 'ffn', 'layer'])]
        if transformer_layers:
            print(f"🤖 Transformer-related layers: {len(transformer_layers)}")
    
    else:
        print("🔍 Direct tensor or other format")
        if hasattr(model, 'shape'):
            print(f"   Shape: {model.shape}")
            print(f"   Dtype: {model.dtype}")
            if hasattr(model, 'numel'):
                print(f"   Elements: {model.numel():,}")
    
    # Try to estimate model size in parameters
    if isinstance(model, dict):
        total_params = 0
        for value in model.values():
            if hasattr(value, 'numel'):
                total_params += value.numel()
        
        if total_params > 0:
            print(f"\n📈 Estimated total parameters: {total_params:,}")
            
            # Estimate model size in billions
            params_billions = total_params / 1_000_000_000
            print(f"🎯 Model size: ~{params_billions:.1f}B parameters")
            
            if params_billions > 6 and params_billions < 8:
                print("🚀 This appears to be a ~7B model!")
            elif params_billions > 1:
                print(f"📊 This is a {params_billions:.1f}B parameter model")
        
        # Look for audio-specific indicators
        audio_indicators = []
        for key in model.keys():
            key_lower = key.lower()
            if any(word in key_lower for word in ['audio', 'sound', 'wave', 'spectrogram', 'mel', 'encoder', 'decoder', 'codec']):
                audio_indicators.append(key)
        
        if audio_indicators:
            print(f"🎵 Audio processing indicators found: {audio_indicators}")
        else:
            print("🔍 No obvious audio indicators in keys")
            
        # Look for NVIDIA-specific indicators
        nvidia_indicators = []
        for key in model.keys():
            key_lower = key.lower()
            if any(word in key_lower for word in ['nvidia', 'riva', 'nemo', 'waveglow', 'waveflow', 'hifigan']):
                nvidia_indicators.append(key)
        
        if nvidia_indicators:
            print(f"🏢 NVIDIA-specific indicators: {nvidia_indicators}")
        else:
            print("🔍 No obvious NVIDIA indicators in keys")

except Exception as e:
    print(f"❌ Error investigating model: {e}")
    print("🔧 Trying alternative methods...")
    
    # Try to get basic info without full loading
    try:
        import pickle
        with open(model_path, 'rb') as f:
            # Try to read just the first part
            f.seek(0)
            header = f.read(100)
            print(f"📋 File header (first 100 bytes): {header}")
    except Exception as e2:
        print(f"❌ Could not read file: {e2}")
PYTHON

echo ""
echo "🎯 Model Investigation Complete!"
echo ""
echo "📋 Summary:"
echo "   📁 Location: models-consolidated/specialized/ng.pt"
echo "   💾 Size: 1.8GB"
echo "   🔍 Format: PyTorch (.pt)"
echo ""
echo "🚀 Next Steps:"
echo "   1. Review the analysis above"
echo "   2. Determine if this is your 7B audio-to-audio model"
echo "   3. Integrate with AudioPersonalityPlex if confirmed"
echo "   4. Update voice integration pipeline"
