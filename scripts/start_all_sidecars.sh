#!/bin/bash
# 🚀 Trinity Sidecar Launcher - Complete AI Process Isolation
# 
# Starts all AI sidecars for maximum safety, hot reloads, and process isolation
# Trinity main process becomes a lightweight orchestrator only

set -euo pipefail

# Script configuration
SCRIPT_NAME="Trinity Sidecar Launcher"
SCRIPT_VERSION="1.0.0"
LOG_DIR="/tmp/trinity_sidecars"
PID_DIR="/tmp/trinity_sidecars"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Sidecar configurations
declare -A SIDECARS=(
    ["vllm-omni"]="8000:90GB:GPU:vLLM-OMNI server for 97B Qwen Conductor and SDXL"
    ["trinity-sidecar-npu"]="8001:2GB:NPU:Personaplex-7B voice model with ORT-2 VitisAI"
    ["trinity-document-manager"]="8002:4GB:CPU:Document management with 128K context"
    ["trinity-data-pipeline"]="8003:8GB:CPU:Parquet data processing and embeddings"
    ["trinity-blueprint-reviewer"]="8004:4GB:CPU:ADDIE blueprint synthesis and review"
    ["trinity-music-ai"]="8005:2GB:CPU:MusicGPT CLI integration with OBS"
    ["trinity-agent-steward"]="8006:1GB:CPU:Educational music generation subagent"
    ["trinity-bevy-graphics"]="8007:4GB:GPU:Bevy graphics generation with vision models"
    ["trinity-skills"]="8008:2GB:CPU:AI skills processing (coder, writer, etc.)"
    ["trinity-memory"]="8009:2GB:CPU:Memory management and tracking"
)

# Logging function
log() {
    echo -e "${BLUE}[$SCRIPT_NAME]${NC} $1" | tee -a "$LOG_DIR/sidecar_launcher.log"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1" | tee -a "$LOG_DIR/sidecar_launcher.log"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1" | tee -a "$LOG_DIR/sidecar_launcher.log"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1" | tee -a "$LOG_DIR/sidecar_launcher.log"
}

# Create directories
create_directories() {
    mkdir -p "$LOG_DIR" "$PID_DIR"
    log "Created directories: $LOG_DIR, $PID_DIR"
}

# Check if sidecar is running
is_sidecar_running() {
    local sidecar_name="$1"
    local pid_file="$PID_DIR/$sidecar_name.pid"
    
    if [[ -f "$pid_file" ]]; then
        local pid=$(cat "$pid_file")
        if kill -0 "$pid" 2>/dev/null; then
            return 0
        else
            # Stale PID file
            rm -f "$pid_file"
            return 1
        fi
    fi
    return 1
}

# Check if sidecar binary exists
check_sidecar_binary() {
    local sidecar_name="$1"
    local binary_path="target/release/$sidecar_name"
    
    if [[ ! -f "$binary_path" ]]; then
        log_error "Binary not found: $binary_path"
        log "Please build the sidecar with: cargo build --release -p $sidecar_name"
        return 1
    fi
    
    return 0
}

