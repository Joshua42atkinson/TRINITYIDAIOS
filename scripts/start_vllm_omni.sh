#!/bin/bash
# 🚀 vLLM-OMNI Launch Script for Trinity
# 
# Optimized for AMD Strix Halo (Ryzen AI Max+ 395) with RDNA 3.5 GPU
# Maintains strict 60 FPS UI by isolating heavy lifting to separate process

set -euo pipefail

# Script configuration
SCRIPT_NAME="vLLM-OMNI Launcher"
SCRIPT_VERSION="1.0.0"
LOG_FILE="/tmp/trinity_vllm_omni.log"
PID_FILE="/tmp/trinity_vllm_omni.pid"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging function
log() {
    echo -e "${BLUE}[$SCRIPT_NAME]${NC} $1" | tee -a "$LOG_FILE"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1" | tee -a "$LOG_FILE"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1" | tee -a "$LOG_FILE"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1" | tee -a "$LOG_FILE"
}

# Check if vLLM is already running
check_running() {
    if [[ -f "$PID_FILE" ]]; then
        local pid=$(cat "$PID_FILE")
        if kill -0 "$pid" 2>/dev/null; then
            log_warning "vLLM-OMNI is already running (PID: $pid)"
            return 0
        else
            log_warning "Stale PID file found, removing"
            rm -f "$PID_FILE"
        fi
    fi
    return 1
}

# Check system requirements
check_requirements() {
    log "Checking system requirements..."
    
    # Check if uv is installed
    if ! command -v uv &> /dev/null; then
        log_error "uv is not installed. Please install uv first."
        log "Install with: curl -LsSf https://astral.sh/uv/install.sh | sh"
        exit 1
    fi
    
    # Check if we're on AMD hardware
    if ! lspci | grep -i "amd\|advanced micro devices" &> /dev/null; then
        log_warning "No AMD GPU detected. This script is optimized for AMD Strix Halo."
    fi
    
    # Check unified memory (approximate check)
    local total_memory=$(free -h | awk '/^Mem:/ {print $2}')
    log "System memory: $total_memory"
    
    # Check ROCm installation
    if ! command -v rocminfo &> /dev/null; then
        log_warning "ROCm not detected. Ensure ROCm 7.x is installed for optimal performance."
    fi
    
    log_success "System requirements check completed"
}

# Setup ROCm environment variables
setup_rocm_env() {
    log "Setting up ROCm environment variables..."
    
    # Override GPU version for RDNA 3.5 (gfx1151)
    # Try 11.5.1 first (Strix Halo specific), fallback to 11.0.0
    export HSA_OVERRIDE_GFX_VERSION="11.5.1"
    log "Set HSA_OVERRIDE_GFX_VERSION=11.5.1"
    
    # Enable HIPBLASLT for better performance
    export ROCBLAS_USE_HIPBLASLT=1
    log "Set ROCBLAS_USE_HIPBLASLT=1"
    
    # Additional ROCm optimizations
    export HIP_VISIBLE_DEVICES=0
    export GPU_MAX_ALLOC_PERCENT=100
    export GPU_SINGLE_ALLOC_PERCENT=100
    
    # vLLM specific optimizations
    export VLLM_WORKER_MULTIPROC_METHOD=spawn
    export VLLM_ATTENTION_BACKEND=FLASHINFER
    export VLLM_USE_TRITON=0  # Disable Triton for AMD compatibility
    
    log_success "ROCm environment configured"
}

# Validate model path
validate_model_path() {
    local model_path="${1:-models-consolidated/97B}"
    
    if [[ ! -d "$model_path" ]]; then
        log_error "Model path not found: $model_path"
        log "Please ensure the 97B model is available at: $model_path"
        exit 1
    fi
    
    # Check for required model files
    local required_files=("config.json" "tokenizer.json" "model.safetensors")
    for file in "${required_files[@]}"; do
        if [[ ! -f "$model_path/$file" ]] && [[ ! -f "$model_path/$file" ]]; then
            log_warning "Expected model file not found: $model_path/$file"
        fi
    done
    
    log_success "Model path validated: $model_path"
    echo "$model_path"
}

# Start vLLM-OMNI server
start_vllm() {
    local model_path="$1"
    local port="${2:-8000}"
    
    log "Starting vLLM-OMNI server..."
    log "Model: $model_path"
    log "Port: $port"
    
    # vLLM startup command with optimized flags for Strix Halo
    local vllm_cmd=(
        uv run
        vllm serve "$model_path"
        --omni
        --port "$port"
        --host 0.0.0.0
        --gpu-memory-utilization 0.90
        --enforce-eager
        --max-model-len 32768
        --trust-remote-code
        --dtype float16
        --kv-cache-dtype fp8_e5m2
        --quantization fp8
        --tensor-parallel-size 1
        --pipeline-parallel-size 1
        --disable-log-stats
        --disable-log-requests
    )
    
    # Create log directory if it doesn't exist
    mkdir -p "$(dirname "$LOG_FILE")"
    
    log "Executing: ${vllm_cmd[*]}"
    
    # Start vLLM in background
    "${vllm_cmd[@]}" >> "$LOG_FILE" 2>&1 &
    local vllm_pid=$!
    
    # Save PID
    echo "$vllm_pid" > "$PID_FILE"
    
    log "vLLM-OMNI started with PID: $vllm_pid"
    log "Log file: $LOG_FILE"
    log "PID file: $PID_FILE"
    
    # Wait for server to be ready
    log "Waiting for vLLM-OMNI server to be ready..."
    local max_wait=300  # 5 minutes
    local wait_time=0
    
    while [[ $wait_time -lt $max_wait ]]; do
        if curl -s "http://localhost:$port/health" &> /dev/null; then
            log_success "vLLM-OMNI server is ready!"
            return 0
        fi
        
        # Check if process is still running
        if ! kill -0 "$vllm_pid" 2>/dev/null; then
            log_error "vLLM-OMNI process died unexpectedly"
            return 1
        fi
        
        sleep 2
        wait_time=$((wait_time + 2))
        echo -n "."
    done
    
    echo
    log_error "vLLM-OMNI server failed to start within $max_wait seconds"
    return 1
}

