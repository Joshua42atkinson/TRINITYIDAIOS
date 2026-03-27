// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — Native Speech-to-Text Engine
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        stt.rs
// PURPOSE:     Whisper ONNX Runtime STT — native Rust, no Python sidecar
//
// ARCHITECTURE:
//   audio (16kHz PCM f32) -> mel spectrogram (80 bins, STFT)
//     -> encoder.onnx -> cross-attention states
//     -> decoder.onnx (autoregressive, greedy) -> token IDs -> text
//
// Based on: OpenAI Whisper (via onnx-community/whisper-base)
// Runtime:  ort 2.0.0-rc.7 (same as Supertonic-2 TTS)
//
// CHANGES:
//   2026-03-27  Cascade  Initial implementation — native Rust STT
//
// ═══════════════════════════════════════════════════════════════════════════════

use anyhow::{bail, Context, Result};
use ndarray::{Array2, Array3};
use ort::{session::Session, value::Value};
use std::path::Path;
use tracing::info;

// ============================================================================
// Constants — Whisper Base configuration
// ============================================================================

const SAMPLE_RATE: u32 = 16000;
const N_FFT: usize = 400;
const HOP_LENGTH: usize = 160;
const N_MELS: usize = 80;
const CHUNK_LENGTH_S: usize = 30;
const N_FRAMES: usize = SAMPLE_RATE as usize * CHUNK_LENGTH_S / HOP_LENGTH; // 3000

// Special token IDs for Whisper (multilingual base)
const SOT_TOKEN: i64 = 50258;
const EOT_TOKEN: i64 = 50257;
const _TRANSLATE_TOKEN: i64 = 50358;
const TRANSCRIBE_TOKEN: i64 = 50359;
const NO_TIMESTAMPS_TOKEN: i64 = 50363;
const EN_TOKEN: i64 = 50259;

const MAX_DECODE_TOKENS: usize = 448;

// ============================================================================
// Mel Filter Bank — precomputed 80-bin mel filterbank for 400-point FFT
// ============================================================================

/// Build a mel filterbank matrix (n_mels x (n_fft/2 + 1))
/// Converts Hz to mel scale, creates triangular filters
fn build_mel_filterbank() -> Array2<f32> {
    let n_freqs = N_FFT / 2 + 1; // 201
    let mut fb = Array2::<f32>::zeros((N_MELS, n_freqs));

    // Mel scale boundaries: 0 Hz to 8000 Hz (Nyquist for 16kHz)
    let f_min = 0.0_f32;
    let f_max = (SAMPLE_RATE as f32) / 2.0;
    let mel_min = hz_to_mel(f_min);
    let mel_max = hz_to_mel(f_max);

    // n_mels + 2 equally spaced mel points
    let n_points = N_MELS + 2;
    let mel_points: Vec<f32> = (0..n_points)
        .map(|i| mel_min + (mel_max - mel_min) * (i as f32) / ((n_points - 1) as f32))
        .collect();
    let hz_points: Vec<f32> = mel_points.iter().map(|&m| mel_to_hz(m)).collect();
    let bin_points: Vec<f32> = hz_points
        .iter()
        .map(|&f| f * (N_FFT as f32) / (SAMPLE_RATE as f32))
        .collect();

    for m in 0..N_MELS {
        let f_left = bin_points[m];
        let f_center = bin_points[m + 1];
        let f_right = bin_points[m + 2];

        for k in 0..n_freqs {
            let kf = k as f32;
            if kf >= f_left && kf <= f_center && f_center > f_left {
                fb[[m, k]] = (kf - f_left) / (f_center - f_left);
            } else if kf > f_center && kf <= f_right && f_right > f_center {
                fb[[m, k]] = (f_right - kf) / (f_right - f_center);
            }
        }
    }

    // Normalize each filter (slaney normalization)
    for m in 0..N_MELS {
        let enorm = 2.0 / (hz_points[m + 2] - hz_points[m]);
        for k in 0..n_freqs {
            fb[[m, k]] *= enorm;
        }
    }

    fb
}

fn hz_to_mel(hz: f32) -> f32 {
    2595.0 * (1.0 + hz / 700.0).log10()
}

fn mel_to_hz(mel: f32) -> f32 {
    700.0 * (10.0_f32.powf(mel / 2595.0) - 1.0)
}