# Start individual sidecar
start_sidecar() {
    local sidecar_name="$1"
    local port="$2"
    local memory_limit="$3"
    local processor_type="$4"
    local description="$5"
    
    log "🚀 Starting sidecar: $sidecar_name"
    log "   Port: $port"
    log "   Memory Limit: $memory_limit"
    log "   Processor: $processor_type"
    log "   Description: $description"
    
    if is_sidecar_running "$sidecar_name"; then
        log_warning "Sidecar $sidecar_name is already running"
        return 0
    fi
    
    if ! check_sidecar_binary "$sidecar_name"; then
        log_error "Cannot start $sidecar_name: binary not found"
        return 1
    fi
    
    # Set up environment variables
    local env_vars=()
    
    case "$sidecar_name" in
        "vllm-omni")
            env_vars+=(
                "HSA_OVERRIDE_GFX_VERSION=11.5.1"
                "ROCBLAS_USE_HIPBLASLT=1"
                "GPU_MAX_ALLOC_PERCENT=100"
                "VLLM_WORKER_MULTIPROC_METHOD=spawn"
                "VLLM_ATTENTION_BACKEND=FLASHINFER"
                "VLLM_USE_TRITON=0"
            )
            ;;
        "trinity-sidecar-npu")
            env_vars+=(
                "ORT_VITISAI_EP=1"
                "ORT_VITISAI_DEVICE_ID=0"
                "ORT_TENSORRT_ENGINE_CACHE_ENABLE=0"
            )
            ;;
        "trinity-document-manager")
            env_vars+=(
                "DATABASE_URL=postgresql://trinity:password@localhost:5432/trinity"
                "EMBEDDING_MODEL_PATH=models-consolidated/all-MiniLM-L6-v2"
            )
            ;;
    esac
    
    # Build command
    local binary_path="target/release/$sidecar_name"
    local log_file="$LOG_DIR/$sidecar_name.log"
    local pid_file="$PID_DIR/$sidecar_name.pid"
    
    # Set up command based on sidecar type
    local cmd_args=()
    
    case "$sidecar_name" in
        "vllm-omni")
            # vLLM-OMNI uses uv run
            cmd_args=(
                "uv" "run" "vllm" "serve" "models-consolidated/97B"
                "--omni" "--port" "$port" "--host" "0.0.0.0"
                "--gpu-memory-utilization" "0.90" "--enforce-eager"
                "--max-model-len" "32768" "--trust-remote-code"
                "--dtype" "float16" "--kv-cache-dtype" "fp8_e5m2"
                "--quantization" "fp8"
            )
            ;;
        "trinity-sidecar-npu")
            cmd_args=(
                "start" "--bind-addr" "127.0.0.1:$port"
                "--model-path" "models-consolidated/Personaplex-7B-v1-onnx/model.onnx"
                "--sample-rate" "16000" "--chunk-size" "1024"
                "--cpu-fallback" "true" "--log-level" "info"
            )
            ;;
        "trinity-document-manager")
            cmd_args=(
                "serve" "--port" "$port" "--host" "0.0.0.0"
            )
            ;;
        *)
            # Default arguments for other sidecars
            cmd_args=(
                "serve" "--port" "$port"
            )
            ;;
    esac
    
    # Start the sidecar
    log "🚀 Executing: $binary_path ${cmd_args[*]}"
    
    # Set environment variables and start
    (
        for env_var in "${env_vars[@]}"; do
            export "$env_var"
        done
        
        exec "$binary_path" "${cmd_args[@]}" > "$log_file" 2>&1 &
        echo $! > "$pid_file"
    )
    
    local sidecar_pid=$!
    
    # Wait a moment to check if it started successfully
    sleep 2
    
    if kill -0 "$sidecar_pid" 2>/dev/null; then
        log_success "✅ Sidecar $sidecar_name started (PID: $sidecar_pid)"
        return 0
    else
        log_error "❌ Sidecar $sidecar_name failed to start"
        rm -f "$pid_file"
        return 1
    fi
}

# Stop individual sidecar
stop_sidecar() {
    local sidecar_name="$1"
    local pid_file="$PID_DIR/$sidecar_name.pid"
    
    if ! is_sidecar_running "$sidecar_name"; then
        log_warning "Sidecar $sidecar_name is not running"
        return 0
    fi
    
    local pid=$(cat "$pid_file")
    log "🛑 Stopping sidecar $sidecar_name (PID: $pid)"
    
    # Send SIGTERM first
    kill -TERM "$pid" 2>/dev/null || true
    
    # Wait for graceful shutdown
    local wait_time=0
    local max_wait=30
    
    while [[ $wait_time -lt $max_wait ]]; do
        if ! kill -0 "$pid" 2>/dev/null; then
            log_success "✅ Sidecar $sidecar_name stopped gracefully"
            rm -f "$pid_file"
            return 0
        fi
        
        sleep 1
        wait_time=$((wait_time + 1))
        echo -n "."
    done
    
    echo
    log_warning "Graceful shutdown failed, force killing $sidecar_name"
    
    # Force kill
    kill -KILL "$pid" 2>/dev/null || true
    rm -f "$pid_file"
    
    log_success "✅ Sidecar $sidecar_name stopped forcefully"
}

