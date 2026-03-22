#!/bin/bash
# Trinity Conductor Test Runner
# Tests Qwen3.5-REAP-97B memory tracking and performance

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║           TRINITY CONDUCTOR TEST RUNNER                      ║"
echo "║         Qwen3.5-REAP-97B Memory & Performance Tests           ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

# Build the test binary
echo "🔨 Building trinity-conductor-test..."
cargo build --bin trinity-conductor-test --release

if [ $? -ne 0 ]; then
    echo "❌ Build failed!"
    exit 1
fi

echo "✅ Build successful!"
echo ""

# Run basic memory test
echo "🧠 Running basic memory test..."
timeout 30s cargo run --release --bin trinity-conductor-test -- \
    --test-mode memory \
    --cycles 10 \
    --output test_results_memory.json \
    --verbose

echo ""

# Run KV cache test  
echo "🔍 Running KV cache quantization test..."
timeout 30s cargo run --release --bin trinity-conductor-test -- \
    --test-mode kv-cache \
    --quantization "Q4_K_M,Q5_K_M,Q8_0" \
    --context-size 8192 \
    --output test_results_kv_cache.json \
    --verbose

echo ""

# Run layer offloading test
echo "🏗️ Running layer offloading test..."
timeout 30s cargo run --release --bin trinity-conductor-test -- \
    --test-mode layers \
    --layers 38 \
    --output test_results_layers.json \
    --verbose

echo ""

# Run stress test
echo "💪 Running stress test (30 seconds)..."
timeout 35s cargo run --release --bin trinity-conductor-test -- \
    --test-mode stress \
    --duration 30 \
    --output test_results_stress.json \
    --verbose

echo ""

# Run thermal test
echo "🌡️ Running thermal stability test (20 seconds)..."
timeout 25s cargo run --release --bin trinity-conductor-test -- \
    --test-mode thermal \
    --duration 20 \
    --output test_results_thermal.json \
    --verbose

echo ""

# Display results summary
echo "📊 Test Results Summary:"
echo "======================="

for file in test_results_*.json; do
    if [ -f "$file" ]; then
        echo "📄 $file:"
        
        # Extract key metrics using jq if available, otherwise basic grep
        if command -v jq &> /dev/null; then
            peak_memory=$(jq -r '.statistics.peak_memory_gb // "N/A"' "$file")
            avg_temp=$(jq -r '.statistics.average_temperature_c // "N/A"' "$file")
            efficiency=$(jq -r '.statistics.memory_efficiency_score // "N/A"' "$file")
            
            echo "   Peak Memory: ${peak_memory}GB"
            echo "   Avg Temperature: ${avg_temp}°C"
            echo "   Efficiency Score: ${efficiency}%"
        else
            echo "   (Install jq for detailed metrics)"
        fi
        echo ""
    fi
done

echo "🎉 All tests completed!"
echo "📁 Results saved to test_results_*.json files"
echo ""
echo "📋 Next Steps:"
echo "   1. Review JSON results for detailed metrics"
echo "   2. Analyze memory efficiency scores"
echo "   3. Check thermal stability during stress tests"
echo "   4. Validate KV cache quantization performance"
echo "   5. Optimize layer offloading based on results"
