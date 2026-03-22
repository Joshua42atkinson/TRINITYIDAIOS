// Memory Math Calculation for Trinity on GMKtec Evo X2
// LPDDR5X 6400 MT/s with 128-bit bus (16 bytes per transfer)

fn main() {
    println!("=== Trinity Memory Math Calculation ===");
    
    // LPDDR5X specifications - STRIX HALO SPECIAL!
    let lpddr5x_mhz = 8000; // MT/s (Strix Halo supports up to 8000 MT/s)
    let bus_width_bytes = 32; // 256-bit bus = 32 bytes
    let theoretical_bandwidth = lpddr5x_mhz * bus_width_bytes;
    
    println!("\n1. LPDDR5X Bandwidth Calculation:");
    println!("   LPDDR5X Speed: {} MT/s", lpddr5x_mhz);
    println!("   Bus Width: {} bytes (256-bit)", bus_width_bytes);
    println!("   Theoretical Bandwidth: {} MB/s", theoretical_bandwidth);
    println!("   Theoretical Bandwidth: {:.2} GB/s", theoretical_bandwidth as f64 / 1024.0);
    
    // System memory
    let total_memory_gb: f64 = 128.0;
    
    println!("\n2. System Memory:");
    println!("   Total Memory: {} GB", total_memory_gb);
    
    // Model sizes (from HuggingFace)
    let qwen_122b_model_gb = 46.6; // IQ3_S quantized
    let qwen_vision_gb = 0.912; // mmproj-BF16.gguf
    let minimax_model_gb = 66.0; // MiniMax-M2.5-REAP-50-Q4_K_M.gguf
    let qwen_35b_model_gb = 20.0; // Qwen3.5-35B-A3B-Q4_K_M.gguf
    
    println!("\n3. Model Sizes (GGUF Quantized):");
    println!("   Qwen3.5-122B-A10B (IQ3_S): {:.1} GB", qwen_122b_model_gb);
    println!("   Qwen Vision Projector: {:.3} GB", qwen_vision_gb);
    println!("   MiniMax-M2.5-REAP-50 (Q4_K_M): {:.1} GB", minimax_model_gb);
    println!("   Qwen3.5-35B-A3B (Q4_K_M): {:.1} GB", qwen_35b_model_gb);
    
    // Memory allocation scenarios
    println!("\n4. Memory Allocation Scenarios:");
    
    // Scenario 1: Qwen 122B as primary
    let scenario1_ai = qwen_122b_model_gb + qwen_vision_gb + 4.0 + 8.0 + 2.0 + 1.0 + 15.0; // AI + audio + embedding + RAG + MCP + Bevy
    let scenario1_system = total_memory_gb - scenario1_ai;
    println!("   Scenario 1 (Qwen 122B Primary):");
    println!("     AI Models + Core: {:.1} GB", scenario1_ai);
    println!("     System Available: {:.1} GB", scenario1_system);
    
    // Scenario 2: MiniMax as primary
    let scenario2_ai = minimax_model_gb + qwen_35b_model_gb + qwen_vision_gb + 4.0 + 8.0 + 2.0 + 1.0 + 15.0;
    let scenario2_system = total_memory_gb - scenario2_ai;
    println!("   Scenario 2 (MiniMax + Qwen 35B):");
    println!("     AI Models + Core: {:.1} GB", scenario2_ai);
    println!("     System Available: {:.1} GB", scenario2_system);
    
    // Scenario 3: Single model only
    let scenario3_ai = qwen_122b_model_gb + qwen_vision_gb + 4.0 + 8.0 + 2.0 + 1.0 + 15.0;
    let scenario3_system = total_memory_gb - scenario3_ai;
    println!("   Scenario 3 (Qwen 122B Only):");
    println!("     AI Model + Core: {:.1} GB", scenario3_ai);
    println!("     System Available: {:.1} GB", scenario3_system);
    
    // Bandwidth analysis
    println!("\n5. Bandwidth Analysis:");
    let models_loaded_gb = qwen_122b_model_gb + qwen_vision_gb;
    let load_time_seconds = models_loaded_gb * 1024.0 / theoretical_bandwidth as f64;
    println!("   Loading Qwen 122B + Vision: {:.1} GB", models_loaded_gb);
    println!("   At theoretical bandwidth: {:.2} seconds", load_time_seconds);
    println!("   At 50% efficiency: {:.2} seconds", load_time_seconds * 2.0);
    println!("   At 25% efficiency: {:.2} seconds", load_time_seconds * 4.0);
    
    // Critical insight
    println!("\n6. CRITICAL INSIGHT:");
    println!("   The 59GB download size vs 47GB loaded size discrepancy:");
    println!("   - Download size: 59GB (compressed GGUF)");
    println!("   - Loaded size: 47GB (decompressed in memory)");
    println!("   - Ratio: {:.2}x compression", 59.0 / 47.0);
    println!("   - GGUF files are compressed on disk, decompress when loaded");
    
    println!("\n7. Memory Bandwidth Bottleneck:");
    println!("   With 256 GB/s theoretical bandwidth:");
    println!("   - Loading 47GB model: ~2 seconds theoretical");
    println!("   - Real-world with overhead: 4-8 seconds");
    println!("   - This is why we need model rotation!");
    
    println!("\n=== Calculation Complete ===");
}
