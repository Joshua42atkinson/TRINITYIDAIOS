# Audio Pipeline Engineering Plan

Instead of PersonaPlex (which lacks tool calling and doesn't fit the compute needs), we will build an STT -> STT-Processing -> TTS loop directly using the NPU in the trinity headless server.

1. **Character Sheet Audio Prefs**: Add `AudioPreferences { genre, voice_id, music_flow_enabled, bg_music_genre }` to the Character Sheet / Iron Road state.
2. **NPU STT**: We already have STT models in `npu_audio.rs` using ONNX. We will use `cpal` to capture microphone input, stream it to `npu_audio.rs` for STT.
3. **Trinity ID AI OS Iron Road Audio UI**: The parsed text goes to the LitRPG DEV system (Trinity Main loop), which responds using the narrative tool calling.
4. **NPU TTS**: The text output goes back to `npu_audio.rs` (or piper) for TTS.
5. **Music Streaming**: A parallel thread/service using `rodio` to stream background music based on preferences.

Let's look at `npu_audio.rs` to see what is missing.
