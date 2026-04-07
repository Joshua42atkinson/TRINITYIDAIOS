use std::path::PathBuf;
use std::process::Command;
use tracing::{info, warn};
use tokio::time::{sleep, Duration};

const RECYCLER_PORT: u16 = 8001;
const PETE_PORT: u16 = 8002;
const ART_PORT: u16 = 8003;

fn get_vllm_models_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_default()
        .join("trinity-models/vllm")
}

pub async fn start_fleet() {
    let models_dir = get_vllm_models_dir();
    
    // 1. Great Recycler (35% VRAM)
    let recycler_model = models_dir.join("gemma-4-31B-it-AWQ-4bit");
    let speculative_model = models_dir.join("gemma-4-E2B-it-AWQ-4bit");
    if !crate::inference::check_health(&format!("http://127.0.0.1:{}/v1/models", RECYCLER_PORT)).await {
        if recycler_model.exists() && speculative_model.exists() {
            info!("🚀 Starting Great Recycler Sidecar (Port {})", RECYCLER_PORT);
            let _ = Command::new("python3")
                .arg("-m")
                .arg("vllm.entrypoints.openai.api_server")
                .arg("--model").arg(&recycler_model)
                .arg("--speculative-model").arg(&speculative_model)
                .arg("--num-speculative-tokens").arg("5")
                .arg("--quantization").arg("awq")
                .arg("--gpu-memory-utilization").arg("0.35")
                .arg("--port").arg(RECYCLER_PORT.to_string())
                .arg("--max-model-len").arg("524288")
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
                .map_err(|e| warn!("⚠️ Failed to launch Recycler: {}", e));
        } else {
            warn!("⚠️ Great Recycler models missing in {}", models_dir.display());
        }
    } else {
        info!("✅ Great Recycler already running on :{}", RECYCLER_PORT);
    }
    
    // 2. Programmer Pete (25% VRAM)
    let pete_model = models_dir.join("gemma-4-26B-A4B-it-AWQ-4bit");
    if !crate::inference::check_health(&format!("http://127.0.0.1:{}/v1/models", PETE_PORT)).await {
        if pete_model.exists() {
            info!("🚀 Starting Programmer Pete Sidecar (Port {})", PETE_PORT);
            let _ = Command::new("python3")
                .arg("-m")
                .arg("vllm.entrypoints.openai.api_server")
                .arg("--model").arg(&pete_model)
                .arg("--quantization").arg("awq")
                .arg("--gpu-memory-utilization").arg("0.25")
                .arg("--port").arg(PETE_PORT.to_string())
                .arg("--max-model-len").arg("524288")
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
                .map_err(|e| warn!("⚠️ Failed to launch Pete: {}", e));
        } else {
            warn!("⚠️ Programmer Pete model missing in {}", models_dir.display());
        }
    } else {
        info!("✅ Programmer Pete already running on :{}", PETE_PORT);
    }
    
    // 3. ART Engine (25% VRAM)
    let art_model = models_dir.join("gemma-4-E4B-it-AWQ-4bit");
    if !crate::inference::check_health(&format!("http://127.0.0.1:{}/v1/models", ART_PORT)).await {
        if art_model.exists() {
            info!("🚀 Starting ART Engine Sidecar (Port {})", ART_PORT);
            let _ = Command::new("python3")
                .arg("-m")
                .arg("vllm.entrypoints.openai.api_server")
                .arg("--model").arg(&art_model)
                .arg("--quantization").arg("awq")
                .arg("--gpu-memory-utilization").arg("0.25")
                .arg("--port").arg(ART_PORT.to_string())
                .arg("--max-model-len").arg("262144")
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
                .map_err(|e| warn!("⚠️ Failed to launch ART: {}", e));
        } else {
            warn!("⚠️ ART Engine model missing in {}", models_dir.display());
        }
    } else {
        info!("✅ ART Engine already running on :{}", ART_PORT);
    }
}
