#!/bin/bash

# Trinity Startup Script - Optimized for Testing

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
exec "$ROOT_DIR/run_trinity.sh"
