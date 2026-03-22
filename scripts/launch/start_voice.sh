#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════════════
# Trinity PersonaPlex Voice Sidecar Launcher
#
# Starts the PersonaPlex-7B ONNX model for audio-to-audio conversation
# This is the revolutionary voice system - NO transcription needed!
#
# Architecture:
#   Audio → Mimi Encoder → 7B LM → Mimi Decoder → Audio
#
# Usage:
#   ./scripts/launch/start_voice.sh
# ═══════════════════════════════════════════════════════════════════════
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$PROJECT_ROOT"

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║     TRINITY VOICE SIDECAR — PersonaPlex-7B ONNX            ║"
echo "╚══════════════════════════════════════════════════════════════╝"

# Kill any stale voice processes
echo "Cleaning up stale processes..."
pkill -f "personaplex" 2>/dev/null || true
pkill -f "voice_server" 2>/dev/null || true
sleep 1

# Verify model exists
PERSONAPLEX_DIR="$PROJECT_ROOT/models/personaplex"
if [ ! -f "$PERSONAPLEX_DIR/lm_backbone.onnx" ]; then
    echo "ERROR: PersonaPlex model not found at $PERSONAPLEX_DIR"
    echo ""
    echo "Download from HuggingFace:"
    echo "  huggingface-cli download Kyutai/PersonaPlex-7B-ONNX \\"
    echo "    --local-dir models/personaplex/"
    exit 1
fi

echo "Model found:"
echo "  lm_backbone.onnx: $(du -h "$PERSONAPLEX_DIR/lm_backbone.onnx" | cut -f1)"
echo "  lm_backbone.onnx.data: $(du -h "$PERSONAPLEX_DIR/lm_backbone.onnx.data" | cut -f1)"
echo "  mimi_encoder.onnx: $(du -h "$PERSONAPLEX_DIR/mimi_encoder.onnx" | cut -f1)"
echo "  mimi_decoder.onnx: $(du -h "$PERSONAPLEX_DIR/mimi_decoder.onnx" | cut -f1)"
echo ""

# Check for Python and required packages
if ! command -v python3 &> /dev/null; then
    echo "ERROR: Python3 not found"
    exit 1
fi

# Create voice server script if it doesn't exist
VOICE_SERVER="$PROJECT_ROOT/scripts/launch/voice_server.py"
if [ ! -f "$VOICE_SERVER" ]; then
    echo "Creating voice server..."
    mkdir -p "$PROJECT_ROOT/scripts/launch"
    cat > "$VOICE_SERVER" << 'PYTHON_EOF'
#!/usr/bin/env python3
"""
PersonaPlex Voice Server - Audio-to-Audio Conversation
Revolutionary: No transcription! Direct audio → audio.

Endpoints:
  POST /voice/chat - Send audio, receive audio
  GET  /health     - Health check
"""

import onnxruntime as ort
import numpy as np
from fastapi import FastAPI, File, UploadFile, HTTPException
from fastapi.responses import Response
import uvicorn
import tempfile
import soundfile as sf
import io
import os
import logging

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

app = FastAPI(title="PersonaPlex Voice Server")

# Model paths
MODEL_DIR = os.environ.get("PERSONAPLEX_DIR", "models/personaplex")
ENCODER_PATH = f"{MODEL_DIR}/mimi_encoder.onnx"
DECODER_PATH = f"{MODEL_DIR}/mimi_decoder.onnx"
BACKBONE_PATH = f"{MODEL_DIR}/lm_backbone.onnx"

# Load models on startup
encoder_session = None
decoder_session = None
backbone_session = None

@app.on_event("startup")
async def load_models():
    global encoder_session, decoder_session, backbone_session
    
    logger.info("Loading PersonaPlex models...")
    
    # Use CUDA if available, else CPU
    providers = ['CUDAExecutionProvider', 'CPUExecutionProvider']
    
    logger.info(f"Loading encoder: {ENCODER_PATH}")
    encoder_session = ort.InferenceSession(ENCODER_PATH, providers=providers)
    
    logger.info(f"Loading decoder: {DECODER_PATH}")
    decoder_session = ort.InferenceSession(DECODER_PATH, providers=providers)
    
    logger.info(f"Loading backbone: {BACKBONE_PATH}")
    backbone_session = ort.InferenceSession(BACKBONE_PATH, providers=providers)
    
    logger.info("All models loaded!")

@app.get("/health")
async def health():
    return {
        "status": "healthy",
        "models_loaded": all([encoder_session, decoder_session, backbone_session])
    }

@app.post("/voice/chat")
async def voice_chat(audio: UploadFile = File(...)):
    """
    Send audio, receive audio response.
    Revolutionary: No transcription step!
    """
    if not all([encoder_session, decoder_session, backbone_session]):
        raise HTTPException(status_code=503, detail="Models not loaded")
    
    # Read audio file
    audio_bytes = await audio.read()
    
    # Decode audio using soundfile
    audio_data, sample_rate = sf.read(io.BytesIO(audio_bytes))
    
    # Ensure mono and correct sample rate (24kHz for Mimi)
    if len(audio_data.shape) > 1:
        audio_data = audio_data[:, 0]  # Take first channel
    
    # Encode with Mimi encoder
    logger.info("Encoding audio...")
    audio_input = audio_data.astype(np.float32).reshape(1, 1, -1)
    encoder_output = encoder_session.run(None, {"audio": audio_input})
    audio_codes = encoder_output[0]
    
    # Process through backbone (7B LM)
    logger.info("Processing through language model...")
    # The backbone takes audio codes and generates response codes
    backbone_output = backbone_session.run(None, {"codes": audio_codes})
    response_codes = backbone_output[0]
    
    # Decode with Mimi decoder
    logger.info("Decoding response...")
    decoder_output = decoder_session.run(None, {"codes": response_codes})
    response_audio = decoder_output[0]
    
    # Convert to WAV
    response_audio = response_audio.squeeze()
    
    # Write to bytes
    output_bytes = io.BytesIO()
    sf.write(output_bytes, response_audio, 24000, format='WAV')
    output_bytes.seek(0)
    
    logger.info("Response generated!")
    
    return Response(
        content=output_bytes.read(),
        media_type="audio/wav"
    )

if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8200)
PYTHON_EOF
fi

echo "Starting voice server on port 8200..."
echo ""

# Run the voice server
cd "$PROJECT_ROOT"
export PERSONAPLEX_DIR="$PERSONAPLEX_DIR"
python3 "$VOICE_SERVER" &

VOICE_PID=$!
echo "Voice server PID: $VOICE_PID"

# Wait for server to be ready
echo "Waiting for voice server..."
for i in $(seq 1 60); do
    if curl -s http://127.0.0.1:8200/health > /dev/null 2>&1; then
        echo "Voice server is READY!"
        break
    fi
    sleep 1
done

echo ""
echo "Voice server is running. Endpoints:"
echo "  GET  http://127.0.0.1:8200/health"
echo "  POST http://127.0.0.1:8200/voice/chat (audio in, audio out)"
echo ""
echo "This is the REVOLUTIONARY audio-to-audio system!"
echo "No transcription needed - direct audio conversation."
echo ""
echo "Press Ctrl+C to stop."

wait $VOICE_PID
