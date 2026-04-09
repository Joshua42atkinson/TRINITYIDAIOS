#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# TRINITY ID AI OS — launch_longcat.sh
# ═══════════════════════════════════════════════════════════════════════════════
#
# PURPOSE:  Launch LongCat-Next sidecar — callable from Trinity's web UI
# USAGE:    bash scripts/launch/launch_longcat.sh [start|stop|status]
#
# This script manages the LongCat-Next process lifecycle via PID file,
# enabling the Rust backend to start/stop inference from HTTP endpoints.
# ═══════════════════════════════════════════════════════════════════════════════

set -uo pipefail

PIDFILE="/tmp/trinity_longcat.pid"
LOGFILE="/tmp/trinity_longcat.log"
SCRIPT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
ACTION="${1:-start}"

case "$ACTION" in
  start)
    # Check if already running
    if [ -f "$PIDFILE" ]; then
      PID=$(cat "$PIDFILE")
      if kill -0 "$PID" 2>/dev/null; then
        echo '{"status":"already_running","pid":'$PID'}'
        exit 0
      fi
      rm -f "$PIDFILE"
    fi

    # Check if model exists
    MODEL_DIR="$HOME/trinity-models/sglang/LongCat-Next"
    if [ ! -d "$MODEL_DIR" ]; then
      echo '{"status":"error","message":"Model not found at '"$MODEL_DIR"'"}'
      exit 1
    fi

    echo "🐱 Starting LongCat-Next via distrobox..." >> "$LOGFILE"

    # Launch in background inside distrobox
    cd "$SCRIPT_DIR"
    nohup distrobox enter sglang-engine -- bash ./longcat_omni_sidecar/launch_engine.sh \
      >> "$LOGFILE" 2>&1 &
    
    BGPID=$!
    echo "$BGPID" > "$PIDFILE"
    
    echo '{"status":"starting","pid":'$BGPID',"log":"'"$LOGFILE"'"}'
    ;;

  stop)
    if [ ! -f "$PIDFILE" ]; then
      # Try to find by port
      PID=$(lsof -ti:8010 2>/dev/null | head -1)
      if [ -z "$PID" ]; then
        echo '{"status":"not_running"}'
        exit 0
      fi
    else
      PID=$(cat "$PIDFILE")
    fi

    if kill -0 "$PID" 2>/dev/null; then
      # Kill the process tree
      pkill -P "$PID" 2>/dev/null
      kill "$PID" 2>/dev/null
      
      # Also kill anything on port 8010
      lsof -ti:8010 2>/dev/null | xargs -r kill 2>/dev/null
      
      rm -f "$PIDFILE"
      echo '{"status":"stopped","pid":'$PID'}'
    else
      rm -f "$PIDFILE"
      echo '{"status":"not_running"}'
    fi
    ;;

  status)
    if [ -f "$PIDFILE" ]; then
      PID=$(cat "$PIDFILE")
      if kill -0 "$PID" 2>/dev/null; then
        # Check health endpoint
        HEALTH=$(curl -s --connect-timeout 2 http://127.0.0.1:8010/health 2>/dev/null)
        if [ $? -eq 0 ]; then
          echo '{"status":"running","pid":'$PID',"health":'"$HEALTH"'}'
        else
          echo '{"status":"loading","pid":'$PID'}'
        fi
      else
        rm -f "$PIDFILE"
        echo '{"status":"not_running"}'
      fi
    else
      # Check port directly
      if curl -s --connect-timeout 1 http://127.0.0.1:8010/health >/dev/null 2>&1; then
        echo '{"status":"running","pid":"unknown"}'
      else
        echo '{"status":"not_running"}'
      fi
    fi
    ;;
    
  *)
    echo '{"status":"error","message":"Usage: launch_longcat.sh [start|stop|status]"}'
    exit 1
    ;;
esac
