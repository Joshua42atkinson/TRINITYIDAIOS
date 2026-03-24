// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — Supertonic-2 TTS Engine
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        supertonic.rs
// PURPOSE:     Native ONNX Runtime TTS — Supertonic-2 (66M params, 167× realtime)
//
// ARCHITECTURE:
//   text → UnicodeProcessor → duration_predictor → text_encoder
//     → vector_estimator (flow matching, 5 steps) → vocoder → 44.1kHz WAV
//
// Based on: https://github.com/supertone-inc/supertonic (MIT/OpenRAIL-M)
// ═══════════════════════════════════════════════════════════════════════════════

use anyhow::{bail, Context, Result};
use hound::{SampleFormat, WavSpec, WavWriter};
use ndarray::{Array, Array3};
use ort::{session::Session, value::Value};
use rand_distr::{Distribution, Normal};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Cursor};
use std::path::Path;
use tracing::info;
use unicode_normalization::UnicodeNormalization;

// ============================================================================
// Configuration
// ============================================================================

pub const AVAILABLE_LANGS: &[&str] = &["en", "ko", "es", "pt", "fr"];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub ae: AEConfig,
    pub ttl: TTLConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AEConfig {
    pub sample_rate: i32,
    pub base_chunk_size: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TTLConfig {
    pub chunk_compress_factor: i32,
    pub latent_dim: i32,
}

fn load_cfgs<P: AsRef<Path>>(onnx_dir: P) -> Result<Config> {
    let cfg_path = onnx_dir.as_ref().join("tts.json");
    let file = File::open(cfg_path)?;
    let reader = BufReader::new(file);
    let cfgs: Config = serde_json::from_reader(reader)?;
    Ok(cfgs)
}

// ============================================================================
// Voice Style
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
struct VoiceStyleData {
    style_ttl: StyleComponent,
    style_dp: StyleComponent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StyleComponent {
    data: Vec<Vec<Vec<f32>>>,
    dims: Vec<usize>,
    #[serde(rename = "type")]
    dtype: String,
}

#[derive(Clone)]
pub struct Style {
    pub ttl: Array3<f32>,
    pub dp: Array3<f32>,
}

pub fn load_voice_style(path: &str) -> Result<Style> {
    let file = File::open(path).context("Failed to open voice style file")?;
    let reader = BufReader::new(file);
    let data: VoiceStyleData = serde_json::from_reader(reader)?;

    let ttl_dims = &data.style_ttl.dims;
    let dp_dims = &data.style_dp.dims;

    // Flatten TTL
    let mut ttl_flat = Vec::with_capacity(ttl_dims[1] * ttl_dims[2]);
    for batch in &data.style_ttl.data {
        for row in batch {
            ttl_flat.extend_from_slice(row);
        }
    }
    let ttl = Array3::from_shape_vec((1, ttl_dims[1], ttl_dims[2]), ttl_flat)?;

    // Flatten DP
    let mut dp_flat = Vec::with_capacity(dp_dims[1] * dp_dims[2]);
    for batch in &data.style_dp.data {
        for row in batch {
            dp_flat.extend_from_slice(row);
        }
    }
    let dp = Array3::from_shape_vec((1, dp_dims[1], dp_dims[2]), dp_flat)?;

    Ok(Style { ttl, dp })
}

// ============================================================================
// Unicode Text Processor
// ============================================================================

pub struct UnicodeProcessor {
    indexer: Vec<i64>,
}

impl UnicodeProcessor {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let indexer: Vec<i64> = serde_json::from_reader(reader)?;
        Ok(UnicodeProcessor { indexer })
    }

    fn call(&self, text_list: &[String], lang_list: &[String]) -> Result<(Vec<Vec<i64>>, Array3<f32>)> {
        let mut processed: Vec<String> = Vec::new();
        for (text, lang) in text_list.iter().zip(lang_list.iter()) {
            processed.push(preprocess_text(text, lang)?);
        }

        let lengths: Vec<usize> = processed.iter().map(|t| t.chars().count()).collect();
        let max_len = *lengths.iter().max().unwrap_or(&0);

        let mut text_ids = Vec::new();
        for text in &processed {
            let mut row = vec![0i64; max_len];
            for (j, c) in text.chars().enumerate() {
                let val = c as usize;
                row[j] = if val < self.indexer.len() { self.indexer[val] } else { -1 };
            }
            text_ids.push(row);
        }

        let text_mask = length_to_mask(&lengths, Some(max_len));
        Ok((text_ids, text_mask))
    }
}

fn preprocess_text(text: &str, lang: &str) -> Result<String> {
    let mut text: String = text.nfkd().collect();

    // Remove emojis
    let emoji_re = Regex::new(r"[\x{1F600}-\x{1F64F}\x{1F300}-\x{1F5FF}\x{1F680}-\x{1F6FF}\x{2600}-\x{26FF}\x{2700}-\x{27BF}]+").unwrap();
    text = emoji_re.replace_all(&text, "").to_string();

    // Replace dashes/symbols
    for (from, to) in &[
        ("–", "-"), ("‑", "-"), ("—", "-"), ("_", " "),
        ("\u{201C}", "\""), ("\u{201D}", "\""), ("\u{2018}", "'"), ("\u{2019}", "'"),
        ("´", "'"), ("`", "'"), ("[", " "), ("]", " "), ("|", " "),
        ("/", " "), ("#", " "), ("→", " "), ("←", " "),
    ] {
        text = text.replace(from, to);
    }

    for sym in &["♥", "☆", "♡", "©", "\\"] {
        text = text.replace(sym, "");
    }

    text = text.replace("@", " at ");
    text = text.replace("e.g.,", "for example, ");
    text = text.replace("i.e.,", "that is, ");

    // Fix spacing
    text = Regex::new(r" ,").unwrap().replace_all(&text, ",").to_string();
    text = Regex::new(r" \.").unwrap().replace_all(&text, ".").to_string();
    text = Regex::new(r" !").unwrap().replace_all(&text, "!").to_string();
    text = Regex::new(r" \?").unwrap().replace_all(&text, "?").to_string();
    text = Regex::new(r"\s+").unwrap().replace_all(&text, " ").to_string();
    text = text.trim().to_string();

    // Add trailing period if needed
    if !text.is_empty() {
        let ends_punct = Regex::new(r#"[.!?;:,'")\]}…]$"#).unwrap();
        if !ends_punct.is_match(&text) {
            text.push('.');
        }
    }

    if !AVAILABLE_LANGS.contains(&lang) {
        bail!("Invalid language: {}. Available: {:?}", lang, AVAILABLE_LANGS);
    }

    text = format!("<{lang}>{text}</{lang}>");
    Ok(text)
}

// ============================================================================
// Math helpers
// ============================================================================

fn length_to_mask(lengths: &[usize], max_len: Option<usize>) -> Array3<f32> {
    let bsz = lengths.len();
    let max_len = max_len.unwrap_or_else(|| *lengths.iter().max().unwrap_or(&0));
    let mut mask = Array3::<f32>::zeros((bsz, 1, max_len));
    for (i, &len) in lengths.iter().enumerate() {
        for j in 0..len.min(max_len) {
            mask[[i, 0, j]] = 1.0;
        }
    }
    mask
}

fn sample_noisy_latent(
    duration: &[f32], sample_rate: i32, base_chunk_size: i32,
    chunk_compress: i32, latent_dim: i32,
) -> (Array3<f32>, Array3<f32>) {
    let bsz = duration.len();
    let max_dur = duration.iter().fold(0.0f32, |a, &b| a.max(b));
    let wav_len_max = (max_dur * sample_rate as f32) as usize;
    let wav_lengths: Vec<usize> = duration.iter().map(|&d| (d * sample_rate as f32) as usize).collect();
    let chunk_size = (base_chunk_size * chunk_compress) as usize;
    let latent_len = (wav_len_max + chunk_size - 1) / chunk_size;
    let latent_dim_val = (latent_dim * chunk_compress) as usize;

    let mut noisy = Array3::<f32>::zeros((bsz, latent_dim_val, latent_len));
    let normal = Normal::new(0.0, 1.0).unwrap();
    let mut rng = rand::thread_rng();

    for b in 0..bsz {
        for d in 0..latent_dim_val {
            for t in 0..latent_len {
                noisy[[b, d, t]] = normal.sample(&mut rng);
            }
        }
    }

    let latent_lengths: Vec<usize> = wav_lengths.iter().map(|&l| (l + chunk_size - 1) / chunk_size).collect();
    let latent_mask = length_to_mask(&latent_lengths, Some(latent_len));

    // Apply mask
    for b in 0..bsz {
        for d in 0..latent_dim_val {
            for t in 0..latent_len {
                noisy[[b, d, t]] *= latent_mask[[b, 0, t]];
            }
        }
    }

    (noisy, latent_mask)
}

// ============================================================================
// Text Chunking
// ============================================================================

fn chunk_text(text: &str, max_len: usize) -> Vec<String> {
    let text = text.trim();
    if text.is_empty() {
        return vec![String::new()];
    }
    if text.len() <= max_len {
        return vec![text.to_string()];
    }

    let para_re = Regex::new(r"\n\s*\n").unwrap();
    let sent_re = Regex::new(r"([.!?])\s+").unwrap();
    let mut chunks = Vec::new();

    for para in para_re.split(text) {
        let para = para.trim();
        if para.is_empty() { continue; }
        if para.len() <= max_len {
            chunks.push(para.to_string());
            continue;
        }

        let mut current = String::new();
        let mut cur_len = 0;

        // Split by sentence boundary
        let mut last = 0;
        let sentences: Vec<&str> = {
            let mut sents = Vec::new();
            for m in sent_re.find_iter(para) {
                sents.push(&para[last..m.end()]);
                last = m.end();
            }
            if last < para.len() { sents.push(&para[last..]); }
            if sents.is_empty() { sents.push(para); }
            sents
        };

        for sent in sentences {
            let sent = sent.trim();
            if sent.is_empty() { continue; }
            if cur_len + sent.len() + 1 > max_len && !current.is_empty() {
                chunks.push(current.trim().to_string());
                current.clear();
                cur_len = 0;
            }
            if !current.is_empty() { current.push(' '); cur_len += 1; }
            current.push_str(sent);
            cur_len += sent.len();
        }
        if !current.is_empty() { chunks.push(current.trim().to_string()); }
    }

    if chunks.is_empty() { vec![text.to_string()] } else { chunks }
}

// ============================================================================
// TTS Engine
// ============================================================================

pub struct SupertonicEngine {
    cfgs: Config,
    text_processor: UnicodeProcessor,
    dp_ort: Session,
    text_enc_ort: Session,
    vector_est_ort: Session,
    vocoder_ort: Session,
    pub sample_rate: i32,
    voices: std::collections::HashMap<String, Style>,
}

impl SupertonicEngine {
    /// Load the engine from an ONNX model directory.
    pub fn load<P: AsRef<Path>>(model_dir: P) -> Result<Self> {
        let model_dir = model_dir.as_ref();
        let onnx_dir = model_dir.join("onnx");

        info!("Loading Supertonic-2 from {}...", model_dir.display());
        let t0 = std::time::Instant::now();

        let cfgs = load_cfgs(&onnx_dir)?;
        let text_processor = UnicodeProcessor::new(onnx_dir.join("unicode_indexer.json"))?;

        let dp_ort = Session::builder()?.commit_from_file(onnx_dir.join("duration_predictor.onnx"))?;
        let text_enc_ort = Session::builder()?.commit_from_file(onnx_dir.join("text_encoder.onnx"))?;
        let vector_est_ort = Session::builder()?.commit_from_file(onnx_dir.join("vector_estimator.onnx"))?;
        let vocoder_ort = Session::builder()?.commit_from_file(onnx_dir.join("vocoder.onnx"))?;

        let sample_rate = cfgs.ae.sample_rate;

        // Pre-load all voice styles
        let styles_dir = model_dir.join("voice_styles");
        let mut voices = std::collections::HashMap::new();
        if styles_dir.exists() {
            for entry in std::fs::read_dir(&styles_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().map_or(false, |e| e == "json") {
                    let name = path.file_stem().unwrap().to_string_lossy().to_string();
                    match load_voice_style(path.to_str().unwrap()) {
                        Ok(style) => {
                            info!("  Voice: {}", name);
                            voices.insert(name, style);
                        }
                        Err(e) => tracing::warn!("Failed to load voice {}: {}", name, e),
                    }
                }
            }
        }

        info!("Supertonic-2 loaded in {:.1}s ({} voices)", t0.elapsed().as_secs_f32(), voices.len());

        Ok(SupertonicEngine {
            cfgs,
            text_processor,
            dp_ort,
            text_enc_ort,
            vector_est_ort,
            vocoder_ort,
            sample_rate,
            voices,
        })
    }

    /// List available voice names.
    pub fn voice_names(&self) -> Vec<String> {
        self.voices.keys().cloned().collect()
    }

    /// Get a voice style by name (e.g., "M1", "F3").
    pub fn get_voice(&self, name: &str) -> Option<&Style> {
        self.voices.get(name)
    }

    /// Synthesize text to WAV bytes.
    pub fn synthesize(&mut self, text: &str, voice: &str) -> Result<Vec<u8>> {
        let style = self.voices.get(voice)
            .or_else(|| self.voices.get("M1"))
            .context("No voice styles loaded")?
            .clone();

        let lang = "en";
        let total_step = 5;
        let speed = 1.05f32;
        let silence_dur = 0.3f32;
        let max_chunk = 300;

        let chunks = chunk_text(text, max_chunk);
        let mut wav_cat: Vec<f32> = Vec::new();

        for (i, chunk) in chunks.iter().enumerate() {
            if chunk.is_empty() { continue; }
            let (wav, duration) = self.infer(&[chunk.clone()], &[lang.to_string()], &style, total_step, speed)?;
            let dur = duration[0];
            let wav_len = (self.sample_rate as f32 * dur) as usize;
            let wav_chunk = &wav[..wav_len.min(wav.len())];

            if i > 0 {
                let silence_len = (silence_dur * self.sample_rate as f32) as usize;
                wav_cat.extend(std::iter::repeat(0.0f32).take(silence_len));
            }
            wav_cat.extend_from_slice(wav_chunk);
        }

        // Encode to WAV bytes in memory
        let mut cursor = Cursor::new(Vec::new());
        {
            let spec = WavSpec {
                channels: 1,
                sample_rate: self.sample_rate as u32,
                bits_per_sample: 16,
                sample_format: SampleFormat::Int,
            };
            let mut writer = WavWriter::new(&mut cursor, spec)?;
            for &sample in &wav_cat {
                let clamped = sample.max(-1.0).min(1.0);
                writer.write_sample((clamped * 32767.0) as i16)?;
            }
            writer.finalize()?;
        }

        Ok(cursor.into_inner())
    }

    /// Core inference: text → ONNX pipeline → waveform samples.
    fn infer(
        &mut self,
        text_list: &[String],
        lang_list: &[String],
        style: &Style,
        total_step: usize,
        speed: f32,
    ) -> Result<(Vec<f32>, Vec<f32>)> {
        let bsz = text_list.len();

        // Tokenize
        let (text_ids, text_mask) = self.text_processor.call(text_list, lang_list)?;
        let text_ids_shape = (bsz, text_ids[0].len());
        let mut flat_ids = Vec::new();
        for row in &text_ids {
            flat_ids.extend_from_slice(row);
        }
        let text_ids_array = Array::from_shape_vec(text_ids_shape, flat_ids)?;

        let text_ids_val = Value::from_array(text_ids_array)?;
        let text_mask_val = Value::from_array(text_mask.clone())?;
        let style_dp_val = Value::from_array(style.dp.clone())?;

        // Duration prediction
        let dp_out = self.dp_ort.run(ort::inputs! {
            "text_ids" => &text_ids_val,
            "style_dp" => &style_dp_val,
            "text_mask" => &text_mask_val
        })?;

        let (_, dur_data) = dp_out["duration"].try_extract_tensor::<f32>()?;
        let mut duration: Vec<f32> = dur_data.to_vec();
        for d in duration.iter_mut() { *d /= speed; }

        // Text encoding
        let style_ttl_val = Value::from_array(style.ttl.clone())?;
        let enc_out = self.text_enc_ort.run(ort::inputs! {
            "text_ids" => &text_ids_val,
            "style_ttl" => &style_ttl_val,
            "text_mask" => &text_mask_val
        })?;

        let (emb_shape, emb_data) = enc_out["text_emb"].try_extract_tensor::<f32>()?;
        let text_emb = Array3::from_shape_vec(
            (emb_shape[0] as usize, emb_shape[1] as usize, emb_shape[2] as usize),
            emb_data.to_vec(),
        )?;

        // Sample noisy latent
        let (mut xt, latent_mask) = sample_noisy_latent(
            &duration, self.sample_rate,
            self.cfgs.ae.base_chunk_size, self.cfgs.ttl.chunk_compress_factor, self.cfgs.ttl.latent_dim,
        );

        let total_step_arr = Array::from_elem(bsz, total_step as f32);

        // Flow matching denoising loop
        for step in 0..total_step {
            let step_arr = Array::from_elem(bsz, step as f32);

            let xt_val = Value::from_array(xt.clone())?;
            let emb_val = Value::from_array(text_emb.clone())?;
            let lm_val = Value::from_array(latent_mask.clone())?;
            let tm_val = Value::from_array(text_mask.clone())?;
            let cs_val = Value::from_array(step_arr)?;
            let ts_val = Value::from_array(total_step_arr.clone())?;

            let est_out = self.vector_est_ort.run(ort::inputs! {
                "noisy_latent" => &xt_val,
                "text_emb" => &emb_val,
                "style_ttl" => &style_ttl_val,
                "latent_mask" => &lm_val,
                "text_mask" => &tm_val,
                "current_step" => &cs_val,
                "total_step" => &ts_val
            })?;

            let (den_shape, den_data) = est_out["denoised_latent"].try_extract_tensor::<f32>()?;
            xt = Array3::from_shape_vec(
                (den_shape[0] as usize, den_shape[1] as usize, den_shape[2] as usize),
                den_data.to_vec(),
            )?;
        }

        // Vocoder
        let lat_val = Value::from_array(xt)?;
        let voc_out = self.vocoder_ort.run(ort::inputs! {
            "latent" => &lat_val
        })?;

        let (_, wav_data) = voc_out["wav_tts"].try_extract_tensor::<f32>()?;
        Ok((wav_data.to_vec(), duration))
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preprocess_basic() {
        let result = preprocess_text("Hello world", "en").unwrap();
        assert!(result.starts_with("<en>"));
        assert!(result.ends_with("</en>"));
        assert!(result.contains("Hello world."));
    }

    #[test]
    fn test_preprocess_removes_emojis() {
        let result = preprocess_text("Hello 🔥 world 🎉", "en").unwrap();
        assert!(!result.contains("🔥"));
        assert!(!result.contains("🎉"));
        assert!(result.contains("Hello"));
        assert!(result.contains("world"));
    }

    #[test]
    fn test_preprocess_keeps_trailing_punct() {
        let result = preprocess_text("Is this working?", "en").unwrap();
        assert!(result.contains("Is this working?"));
        // Should NOT add an extra period
        assert!(!result.contains("?."));
    }

    #[test]
    fn test_preprocess_adds_period() {
        let result = preprocess_text("No punctuation here", "en").unwrap();
        assert!(result.contains("No punctuation here."));
    }

    #[test]
    fn test_preprocess_replaces_dashes() {
        let result = preprocess_text("hello—world", "en").unwrap();
        assert!(result.contains("hello-world"));
    }

    #[test]
    fn test_preprocess_invalid_lang() {
        let result = preprocess_text("Hello", "zz");
        assert!(result.is_err());
    }

    #[test]
    fn test_chunk_text_short() {
        let chunks = chunk_text("Short sentence.", 300);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], "Short sentence.");
    }

    #[test]
    fn test_chunk_text_long() {
        let long = "First sentence. Second sentence. Third sentence. Fourth sentence. Fifth sentence.";
        let chunks = chunk_text(long, 40);
        assert!(chunks.len() > 1, "Expected multiple chunks, got {}", chunks.len());
        for chunk in &chunks {
            assert!(chunk.len() <= 45, "Chunk too long: {} chars", chunk.len()); // Allow small overflow
        }
    }

    #[test]
    fn test_chunk_text_empty() {
        let chunks = chunk_text("", 300);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], "");
    }

    #[test]
    fn test_length_to_mask() {
        let mask = length_to_mask(&[3, 5], Some(6));
        assert_eq!(mask.shape(), &[2, 1, 6]);
        // First row: [1,1,1,0,0,0]
        assert_eq!(mask[[0, 0, 2]], 1.0);
        assert_eq!(mask[[0, 0, 3]], 0.0);
        // Second row: [1,1,1,1,1,0]
        assert_eq!(mask[[1, 0, 4]], 1.0);
        assert_eq!(mask[[1, 0, 5]], 0.0);
    }

    #[test]
    fn test_engine_load() {
        let model_dir = dirs::home_dir()
            .unwrap_or_default()
            .join("trinity-models/tts/supertonic-2");
        if !model_dir.join("onnx").exists() {
            eprintln!("Skipping engine load test — model not found at {:?}", model_dir);
            return;
        }
        let engine = SupertonicEngine::load(&model_dir);
        assert!(engine.is_ok(), "Engine failed to load: {:?}", engine.err());
        let engine = engine.unwrap();
        assert!(engine.voices.len() >= 10, "Expected 10 voices, got {}", engine.voices.len());
        assert_eq!(engine.sample_rate, 44100);
    }

    #[test]
    fn test_synthesize() {
        let model_dir = dirs::home_dir()
            .unwrap_or_default()
            .join("trinity-models/tts/supertonic-2");
        if !model_dir.join("onnx").exists() {
            eprintln!("Skipping synthesis test — model not found");
            return;
        }
        let mut engine = SupertonicEngine::load(&model_dir).unwrap();
        let t0 = std::time::Instant::now();
        let wav = engine.synthesize("Hello world.", "M1");
        let ms = t0.elapsed().as_millis();
        assert!(wav.is_ok(), "Synthesis failed: {:?}", wav.err());
        let wav = wav.unwrap();
        assert!(wav.len() > 1000, "WAV too small: {} bytes", wav.len());
        assert_eq!(&wav[0..4], b"RIFF", "Not a valid WAV file");
        assert_eq!(&wav[8..12], b"WAVE", "Not a valid WAV file");
        eprintln!("  Synthesized {} bytes in {}ms", wav.len(), ms);
    }

    #[test]
    fn test_multi_voice_synthesis() {
        let model_dir = dirs::home_dir()
            .unwrap_or_default()
            .join("trinity-models/tts/supertonic-2");
        if !model_dir.join("onnx").exists() {
            eprintln!("Skipping multi-voice test — model not found");
            return;
        }
        let mut engine = SupertonicEngine::load(&model_dir).unwrap();
        for voice in &["M1", "M3", "F1", "F3"] {
            let t0 = std::time::Instant::now();
            let wav = engine.synthesize("Testing voice quality.", voice).unwrap();
            let ms = t0.elapsed().as_millis();
            assert!(wav.len() > 1000, "Voice {} produced too-small WAV", voice);
            assert_eq!(&wav[0..4], b"RIFF");
            eprintln!("  {}: {} bytes in {}ms", voice, wav.len(), ms);
        }
    }
}
