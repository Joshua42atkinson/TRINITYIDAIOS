use ort::session::builder::GraphOptimizationLevel;
use ort::session::Session;
use ort::value::TensorRef;
use std::sync::OnceLock;
use tokenizers::Tokenizer;
use tracing::{debug, info, warn};

/// ONNX model path for nomic-embed-text-v1.5 INT8
const MODEL_PATH: &str = "/home/joshua/trinity-models/vllm/nomic-embed-text-v1.5-AWQ/onnx/model_int8.onnx";
const TOKENIZER_PATH: &str = "/home/joshua/trinity-models/vllm/nomic-embed-text-v1.5-AWQ/tokenizer.json";

/// Expected embedding dimension (must match rag.rs EMBEDDING_DIM)
pub const EMBEDDING_DIM: usize = 768;

struct OrtEmbedder {
    session: std::sync::Mutex<Session>,
    tokenizer: std::sync::Mutex<Tokenizer>,
}

static EMBEDDER: OnceLock<Option<OrtEmbedder>> = OnceLock::new();

fn get_embedder() -> &'static Option<OrtEmbedder> {
    EMBEDDER.get_or_init(|| {
        match load_embedder() {
            Ok(e) => {
                info!("[ORT] Embedding model loaded successfully");
                Some(e)
            }
            Err(e) => {
                warn!("[ORT] Failed to load embedding model: {}, will fall back to API", e);
                None
            }
        }
    })
}

fn load_embedder() -> anyhow::Result<OrtEmbedder> {
    let builder = Session::builder()
        .map_err(|e| anyhow::anyhow!("ORT builder failed: {}", e))?;
    let session = builder
        .with_optimization_level(GraphOptimizationLevel::Level3)
        .map_err(|e| anyhow::anyhow!("ORT opt level failed: {}", e))?
        .with_intra_threads(2)
        .map_err(|e| anyhow::anyhow!("ORT threads failed: {}", e))?
        .commit_from_file(MODEL_PATH)
        .map_err(|e| anyhow::anyhow!("Failed to load ONNX model from {}: {}", MODEL_PATH, e))?;

    let tokenizer = Tokenizer::from_file(TOKENIZER_PATH)
        .map_err(|e| anyhow::anyhow!("Failed to load tokenizer from {}: {}", TOKENIZER_PATH, e))?;

    Ok(OrtEmbedder {
        session: std::sync::Mutex::new(session),
        tokenizer: std::sync::Mutex::new(tokenizer),
    })
}

/// Generate embedding using in-process ONNX Runtime.
/// Returns None if ORT is not available (caller should fall back to API/hash).
pub fn generate_embedding_ort(text: &str) -> Option<Vec<f32>> {
    let embedder = get_embedder().as_ref()?;

    // Tokenize
    let encoding = {
        let tok = embedder.tokenizer.lock().ok()?;
        match tok.encode(text, true) {
            Ok(e) => e,
            Err(e) => {
                warn!("[ORT] Tokenization failed: {}", e);
                return None;
            }
        }
    };

    let input_ids = encoding.get_ids();
    let attention_mask = encoding.get_attention_mask();
    let seq_len = input_ids.len();

    // Convert to i64 arrays for ONNX
    let input_ids_array = ndarray::Array2::from_shape_vec((1, seq_len), input_ids.iter().map(|&v| v as i64).collect()).ok()?;
    let attention_mask_array = ndarray::Array2::from_shape_vec((1, seq_len), attention_mask.iter().map(|&v| v as i64).collect()).ok()?;
    let token_type_ids_array = ndarray::Array2::from_shape_vec((1, seq_len), vec![0i64; seq_len]).ok()?;

    // Run inference and extract embedding while session lock is held
    // (SessionOutputs borrows from session)
    let embedding_result: Option<Vec<f32>> = {
        let mut session = embedder.session.lock().ok()?;
        let inputs = ort::inputs! {
            "input_ids" => TensorRef::from_array_view(&input_ids_array).ok()?,
            "attention_mask" => TensorRef::from_array_view(&attention_mask_array).ok()?,
            "token_type_ids" => TensorRef::from_array_view(&token_type_ids_array).ok()?,
        };
        let outputs = match session.run(inputs) {
            Ok(o) => o,
            Err(e) => {
                warn!("[ORT] Inference failed: {}", e);
                return None;
            }
        };

        // Extract embedding — try first output
        let output = outputs.values().next()?;
        let (shape, data) = output.try_extract_tensor::<f32>().ok()?;

        // shape is like [1, seq_len, 768] or [1, 768]
        let dims: Vec<usize> = shape.iter().map(|&d| d as usize).collect();

        if dims.len() == 3 {
            // [1, seq_len, 768] — mean pool over seq_len with attention mask
            let mut pooled = vec![0.0f32; EMBEDDING_DIM];
            let mut count = 0u32;
            for i in 0..seq_len {
                let mask = attention_mask[i] as u32;
                if mask > 0 {
                    for j in 0..EMBEDDING_DIM {
                        pooled[j] += data[i * EMBEDDING_DIM + j];
                    }
                    count += 1;
                }
            }
            if count > 0 {
                for v in &mut pooled {
                    *v /= count as f32;
                }
            }
            normalize(&mut pooled);
            Some(pooled)
        } else if dims.len() == 2 {
            // [1, 768] — already pooled
            let mut emb: Vec<f32> = data.to_vec();
            normalize(&mut emb);
            Some(emb)
        } else {
            warn!("[ORT] Unexpected output ndim: {}", dims.len());
            None
        }
    };

    let embedding = embedding_result?;

    if embedding.len() != EMBEDDING_DIM {
        debug!("[ORT] Embedding dim mismatch: got {}, expected {}", embedding.len(), EMBEDDING_DIM);
        return None;
    }

    Some(embedding)
}

fn normalize(vec: &mut [f32]) {
    let norm: f32 = vec.iter().map(|v| v * v).sum::<f32>().sqrt();
    if norm > 0.0 {
        for v in vec.iter_mut() {
            *v /= norm;
        }
    }
}