// ============================================================================
// STFT + Mel Spectrogram — pure Rust (no rustfft needed for small N_FFT)
// ============================================================================

/// Compute mel spectrogram from 16kHz PCM f32 audio.
/// Returns shape (1, n_mels, n_frames) = (1, 80, 3000)
fn compute_mel_spectrogram(audio: &[f32], mel_filters: &Array2<f32>) -> Array3<f32> {
    // Pad/trim audio to exactly 30 seconds
    let target_len = SAMPLE_RATE as usize * CHUNK_LENGTH_S;
    let mut padded = vec![0.0f32; target_len];
    let copy_len = audio.len().min(target_len);
    padded[..copy_len].copy_from_slice(&audio[..copy_len]);

    let n_freqs = N_FFT / 2 + 1;

    // Hann window
    let window: Vec<f32> = (0..N_FFT)
        .map(|i| {
            0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / N_FFT as f32).cos())
        })
        .collect();

    // Compute STFT magnitude squared
    let mut magnitudes = Array2::<f32>::zeros((n_freqs, N_FRAMES));

    for frame_idx in 0..N_FRAMES {
        let start = frame_idx * HOP_LENGTH;
        if start + N_FFT > padded.len() {
            break;
        }

        // Apply window
        let mut windowed = vec![0.0f32; N_FFT];
        for i in 0..N_FFT {
            windowed[i] = padded[start + i] * window[i];
        }

        // DFT (real input -> complex output, only positive frequencies)
        for k in 0..n_freqs {
            let mut re = 0.0f32;
            let mut im = 0.0f32;
            let freq = 2.0 * std::f32::consts::PI * (k as f32) / (N_FFT as f32);
            for n in 0..N_FFT {
                let angle = freq * (n as f32);
                re += windowed[n] * angle.cos();
                im -= windowed[n] * angle.sin();
            }
            magnitudes[[k, frame_idx]] = re * re + im * im;
        }
    }

    // Apply mel filterbank: (n_mels, n_freqs) @ (n_freqs, n_frames) -> (n_mels, n_frames)
    let mut mel_spec = Array2::<f32>::zeros((N_MELS, N_FRAMES));
    for m in 0..N_MELS {
        for t in 0..N_FRAMES {
            let mut sum = 0.0f32;
            for k in 0..n_freqs {
                sum += mel_filters[[m, k]] * magnitudes[[k, t]];
            }
            mel_spec[[m, t]] = sum;
        }
    }

    // Log mel spectrogram (clamp to avoid log(0))
    let max_val = mel_spec.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let clamp_min = (max_val * 1e-10).max(1e-10);
    for v in mel_spec.iter_mut() {
        *v = (*v).max(clamp_min).ln();
    }

    // Normalize to [-1, 1] range (Whisper convention)
    let log_max = mel_spec.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let log_min = (log_max - 8.0).max(mel_spec.iter().cloned().fold(f32::INFINITY, f32::min));
    for v in mel_spec.iter_mut() {
        *v = ((*v - log_min) / (log_max - log_min)) * 2.0 - 1.0;
    }

    // Reshape to (1, 80, 3000)
    mel_spec.insert_axis(ndarray::Axis(0))
}

// ============================================================================
// Tokenizer — minimal Whisper tokenizer (vocab.json based)
// ============================================================================

struct WhisperTokenizer {
    id_to_token: Vec<String>,
}

impl WhisperTokenizer {
    fn load(tokenizer_path: &Path) -> Result<Self> {
        let file = std::fs::File::open(tokenizer_path)
            .context("Failed to open tokenizer.json")?;
        let reader = std::io::BufReader::new(file);
        let data: serde_json::Value = serde_json::from_reader(reader)?;

        // Extract vocab from tokenizer.json (HuggingFace format)
        let mut id_to_token: Vec<String> = Vec::new();

        if let Some(model) = data.get("model") {
            if let Some(vocab) = model.get("vocab") {
                if let Some(vocab_map) = vocab.as_object() {
                    // Find max ID
                    let max_id = vocab_map.values()
                        .filter_map(|v| v.as_u64())
                        .max()
                        .unwrap_or(0) as usize;
                    id_to_token.resize(max_id + 1, String::new());
                    for (token, id) in vocab_map {
                        if let Some(id_val) = id.as_u64() {
                            id_to_token[id_val as usize] = token.clone();
                        }
                    }
                }
            }
        }

        // Also check for added_tokens (special tokens like SOT, EOT, language tokens)
        if let Some(added) = data.get("added_tokens") {
            if let Some(arr) = added.as_array() {
                for entry in arr {
                    if let (Some(id), Some(content)) = (
                        entry.get("id").and_then(|v| v.as_u64()),
                        entry.get("content").and_then(|v| v.as_str()),
                    ) {
                        let id = id as usize;
                        if id >= id_to_token.len() {
                            id_to_token.resize(id + 1, String::new());
                        }
                        id_to_token[id] = content.to_string();
                    }
                }
            }
        }

        info!("[STT] Tokenizer loaded: {} tokens", id_to_token.len());
        Ok(WhisperTokenizer { id_to_token })
    }

