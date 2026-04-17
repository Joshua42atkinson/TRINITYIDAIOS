#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# TRINITY ID AI OS — DAYDREAM XR Build Script
# ═══════════════════════════════════════════════════════════════════════════════
#
# Builds and deploys the DAYDREAM XR binary to a Meta Quest 3 headset.
#
# PREREQUISITES:
#   1. Android SDK installed ($ANDROID_SDK_ROOT)
#   2. Android NDK 25.x installed ($NDK_HOME)
#   3. cargo-ndk installed: cargo install cargo-ndk
#   4. Rust Android target: rustup target add aarch64-linux-android
#   5. libopenxr_loader.so from Meta OpenXR Mobile SDK placed in:
#      crates/trinity-daydream/android/app/src/main/jniLibs/arm64-v8a/
#   6. Quest 3 connected via USB+ADB or Wi-Fi ADB
#
# USAGE:
#   ./scripts/build_xr.sh          # Build + deploy
#   ./scripts/build_xr.sh --check  # Compile check only (no deploy)
#   ./scripts/build_xr.sh --desktop # Build desktop XR simulation
#
# ═══════════════════════════════════════════════════════════════════════════════

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
DAYDREAM_DIR="$PROJECT_ROOT/crates/trinity-daydream"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
GOLD='\033[0;33m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${GOLD}═══════════════════════════════════════════════════════════${NC}"
echo -e "${GOLD}  🌙 TRINITY DAYDREAM XR BUILD${NC}"
echo -e "${GOLD}═══════════════════════════════════════════════════════════${NC}"

# ── Parse arguments ──────────────────────────────────────────────────────────
MODE="${1:-build}"

case "$MODE" in
    --check)
        echo -e "${CYAN}📋 Compile check only (no deployment)${NC}"
        echo ""
        echo -e "${CYAN}Checking desktop build (regression)...${NC}"
        cargo check --features desktop -p trinity-daydream
        echo -e "${GREEN}✅ Desktop build OK${NC}"
        echo ""
        echo -e "${CYAN}Checking XR build...${NC}"
        cargo check --features xr -p trinity-daydream
        echo -e "${GREEN}✅ XR build OK${NC}"
        echo ""
        echo -e "${CYAN}Running protocol tests...${NC}"
        cargo test -p trinity-protocol
        echo -e "${GREEN}✅ All checks passed${NC}"
        exit 0
        ;;
    --desktop)
        echo -e "${CYAN}🖥️  Building desktop XR simulation...${NC}"
        echo "  Uses SteamVR/Monado OpenXR runtime if available,"
        echo "  otherwise falls back to WASD+mouse simulation."
        echo ""
        cargo build --release --bin daydream_xr --features xr -p trinity-daydream
        echo -e "${GREEN}✅ Desktop XR binary built${NC}"
        echo -e "  Run: ${CYAN}cargo run --release --bin daydream_xr --features xr -p trinity-daydream${NC}"
        exit 0
        ;;
esac

# ── Android Build ────────────────────────────────────────────────────────────

# Check prerequisites
if [ -z "${ANDROID_SDK_ROOT:-${ANDROID_HOME:-}}" ]; then
    echo -e "${RED}❌ ANDROID_SDK_ROOT not set. Install Android SDK first.${NC}"
    echo "   sudo apt install android-sdk  OR"
    echo "   Download from https://developer.android.com/studio"
    exit 1
fi

SDK_ROOT="${ANDROID_SDK_ROOT:-$ANDROID_HOME}"

# Find NDK
if [ -n "${NDK_HOME:-}" ]; then
    NDK_DIR="$NDK_HOME"
elif [ -d "$SDK_ROOT/ndk" ]; then
    NDK_DIR=$(ls -d "$SDK_ROOT/ndk/"* 2>/dev/null | sort -V | tail -1)
else
    echo -e "${RED}❌ Android NDK not found. Install via:${NC}"
    echo "   sdkmanager 'ndk;25.1.8937393'"
    exit 1
fi

echo -e "${CYAN}📱 Android SDK: ${NC}$SDK_ROOT"
echo -e "${CYAN}📱 Android NDK: ${NC}$NDK_DIR"

# Check cargo-ndk
if ! command -v cargo-ndk &>/dev/null; then
    echo -e "${RED}❌ cargo-ndk not installed. Installing...${NC}"
    cargo install cargo-ndk
fi

# Check OpenXR loader
LOADER_PATH="$DAYDREAM_DIR/android/app/src/main/jniLibs/arm64-v8a/libopenxr_loader.so"
if [ ! -f "$LOADER_PATH" ]; then
    echo -e "${RED}⚠ libopenxr_loader.so not found at:${NC}"
    echo "   $LOADER_PATH"
    echo ""
    echo "  Download from Meta OpenXR Mobile SDK:"
    echo "  https://developer.oculus.com/downloads/package/oculus-openxr-mobile-sdk/"
    echo ""
    echo "  Extract and copy:"
    echo "    cp OpenXR/Libs/Android/arm64-v8a/Release/libopenxr_loader.so \\"
    echo "       $LOADER_PATH"
    echo ""
    echo -e "${GOLD}Continuing build without loader (APK will not run on headset)${NC}"
fi

# ── Compile Rust for ARM64 ───────────────────────────────────────────────────
echo ""
echo -e "${CYAN}🔨 Compiling Rust for arm64-v8a...${NC}"

export ANDROID_SDK_ROOT="$SDK_ROOT"
export NDK_HOME="$NDK_DIR"

cargo ndk -t arm64-v8a \
    -o "$DAYDREAM_DIR/android/app/src/main/jniLibs" \
    build --release \
    --bin daydream_xr \
    --features xr \
    -p trinity-daydream

echo -e "${GREEN}✅ Native library compiled${NC}"

# ── Package APK (Gradle) ────────────────────────────────────────────────────
if [ -f "$DAYDREAM_DIR/android/gradlew" ]; then
    echo ""
    echo -e "${CYAN}📦 Building APK via Gradle...${NC}"
    cd "$DAYDREAM_DIR/android"
    ./gradlew assembleDebug
    
    APK_PATH="app/build/outputs/apk/debug/app-debug.apk"
    if [ -f "$APK_PATH" ]; then
        echo -e "${GREEN}✅ APK built: $APK_PATH${NC}"
        
        # Deploy if ADB is available and a device is connected
        if command -v adb &>/dev/null && adb devices | grep -q "device$"; then
            echo ""
            echo -e "${CYAN}🚀 Installing to headset...${NC}"
            adb install -r "$APK_PATH"
            echo -e "${GREEN}✅ Deployed to Quest 3${NC}"
        else
            echo -e "${GOLD}⚠ No headset connected via ADB. APK saved at:${NC}"
            echo "   $DAYDREAM_DIR/android/$APK_PATH"
        fi
    fi
else
    echo ""
    echo -e "${GOLD}⚠ No Gradle wrapper found. Android scaffold not yet created.${NC}"
    echo "  Native .so library is at:"
    echo "  $DAYDREAM_DIR/android/app/src/main/jniLibs/arm64-v8a/"
    echo ""
    echo "  To complete APK packaging, create the Android project scaffold"
    echo "  in: $DAYDREAM_DIR/android/"
fi

echo ""
echo -e "${GOLD}═══════════════════════════════════════════════════════════${NC}"
echo -e "${GOLD}  🌙 DAYDREAM XR BUILD COMPLETE${NC}"
echo -e "${GOLD}═══════════════════════════════════════════════════════════${NC}"
