use anyhow::Result;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::LlamaModel;

fn main() -> Result<()> {
    println!("Initializing llama.cpp backend...");
    std::env::set_var("RADV_PERFMODE", "nogttspill");
    let backend = LlamaBackend::init()?;
    
    let path = "/home/joshua/trinity-models/gguf/Mistral-Small-4-119B-2603-Q4_K_M-00001-of-00002.gguf";
    println!("Loading model from {}", path);
    
    let model_params = LlamaModelParams::default()
        .with_n_gpu_layers(999)
        .with_use_mmap(false);
    let model_params = std::pin::pin!(model_params);
    
    match LlamaModel::load_from_file(&backend, path, &model_params) {
        Ok(model) => println!("Success! Vocab: {}", model.n_vocab()),
        Err(e) => {
            eprintln!("🔥 CRASH DETAIL 🔥");
            eprintln!("{:?}", e);
        }
    }
    
    Ok(())
}