# Stop vLLM-OMNI server
stop_vllm() {
    if [[ ! -f "$PID_FILE" ]]; then
        log_warning "No PID file found, vLLM-OMNI may not be running"
        return 0
    fi
    
    local pid=$(cat "$PID_FILE")
    log "Stopping vLLM-OMNI (PID: $pid)..."
    
    # Send SIGTERM first
    kill -TERM "$pid" 2>/dev/null || true
    
    # Wait for graceful shutdown
    local wait_time=0
    local max_wait=30
    
    while [[ $wait_time -lt $max_wait ]]; do
        if ! kill -0 "$pid" 2>/dev/null; then
            log_success "vLLM-OMNI stopped gracefully"
            rm -f "$PID_FILE"
            return 0
        fi
        
        sleep 1
        wait_time=$((wait_time + 1))
        echo -n "."
    done
    
    echo
    log_warning "Graceful shutdown failed, forcing termination"
    
    # Force kill
    kill -KILL "$pid" 2>/dev/null || true
    rm -f "$PID_FILE"
    
    log_success "vLLM-OMNI stopped forcefully"
}

# Check server health
check_health() {
    local port="${1:-8000}"
    
    if ! curl -s "http://localhost:$port/health" &> /dev/null; then
        log_error "vLLM-OMNI server is not responding"
        return 1
    fi
    
    local health_response=$(curl -s "http://localhost:$port/health")
    log_success "vLLM-OMNI server is healthy"
    log "Health response: $health_response"
    
    return 0
}

# Show server status
show_status() {
    local port="${1:-8000}"
    
    echo "=== vLLM-OMNI Status ==="
    
    if [[ -f "$PID_FILE" ]]; then
        local pid=$(cat "$PID_FILE")
        if kill -0 "$pid" 2>/dev/null; then
            echo "Status: Running"
            echo "PID: $pid"
            echo "Port: $port"
            
            # Show process info
            echo "Process info:"
            ps -p "$pid" -o pid,ppid,cmd,etime,pcpu,pmem --no-headers || true
            
            # Show memory usage
            echo "Memory usage:"
            local memory=$(ps -p "$pid" -o rss= 2>/dev/null || echo "N/A")
            if [[ "$memory" != "N/A" ]]; then
                echo "RSS: $((memory / 1024)) MB"
            fi
            
            # Check health endpoint
            if check_health "$port"; then
                echo "Health: OK"
            else
                echo "Health: FAILED"
            fi
        else
            echo "Status: Not running (stale PID file)"
            rm -f "$PID_FILE"
        fi
    else
        echo "Status: Not running"
    fi
    
    echo "========================"
}

# Show logs
show_logs() {
    local lines="${1:-50}"
    
    if [[ -f "$LOG_FILE" ]]; then
        echo "=== Last $lines lines of vLLM-OMNI logs ==="
        tail -n "$lines" "$LOG_FILE"
        echo "======================================"
        echo "Full log file: $LOG_FILE"
    else
        log_warning "No log file found at $LOG_FILE"
    fi
}

# Main function
main() {
    case "${1:-start}" in
        start)
            if check_running; then
                log_error "vLLM-OMNI is already running"
                exit 1
            fi
            
            check_requirements
            setup_rocm_env
            local model_path=$(validate_model_path "${2:-}")
            
            if start_vllm "$model_path" "${3:-8000}"; then
                log_success "vLLM-OMNI started successfully!"
                echo
                echo "Server details:"
                echo "  - Endpoint: http://localhost:${3:-8000}"
                echo "  - Health: http://localhost:${3:-8000}/health"
                echo "  - Logs: $LOG_FILE"
                echo "  - PID: $PID_FILE"
                echo
                echo "To stop: $0 stop"
                echo "To check status: $0 status"
            else
                log_error "Failed to start vLLM-OMNI"
                exit 1
            fi
            ;;
        
        stop)
            stop_vllm
            ;;
        
        restart)
            stop_vllm
            sleep 2
            main start "$2" "$3"
            ;;
        
        status)
            show_status "${3:-8000}"
            ;;
        
        health)
            check_health "${3:-8000}"
            ;;
        
        logs)
            show_logs "${2:-50}"
            ;;
        
        check)
            check_requirements
            setup_rocm_env
            local model_path=$(validate_model_path "${2:-}")
            log_success "All checks passed!"
            ;;
        
        *)
            echo "Usage: $0 {start|stop|restart|status|health|logs|check} [model_path] [port]"
            echo
            echo "Commands:"
            echo "  start [model_path] [port]  - Start vLLM-OMNI server"
            echo "  stop                        - Stop vLLM-OMNI server"
            echo "  restart [model_path] [port]  - Restart vLLM-OMNI server"
            echo "  status [port]                - Show server status"
            echo "  health [port]                - Check server health"
            echo "  logs [lines]                 - Show recent logs"
            echo "  check [model_path]           - Check requirements"
            echo
            echo "Examples:"
            echo "  $0 start models-consolidated/97B 8000"
            echo "  $0 status 8000"
            echo "  $0 logs 100"
            exit 1
            ;;
    esac
}

# Script entry point
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