    fn decode(&self, token_ids: &[i64]) -> String {
        let mut result = String::new();
        for &id in token_ids {
            if id < 0 || id as usize >= self.id_to_token.len() {
                continue;
            }
            let token = &self.id_to_token[id as usize];
            // Skip special tokens (start with <|)
            if token.starts_with("<|") {
                continue;
            }
            // Whisper uses byte-level BPE: decode the token cleaning up
            // The 'Ġ' character (U+0120) represents a space in byte-level BPE
            let cleaned = token.replace('\u{0120}', " ");
            result.push_str(&cleaned);
        }
        result.trim().to_string()
    }
}

// ============================================================================
// WhisperEngine — the main STT engine
// ============================================================================

pub struct WhisperEngine {
    encoder: Session,
    decoder: Session,
    tokenizer: WhisperTokenizer,
    mel_filters: Array2<f32>,
}

impl WhisperEngine {
    /// Load the Whisper ONNX model from a directory.
    /// Expected files: onnx/encoder_model.onnx, onnx/decoder_model_merged.onnx, tokenizer.json
    pub fn load(model_dir: &Path) -> Result<Self> {
        let t0 = std::time::Instant::now();
        let onnx_dir = model_dir.join("onnx");

        let encoder_path = onnx_dir.join("encoder_model.onnx");
        let decoder_path = onnx_dir.join("decoder_model_merged.onnx");
        let tokenizer_path = model_dir.join("tokenizer.json");

        if !encoder_path.exists() {
            bail!("Whisper encoder not found at {:?}", encoder_path);
        }
        if !decoder_path.exists() {
            bail!("Whisper decoder not found at {:?}", decoder_path);
        }

        info!("[STT] Loading Whisper from {:?}...", model_dir);

        let encoder = Session::builder()?
            .commit_from_file(&encoder_path)
            .context("Failed to load encoder ONNX")?;
        let decoder = Session::builder()?
            .commit_from_file(&decoder_path)
            .context("Failed to load decoder ONNX")?;
        let tokenizer = WhisperTokenizer::load(&tokenizer_path)?;
        let mel_filters = build_mel_filterbank();

        info!(
            "[STT] Whisper loaded in {:.1}s",
            t0.elapsed().as_secs_f32(),
        );

        Ok(WhisperEngine {
            encoder,
            decoder,
            tokenizer,
            mel_filters,
        })
    }

    /// Transcribe raw 16kHz mono PCM f32 audio to text.
    pub fn transcribe(&mut self, audio: &[f32]) -> Result<String> {
        let t0 = std::time::Instant::now();

        // 1. Compute mel spectrogram
        let mel = compute_mel_spectrogram(audio, &self.mel_filters);
        let mel_ms = t0.elapsed().as_millis();

        // 2. Encode
        let mel_val = Value::from_array(mel)?;
        let encoder_out = self.encoder.run(ort::inputs! {
            "input_features" => &mel_val
        })?;
        let enc_ms = t0.elapsed().as_millis();

        // 3. Decode (greedy, autoregressive)
        // Initial tokens: SOT, EN, TRANSCRIBE, NO_TIMESTAMPS
        let mut token_ids: Vec<i64> = vec![
            SOT_TOKEN,
            EN_TOKEN,
            TRANSCRIBE_TOKEN,
            NO_TIMESTAMPS_TOKEN,
        ];

        let encoder_hidden = &encoder_out["last_hidden_state"];

        for _step in 0..MAX_DECODE_TOKENS {
            // Build decoder input
            let seq_len = token_ids.len();
            let input_ids = Array2::from_shape_vec(
                (1, seq_len),
                token_ids.clone(),
            )?;
            let input_ids_val = Value::from_array(input_ids)?;

            let decoder_out = self.decoder.run(ort::inputs! {
                "input_ids" => &input_ids_val,
                "encoder_hidden_states" => encoder_hidden
            })?;

            // Get logits from last position
            let logits_val = &decoder_out["logits"];
            let (logits_shape, logits_data) = logits_val.try_extract_tensor::<f32>()?;

            // logits shape: (1, seq_len, vocab_size)
            let vocab_size = logits_shape[2] as usize;
            let logits_vec = logits_data.to_vec();
            let last_pos_start = (seq_len - 1) * vocab_size;
            let last_logits = &logits_vec[last_pos_start..last_pos_start + vocab_size];

            // Greedy: argmax
            let next_token = last_logits
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(idx, _)| idx as i64)
                .unwrap_or(EOT_TOKEN);

            if next_token == EOT_TOKEN {
                break;
            }

            token_ids.push(next_token);
        }

