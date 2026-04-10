#!/bin/bash
# Trinity Quantization Bootstrap
# This script enters the correct distrobox container, installs dependencies, 
# and kicks off the multi-hour AWQ LongCat-Next compilation process.

LOG_FILE="/home/joshua/Workflow/desktop_trinity/trinity-genesis/longcat_omni_sidecar/quantizer/quantization_run.log"
PYTHON_SCRIPT="/home/joshua/Workflow/desktop_trinity/trinity-genesis/longcat_omni_sidecar/quantizer/quantize_longcat.py"
CONTAINER_NAME="vllm-quant"

echo "[*] Bootstrapping LongCat Extreme Compression Pipeline..." | tee -a "$LOG_FILE"
echo "[*] Waking up container: $CONTAINER_NAME" | tee -a "$LOG_FILE"

# Ensure container is running
distrobox enter "$CONTAINER_NAME" -- echo "Container active."

echo "[*] Syncing required quantization pip packages..." | tee -a "$LOG_FILE"
distrobox enter "$CONTAINER_NAME" -- sh -c '
    /opt/venv/bin/pip install diffusers llmcompressor auto-round --no-deps
' >> "$LOG_FILE" 2>&1

echo "[*] Applying ROCm environment and source patches..." | tee -a "$LOG_FILE"
export RADV_PERFMODE=nogttspill

distrobox enter "$CONTAINER_NAME" -- /opt/venv/bin/python /home/joshua/Workflow/desktop_trinity/trinity-genesis/longcat_omni_sidecar/quantizer/patch_env.py >> "$LOG_FILE" 2>&1

echo "[*] Initiating Background Pipeline! Monitor this file: $LOG_FILE" | tee -a "$LOG_FILE"
echo "--------------------------------------------------------" | tee -a "$LOG_FILE"

# Fire the offline script
distrobox enter "$CONTAINER_NAME" -- stdbuf -oL /opt/venv/bin/python "$PYTHON_SCRIPT" >> "$LOG_FILE" 2>&1 &
PID=$!

echo "[*] Pipeline ignited. Worker PID: $PID" | tee -a "$LOG_FILE"
echo "[!] You can watch the progress via: tail -f $LOG_FILE"
