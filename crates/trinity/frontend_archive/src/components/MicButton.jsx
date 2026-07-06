import { useState, useRef, useCallback } from 'react';

/**
 * MicButton — Reusable speech-to-text microphone button for Trinity.
 *
 * Drop into any chat input row. Records audio from the browser mic,
 * sends it to POST /api/stt/transcribe, and calls onTranscript(text)
 * so the parent can insert it into the chat input.
 *
 * Props:
 *   onTranscript(text)  — called with transcribed text
 *   disabled            — disable mic when chat is streaming
 *   className           — optional extra CSS class
 */
export default function MicButton({ onTranscript, disabled = false, className = '' }) {
  const [recording, setRecording] = useState(false);
  const [processing, setProcessing] = useState(false);
  const mediaRecorderRef = useRef(null);
  const chunksRef = useRef([]);

  const startRecording = useCallback(async () => {
    try {
      const stream = await navigator.mediaDevices.getUserMedia({
        audio: { channelCount: 1, sampleRate: 16000 }
      });

      const mediaRecorder = new MediaRecorder(stream, {
        mimeType: MediaRecorder.isTypeSupported('audio/webm;codecs=opus')
          ? 'audio/webm;codecs=opus'
          : 'audio/webm'
      });

      chunksRef.current = [];

      mediaRecorder.ondataavailable = (e) => {
        if (e.data.size > 0) chunksRef.current.push(e.data);
      };

      mediaRecorder.onstop = async () => {
        // Stop all tracks to release the mic
        stream.getTracks().forEach(t => t.stop());

        const blob = new Blob(chunksRef.current, { type: 'audio/webm' });
        if (blob.size < 100) return; // Too small, ignore

        setProcessing(true);

        try {
          // Convert webm to WAV using AudioContext (browser-native)
          const arrayBuffer = await blob.arrayBuffer();
          const audioCtx = new AudioContext({ sampleRate: 16000 });
          const decoded = await audioCtx.decodeAudioData(arrayBuffer);
          const pcmData = decoded.getChannelData(0); // mono

          // Encode as 16-bit WAV
          const wavBuffer = encodeWAV(pcmData, 16000);

          const response = await fetch('/api/stt/transcribe', {
            method: 'POST',
            headers: { 'Content-Type': 'audio/wav' },
            body: wavBuffer,
          });

          if (response.ok) {
            const data = await response.json();
            if (data.text && data.text.trim()) {
              onTranscript(data.text.trim());
            }
          } else {
            console.error('[MicButton] STT error:', response.status);
          }

          audioCtx.close();
        } catch (err) {
          console.error('[MicButton] Transcription failed:', err);
        } finally {
          setProcessing(false);
        }
      };

      mediaRecorderRef.current = mediaRecorder;
      mediaRecorder.start();
      setRecording(true);
    } catch (err) {
      console.error('[MicButton] Mic access denied:', err);
    }
  }, [onTranscript]);

  const stopRecording = useCallback(() => {
    if (mediaRecorderRef.current && mediaRecorderRef.current.state === 'recording') {
      mediaRecorderRef.current.stop();
      setRecording(false);
    }
  }, []);

  const handleClick = useCallback(() => {
    if (recording) {
      stopRecording();
    } else if (!processing && !disabled) {
      startRecording();
    }
  }, [recording, processing, disabled, startRecording, stopRecording]);

  const label = processing ? '⏳' : recording ? '⏹' : '🎤';
  const title = processing ? 'Transcribing...' : recording ? 'Stop recording' : 'Speak to Trinity';

  return (
    <button
      className={`mic-btn ${recording ? 'mic-btn--recording' : ''} ${processing ? 'mic-btn--processing' : ''} ${className}`}
      onClick={handleClick}
      disabled={disabled && !recording}
      title={title}
      aria-label={title}
      type="button"
    >
      <span className="mic-btn__icon">{label}</span>
      {recording && <span className="mic-btn__pulse" />}
    </button>
  );
}

/**
 * Encode PCM float32 samples as a 16-bit WAV ArrayBuffer.
 */
function encodeWAV(samples, sampleRate) {
  const buffer = new ArrayBuffer(44 + samples.length * 2);
  const view = new DataView(buffer);

  // RIFF header
  writeString(view, 0, 'RIFF');
  view.setUint32(4, 36 + samples.length * 2, true);
  writeString(view, 8, 'WAVE');

  // fmt chunk
  writeString(view, 12, 'fmt ');
  view.setUint32(16, 16, true);           // chunk size
  view.setUint16(20, 1, true);            // PCM format
  view.setUint16(22, 1, true);            // mono
  view.setUint32(24, sampleRate, true);    // sample rate
  view.setUint32(28, sampleRate * 2, true); // byte rate
  view.setUint16(32, 2, true);            // block align
  view.setUint16(34, 16, true);           // bits per sample

  // data chunk
  writeString(view, 36, 'data');
  view.setUint32(40, samples.length * 2, true);

  // Write samples
  let offset = 44;
  for (let i = 0; i < samples.length; i++) {
    const s = Math.max(-1, Math.min(1, samples[i]));
    view.setInt16(offset, s < 0 ? s * 0x8000 : s * 0x7FFF, true);
    offset += 2;
  }

  return buffer;
}

function writeString(view, offset, string) {
  for (let i = 0; i < string.length; i++) {
    view.setUint8(offset + i, string.charCodeAt(i));
  }
}