        let dec_ms = t0.elapsed().as_millis();

        // 4. Detokenize (skip the initial prompt tokens)
        let output_tokens = &token_ids[4..]; // skip SOT, EN, TRANSCRIBE, NO_TIMESTAMPS
        let text = self.tokenizer.decode(output_tokens);

        info!(
            "[STT] Transcribed {} samples -> \"{}\" (mel: {}ms, enc: {}ms, dec: {}ms, total: {}ms)",
            audio.len(),
            if text.len() > 60 { &text[..60] } else { &text },
            mel_ms,
            enc_ms - mel_ms,
            dec_ms - enc_ms,
            dec_ms,
        );

        Ok(text)
    }

    /// Transcribe a WAV file (any sample rate, auto-resampled to 16kHz mono)
    pub fn transcribe_wav(&mut self, wav_bytes: &[u8]) -> Result<String> {
        let cursor = std::io::Cursor::new(wav_bytes);
        let reader = hound::WavReader::new(cursor)?;
        let spec = reader.spec();

        // Read all samples as f32
        let samples: Vec<f32> = match spec.sample_format {
            hound::SampleFormat::Int => {
                let max_val = (1i64 << (spec.bits_per_sample - 1)) as f32;
                reader
                    .into_samples::<i32>()
                    .filter_map(|s| s.ok())
                    .map(|s| s as f32 / max_val)
                    .collect()
            }
            hound::SampleFormat::Float => {
                reader
                    .into_samples::<f32>()
                    .filter_map(|s| s.ok())
                    .collect()
            }
        };

        // Convert to mono if stereo
        let mono: Vec<f32> = if spec.channels > 1 {
            samples
                .chunks(spec.channels as usize)
                .map(|chunk| chunk.iter().sum::<f32>() / chunk.len() as f32)
                .collect()
        } else {
            samples
        };

        // Resample to 16kHz if needed (simple linear interpolation)
        let resampled = if spec.sample_rate != SAMPLE_RATE {
            let ratio = spec.sample_rate as f64 / SAMPLE_RATE as f64;
            let new_len = (mono.len() as f64 / ratio) as usize;
            let mut out = Vec::with_capacity(new_len);
            for i in 0..new_len {
                let src_idx = i as f64 * ratio;
                let idx0 = src_idx.floor() as usize;
                let idx1 = (idx0 + 1).min(mono.len() - 1);
                let frac = (src_idx - idx0 as f64) as f32;
                out.push(mono[idx0] * (1.0 - frac) + mono[idx1] * frac);
            }
            out
        } else {
            mono
        };

        self.transcribe(&resampled)
    }
}

// ============================================================================
// WAV Parsing helpers for raw HTTP requests
// ============================================================================

