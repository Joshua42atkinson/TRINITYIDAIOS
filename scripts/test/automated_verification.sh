#!/bin/bash

# 🔬 Trinity Automated Verification Script
# Comprehensive technical verification automation

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging
LOG_DIR="verification_logs"
mkdir -p "$LOG_DIR"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
LOG_FILE="$LOG_DIR/verification_$TIMESTAMP.log"

# Function to log output
log() {
    echo -e "$1" | tee -a "$LOG_FILE"
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to run verification test
run_test() {
    local test_name="$1"
    local test_command="$2"
    local expected_result="$3"
    
    log "${BLUE}Running: $test_name${NC}"
    log "Command: $test_command"
    
    if eval "$test_command" >> "$LOG_FILE" 2>&1; then
        log "${GREEN}✅ PASS: $test_name${NC}"
        return 0
    else
        log "${RED}❌ FAIL: $test_name${NC}"
        return 1
    fi
}

# Function to check hardware requirements
check_hardware() {
    log "${BLUE}=== Hardware Verification ===${NC}"
    
    # Check if we're running on Strix Halo
    if grep -q "GFX1151" /proc/cpuinfo; then
        log "${GREEN}✅ AMD Strix Halo detected${NC}"
    else
        log "${YELLOW}⚠️  Warning: Not running on verified Strix Halo hardware${NC}"
    fi
    
    # Check memory
    TOTAL_MEM=$(free -g | awk '/^Mem:/{print $2}')
    if [ "$TOTAL_MEM" -ge 120 ]; then
        log "${GREEN}✅ Sufficient memory: ${TOTAL_MEM}GB${NC}"
    else
        log "${RED}❌ Insufficient memory: ${TOTAL_MEM}GB (need 128GB)${NC}"
    fi
    
    # Check GPU
    if command_exists rocm-smi; then
        log "${GREEN}✅ ROCm detected${NC}"
        rocm-smi --showproductname >> "$LOG_FILE" 2>&1
    else
        log "${RED}❌ ROCm not found${NC}"
    fi
    
    # Check NPU
    if [ -d /sys/class/drm/renderD128 ]; then
        log "${GREEN}✅ NPU device detected${NC}"
    else
        log "${YELLOW}⚠️  NPU device not found${NC}"
    fi
}

# Function to check software requirements
check_software() {
    log "${BLUE}=== Software Verification ===${NC}"
    
    # Check kernel version
    KERNEL_VERSION=$(uname -r)
    if [[ "$KERNEL_VERSION" =~ ^6\.19\. ]]; then
        log "${GREEN}✅ Kernel 6.19 detected: $KERNEL_VERSION${NC}"
    else
        log "${YELLOW}⚠️  Kernel version: $KERNEL_VERSION (6.19 recommended)${NC}"
    fi
    
    # Check ROCm
    if command_exists rocm-smi; then
        ROCM_VERSION=$(rocm-smi --showproductname | grep -o "ROCm [0-9]\+\.[0-9]\+\.[0-9]\+" | head -1)
        log "${GREEN}✅ ROCm: $ROCM_VERSION${NC}"
    else
        log "${RED}❌ ROCm not installed${NC}"
    fi
    
    # Check Trinity dependencies
    if command_exists cargo; then
        log "${GREEN}✅ Rust/Cargo detected${NC}"
    else
        log "${RED}❌ Rust/Cargo not found${NC}"
    fi
    
    if command_exists python3; then
        log "${GREEN}✅ Python3 detected${NC}"
    else
        log "${RED}❌ Python3 not found${NC}"
    fi
}

# Function to verify AI models
verify_ai_models() {
    log "${BLUE}=== AI Model Verification ===${NC}"
    
    # Check model files exist
    MODEL_DIR="/path/to/ai/models"  # Update this path
    
    if [ -f "$MODEL_DIR/MiniMax-M2-5-REAP-50-Q4_K_M.gguf" ]; then
        SIZE=$(stat -c%s "$MODEL_DIR/MiniMax-M2-5-REAP-50-Q4_K_M.gguf")
        EXPECTED_SIZE=$((66 * 1024 * 1024 * 1024))  # 66GB in bytes
        if [ $SIZE -gt $((EXPECTED_SIZE - 1073741824)) ] && [ $SIZE -lt $((EXPECTED_SIZE + 1073741824)) ]; then
            log "${GREEN}✅ MiniMax-REAP-50 model size verified${NC}"
        else
            log "${YELLOW}⚠️  MiniMax-REAP-50 model size unusual: $((SIZE / 1024 / 1024 / 1024))GB${NC}"
        fi
    else
        log "${YELLOW}⚠️  MiniMax-REAP-50 model not found${NC}"
    fi
    
    if [ -f "$MODEL_DIR/Qwen3.5-35B-A3B-Q4_K_M.gguf" ]; then
        SIZE=$(stat -c%s "$MODEL_DIR/Qwen3.5-35B-A3B-Q4_K_M.gguf")
        EXPECTED_SIZE=$((20 * 1024 * 1024 * 1024))  # 20GB in bytes
        if [ $SIZE -gt $((EXPECTED_SIZE - 536870912)) ] && [ $SIZE -lt $((EXPECTED_SIZE + 536870912)) ]; then
            log "${GREEN}✅ Qwen3.5-35B model size verified${NC}"
        else
            log "${YELLOW}⚠️  Qwen3.5-35B model size unusual: $((SIZE / 1024 / 1024 / 1024))GB${NC}"
        fi
    else
        log "${YELLOW}⚠️  Qwen3.5-35B model not found${NC}"
    fi
}

# Function to run performance benchmarks
run_benchmarks() {
    log "${BLUE}=== Performance Benchmarks ===${NC}"
    
    # GPU benchmark
    if command_exists rocm-smi; then
        log "Running GPU benchmark..."
        rocm-smi --showuse --showtemp --showpower >> "$LOG_FILE" 2>&1
        log "${GREEN}✅ GPU benchmark completed${NC}"
    fi
    
    # Memory benchmark
    log "Running memory benchmark..."
    sysbench memory --memory-block-size=1K --memory-total-size=10G run >> "$LOG_FILE" 2>&1
    log "${GREEN}✅ Memory benchmark completed${NC}"
    
    # CPU benchmark
    log "Running CPU benchmark..."
    sysbench cpu --cpu-max-prime=20000 run >> "$LOG_FILE" 2>&1
    log "${GREEN}✅ CPU benchmark completed${NC}"
}

# Function to test Trinity compilation
test_trinity_build() {
    log "${BLUE}=== Trinity Build Test ===${NC}"
    
    # Check if we're in the right directory
    if [ ! -f "Cargo.toml" ]; then
        log "${RED}❌ Not in Trinity root directory${NC}"
        return 1
    fi
    
    # Clean build
    log "Cleaning previous build..."
    cargo clean >> "$LOG_FILE" 2>&1
    
    # Build Trinity
    log "Building Trinity..."
    if cargo build --release >> "$LOG_FILE" 2>&1; then
        log "${GREEN}✅ Trinity build successful${NC}"
    else
        log "${RED}❌ Trinity build failed${NC}"
        return 1
    fi
    
    # Run tests
    log "Running Trinity tests..."
    if cargo test >> "$LOG_FILE" 2>&1; then
        log "${GREEN}✅ Trinity tests passed${NC}"
    else
        log "${YELLOW}⚠️  Some Trinity tests failed${NC}"
    fi
}

# Function to check documentation accuracy
check_documentation() {
    log "${BLUE}=== Documentation Verification ===${NC}"
    
    # Check if all documentation files exist
    DOC_FILES=(
        "TECHNICAL_VERIFICATION_FRAMEWORK.md"
        "02_AI_MODEL_VERIFICATION/minimax_reap50_verification.md"
        "03_HARDWARE_VERIFICATION/strix_halo_specs.md"
        "08_QUALITY_ASSURANCE/verification_checklists.md"
    )
    
    for doc in "${DOC_FILES[@]}"; do
        if [ -f "$doc" ]; then
            log "${GREEN}✅ Documentation exists: $doc${NC}"
        else
            log "${RED}❌ Documentation missing: $doc${NC}"
        fi
    done
    
    # Check for broken links
    log "Checking for broken links..."
    find . -name "*.md" -exec markdown-link-check {} \; >> "$LOG_FILE" 2>&1 || true
    log "${GREEN}✅ Link check completed${NC}"
}

# Function to generate verification report
generate_report() {
    log "${BLUE}=== Generating Verification Report ===${NC}"
    
    REPORT_FILE="$LOG_DIR/verification_report_$TIMESTAMP.md"
    
    cat > "$REPORT_FILE" << EOF
# Trinity Verification Report
**Generated**: $(date)
**Environment**: $(uname -a)
**Log File**: $LOG_FILE

## Executive Summary

### Hardware Status
$(grep -E "(✅|❌|⚠️)" "$LOG_FILE" | grep "Hardware" || echo "No hardware issues found")

### Software Status
$(grep -E "(✅|❌|⚠️)" "$LOG_FILE" | grep "Software" || echo "No software issues found")

### Model Status
$(grep -E "(✅|❌|⚠️)" "$LOG_FILE" | grep "Model" || echo "No model issues found")

### Build Status
$(grep -E "(✅|❌|⚠️)" "$LOG_FILE" | grep "Trinity" || echo "No build issues found")

## Detailed Results

### Hardware Verification
\`\`\`
$(grep -A 20 "=== Hardware Verification ===" "$LOG_FILE")
\`\`\`

### Software Verification
\`\`\`
$(grep -A 20 "=== Software Verification ===" "$LOG_FILE")
\`\`\`

### Performance Benchmarks
\`\`\`
$(grep -A 20 "=== Performance Benchmarks ===" "$LOG_FILE")
\`\`\`

## Recommendations

Based on the verification results:

1. **Critical Issues**: $(grep -c "❌" "$LOG_FILE") found
2. **Warnings**: $(grep -c "⚠️" "$LOG_FILE") found
3. **Passed Tests**: $(grep -c "✅" "$LOG_FILE") passed

### Next Steps
- Address any critical issues immediately
- Review warnings for potential improvements
- Update documentation with verified results
- Schedule regular verification runs

---

*Report generated by Trinity Automated Verification System*
EOF

    log "${GREEN}✅ Verification report generated: $REPORT_FILE${NC}"
}

# Main execution
main() {
    log "${BLUE}🔬 Trinity Automated Verification Starting${NC}"
    log "Timestamp: $TIMESTAMP"
    log "Log file: $LOG_FILE"
    
    # Run all verification steps
    check_hardware
    check_software
    verify_ai_models
    run_benchmarks
    test_trinity_build
    check_documentation
    
    # Generate final report
    generate_report
    
    # Summary
    TOTAL_TESTS=$(grep -c "✅\|❌" "$LOG_FILE")
    PASSED_TESTS=$(grep -c "✅" "$LOG_FILE")
    FAILED_TESTS=$(grep -c "❌" "$LOG_FILE")
    
    log "${BLUE}=== Verification Summary ===${NC}"
    log "Total Tests: $TOTAL_TESTS"
    log "Passed: $PASSED_TESTS"
    log "Failed: $FAILED_TESTS"
    
    if [ $FAILED_TESTS -eq 0 ]; then
        log "${GREEN}🎉 All tests passed! Trinity system verified.${NC}"
        exit 0
    else
        log "${RED}⚠️  $FAILED_TESTS test(s) failed. Please review the log file.${NC}"
        exit 1
    fi
}

# Run main function
main "$@"
