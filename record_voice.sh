#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# TRINITY ID AI OS — Voice Recording Tool
# ═══════════════════════════════════════════════════════════════════════════════
#
# PURPOSE:  Record your voice to use as the LongCat-Next TTS audio clone reference.
#           Provides the audio clone data for LongCat-Next Omni-Brain.
#
# USAGE:    ./record_voice.sh [duration_in_seconds] [persona_name]
#           Defaults to 10 seconds and saves to system_audio.wav.
# ═══════════════════════════════════════════════════════════════════════════════


DURATION=${1:-10}
PERSONA=${2:-"system"}

if [ "$PERSONA" == "system" ]; then
    OUTPUT_FILE="$HOME/trinity-models/sglang/LongCat-Next/assets/system_audio.wav"
else
    # Save directly to voices folder inside LongCat sidecar assets
    mkdir -p "$HOME/Workflow/desktop_trinity/trinity-genesis/assets/voices"
    OUTPUT_FILE="$HOME/Workflow/desktop_trinity/trinity-genesis/assets/voices/${PERSONA}.wav"
fi

echo "🎙️ TRINITY ID AI OS — Voice Cloning Recorder"
echo "============================================="
echo "Recording duration : ${DURATION} seconds"
echo "Persona voice      : ${PERSONA}"
echo "Output file        : ${OUTPUT_FILE}"
echo ""
echo "Please speak continuously and clearly for the next ${DURATION} seconds."
echo ""
read -p "Press [Enter] when ready to start recording..."

echo "🔴 RECORDING NOW (Speak for ${DURATION}s)..."

# Ensure output directory exists
mkdir -p "$(dirname "$OUTPUT_FILE")"

# Record using ALSA/PulseAudio to 16kHz, mono, 16-bit PCM which is ideal for Whisper/CosyVoice
arecord -D default -f S16_LE -c 1 -r 16000 -d ${DURATION} "$OUTPUT_FILE" -q

if [ $? -eq 0 ]; then
    echo "✅ Recording complete!"
    echo "Saved to: $OUTPUT_FILE"
    echo ""
    echo "LongCat-Next will now use this reference for TTS generations when using the '${PERSONA}' persona."
else
    echo "❌ Recording failed! Make sure 'arecord' is installed and a microphone is connected."
fi