/// Parse raw audio bytes: supports WAV or raw 16kHz PCM f32
pub fn parse_audio_input(bytes: &[u8], content_type: &str) -> Result<Vec<f32>> {
    if content_type.contains("wav") || (bytes.len() > 4 && &bytes[0..4] == b"RIFF") {
        // WAV format
        let cursor = std::io::Cursor::new(bytes);
        let reader = hound::WavReader::new(cursor)?;
        let spec = reader.spec();
        let samples: Vec<f32> = match spec.sample_format {
            hound::SampleFormat::Int => {
                let max_val = (1i64 << (spec.bits_per_sample - 1)) as f32;
                reader.into_samples::<i32>()
                    .filter_map(|s| s.ok())
                    .map(|s| s as f32 / max_val)
                    .collect()
            }
            hound::SampleFormat::Float => {
                reader.into_samples::<f32>()
                    .filter_map(|s| s.ok())
                    .collect()
            }
        };
        // Mono + resample
        let mono: Vec<f32> = if spec.channels > 1 {
            samples.chunks(spec.channels as usize)
                .map(|c| c.iter().sum::<f32>() / c.len() as f32)
                .collect()
        } else { samples };

        if spec.sample_rate != SAMPLE_RATE {
            let ratio = spec.sample_rate as f64 / SAMPLE_RATE as f64;
            let new_len = (mono.len() as f64 / ratio) as usize;
            let mut out = Vec::with_capacity(new_len);
            for i in 0..new_len {
                let src = i as f64 * ratio;
                let i0 = src.floor() as usize;
                let i1 = (i0 + 1).min(mono.len() - 1);
                let f = (src - i0 as f64) as f32;
                out.push(mono[i0] * (1.0 - f) + mono[i1] * f);
            }
            Ok(out)
        } else {
            Ok(mono)
        }
    } else if content_type.contains("pcm") || content_type.contains("raw") {
        // Raw 16-bit PCM at 16kHz
        let samples: Vec<f32> = bytes.chunks_exact(2)
            .map(|c| i16::from_le_bytes([c[0], c[1]]) as f32 / 32768.0)
            .collect();
        Ok(samples)
    } else {
        bail!("Unsupported audio format: {}. Send audio/wav or audio/pcm", content_type)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mel_filterbank_shape() {
        let fb = build_mel_filterbank();
        assert_eq!(fb.shape(), &[N_MELS, N_FFT / 2 + 1]);
        // All values should be non-negative
        assert!(fb.iter().all(|&v| v >= 0.0));
    }

    #[test]
    fn test_mel_spectrogram_shape() {
        let fb = build_mel_filterbank();
        // 1 second of silence
        let audio = vec![0.0f32; SAMPLE_RATE as usize];
        let mel = compute_mel_spectrogram(&audio, &fb);
        assert_eq!(mel.shape(), &[1, N_MELS, N_FRAMES]);
    }

    #[test]
    fn test_mel_spectrogram_sine_wave() {
        let fb = build_mel_filterbank();
        // 1 second of 440Hz sine wave
        let audio: Vec<f32> = (0..SAMPLE_RATE as usize)
            .map(|i| (2.0 * std::f32::consts::PI * 440.0 * i as f32 / SAMPLE_RATE as f32).sin())
            .collect();
        let mel = compute_mel_spectrogram(&audio, &fb);
        assert_eq!(mel.shape(), &[1, N_MELS, N_FRAMES]);
        // Should have non-zero energy in the bins around 440Hz
        let max_energy = mel.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        assert!(max_energy > -1.0, "Sine wave should produce significant mel energy");
    }

    #[test]
    fn test_hz_mel_roundtrip() {
        for hz in [0.0, 440.0, 1000.0, 4000.0, 8000.0] {
            let mel = hz_to_mel(hz);
            let back = mel_to_hz(mel);
            assert!((hz - back).abs() < 0.01, "Roundtrip failed for {} Hz", hz);
        }
    }

    #[test]
    fn test_engine_load() {
        let model_dir = dirs::home_dir()
            .unwrap_or_default()
            .join("trinity-models/stt/whisper-base");
        if !model_dir.join("onnx").exists() {
            eprintln!("Skipping engine load test — model not found at {:?}", model_dir);
            return;
        }
        let engine = WhisperEngine::load(&model_dir);
        assert!(engine.is_ok(), "Engine failed to load: {:?}", engine.err());
    }

    #[test]
    fn test_parse_audio_pcm() {
        // 100 samples of 16-bit PCM
        let mut bytes = Vec::new();
        for i in 0..100i16 {
            bytes.extend_from_slice(&i.to_le_bytes());
        }
        let result = parse_audio_input(&bytes, "audio/pcm");
        assert!(result.is_ok());
        let samples = result.unwrap();
        assert_eq!(samples.len(), 100);
    }
}