# Check sidecar health
check_sidecar_health() {
    local sidecar_name="$1"
    local port="$2"
    
    if ! is_sidecar_running "$sidecar_name"; then
        echo "❌ $sidecar_name: Not running"
        return 1
    fi
    
    # Try health check based on sidecar type
    case "$sidecar_name" in
        "vllm-omni")
            if curl -s "http://localhost:$port/health" &>/dev/null; then
                echo "✅ $sidecar_name: Healthy"
                return 0
            else
                echo "⚠️ $sidecar_name: Unhealthy (health check failed)"
                return 1
            fi
            ;;
        "trinity-sidecar-npu")
            # NPU sidecar health check via RPC (placeholder)
            echo "✅ $sidecar_name: Running (health check not implemented)"
            return 0
            ;;
        *)
            # Generic check - just verify process is running
            echo "✅ $sidecar_name: Running"
            return 0
            ;;
    esac
}

# Show status of all sidecars
show_status() {
    echo "=== Trinity Sidecar Status ==="
    echo
    
    local running_count=0
    local total_count=${#SIDECARS[@]}
    
    for sidecar_name in "${!SIDECARS[@]}"; do
        local config="${SIDECARS[$sidecar_name]}"
        local port=$(echo "$config" | cut -d: -f1)
        
        if is_sidecar_running "$sidecar_name"; then
            echo "✅ $sidecar_name (Port: $port) - RUNNING"
            running_count=$((running_count + 1))
        else
            echo "❌ $sidecar_name (Port: $port) - STOPPED"
        fi
    done
    
    echo
    echo "Summary: $running_count/$total_count sidecars running"
    echo "=============================="
}

# Show logs for specific sidecar
show_logs() {
    local sidecar_name="$1"
    local lines="${2:-50}"
    local log_file="$LOG_DIR/$sidecar_name.log"
    
    if [[ ! -f "$log_file" ]]; then
        log_error "No log file found for $sidecar_name"
        return 1
    fi
    
    echo "=== Last $lines lines of $sidecar_name logs ==="
    tail -n "$lines" "$log_file"
    echo "=========================================="
    echo "Full log file: $log_file"
}

# Start all sidecars
start_all() {
    log "🚀 Starting all Trinity sidecars"
    
    local failed_sidecars=()
    
    for sidecar_name in "${!SIDECARS[@]}"; do
        local config="${SIDECARS[$sidecar_name]}"
        local port=$(echo "$config" | cut -d: -f1)
        local memory=$(echo "$config" | cut -d: -f2)
        local processor=$(echo "$config" | cut -d: -f3)
        local description=$(echo "$config" | cut -d: -f4-)
        
        if ! start_sidecar "$sidecar_name" "$port" "$memory" "$processor" "$description"; then
            failed_sidecars+=("$sidecar_name")
        fi
        
        # Small delay between startups to avoid resource conflicts
        sleep 1
    done
    
    if [[ ${#failed_sidecars[@]} -eq 0 ]]; then
        log_success "✅ All sidecars started successfully"
        echo
        echo "Sidecar endpoints:"
        for sidecar_name in "${!SIDECARS[@]}"; do
            local config="${SIDECARS[$sidecar_name]}"
            local port=$(echo "$config" | cut -d: -f1)
            echo "  $sidecar_name: http://localhost:$port"
        done
    else
        log_error "❌ Failed to start ${#failed_sidecars[@]} sidecars: ${failed_sidecars[*]}"
        return 1
    fi
}

# Stop all sidecars
stop_all() {
    log "🛑 Stopping all Trinity sidecars"
    
    for sidecar_name in "${!SIDECARS[@]}"; do
        stop_sidecar "$sidecar_name"
    done
    
    log_success "✅ All sidecars stopped"
}

# Restart all sidecars
restart_all() {
    log "🔄 Restarting all Trinity sidecars"
    
    stop_all
    sleep 2
    start_all
}

# Check health of all sidecars
check_health_all() {
    echo "=== Trinity Sidecar Health Check ==="
    echo
    
    local healthy_count=0
    local total_count=${#SIDECARS[@]}
    
    for sidecar_name in "${!SIDECARS[@]}"; do
        local config="${SIDECARS[$sidecar_name]}"
        local port=$(echo "$config" | cut -d: -f1)
        
        if check_sidecar_health "$sidecar_name" "$port"; then
            healthy_count=$((healthy_count + 1))
        fi
    done
    
    echo
    echo "Health Summary: $healthy_count/$total_count sidecars healthy"
    echo "====================================="
}

# Watch sidecar logs
watch_logs() {
    local sidecar_name="$1"
    
    if [[ -z "$sidecar_name" ]]; then
        log_error "Please specify a sidecar name to watch logs"
        echo "Available sidecars:"
        for sidecar in "${!SIDECARS[@]}"; do
            echo "  $sidecar"
        done
        return 1
    fi
    
    local log_file="$LOG_DIR/$sidecar_name.log"
    
    if [[ ! -f "$log_file" ]]; then
        log_error "No log file found for $sidecar_name"
        return 1
    fi
    
    log "📋 Watching logs for $sidecar_name (Ctrl+C to stop)"
    tail -f "$log_file"
}

# Main function
main() {
    # Create directories
    create_directories
    
    case "${1:-status}" in
        start)
            start_all
            ;;
        stop)
            stop_all
            ;;
        restart)
            restart_all
            ;;
        status)
            show_status
            ;;
        health)
            check_health_all
            ;;
        logs)
            show_logs "${2:-}" "${3:-50}"
            ;;
        watch)
            watch_logs "${2:-}"
            ;;
        start-sidecar)
            local sidecar_name="${2:-}"
            if [[ -z "$sidecar_name" ]]; then
                log_error "Please specify sidecar name"
                exit 1
            fi
            
            if [[ -z "${SIDECARS[$sidecar_name]:-}" ]]; then
                log_error "Unknown sidecar: $sidecar_name"
                echo "Available sidecars:"
                for sidecar in "${!SIDECARS[@]}"; do
                    echo "  $sidecar"
                done
                exit 1
            fi
            
            local config="${SIDECARS[$sidecar_name]}"
            local port=$(echo "$config" | cut -d: -f1)
            local memory=$(echo "$config" | cut -d: -f2)
            local processor=$(echo "$config" | cut -d: -f3)
            local description=$(echo "$config" | cut -d: -f4-)
            
            start_sidecar "$sidecar_name" "$port" "$memory" "$processor" "$description"
            ;;
        stop-sidecar)
            stop_sidecar "${2:-}"
            ;;
        *)
            echo "Usage: $0 {start|stop|restart|status|health|logs|watch|start-sidecar|stop-sidecar} [sidecar_name] [lines]"
            echo
            echo "Commands:"
            echo "  start                    - Start all sidecars"
            echo "  stop                     - Stop all sidecars"
            echo "  restart                  - Restart all sidecars"
            echo "  status                   - Show status of all sidecars"
            echo "  health                   - Check health of all sidecars"
            echo "  logs [sidecar] [lines]   - Show logs for sidecar"
            echo "  watch [sidecar]          - Watch logs for sidecar in real-time"
            echo "  start-sidecar <name>     - Start specific sidecar"
            echo "  stop-sidecar <name>      - Stop specific sidecar"
            echo
            echo "Available sidecars:"
            for sidecar in "${!SIDECARS[@]}"; do
                echo "  $sidecar"
            done
            exit 1
            ;;
    esac
}

# Script entry point
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
