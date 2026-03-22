#!/bin/bash
# 📦 Build Trinity Genesis AppImage
# Creates a single-file executable like LM Studio

set -e

echo "🚀 Building Trinity Genesis AppImage..."

# Configuration
APP_NAME="Trinity-Genesis"
APP_DIR="AppDir"
BUILD_DIR="build-appimage"
VERSION=${VERSION:-"0.1.0"}

# Clean and setup
rm -rf "${BUILD_DIR}"
mkdir -p "${BUILD_DIR}"
cd "${BUILD_DIR}"

echo "🔨 Building release binaries..."
cd ..
cargo build --release --package trinity-ui
cargo build --release --package trinity-cli
cargo build --release --package trinity
cd "${BUILD_DIR}"

echo "📁 Setting up AppDir structure..."
mkdir -p "${APP_DIR}/usr/bin"
mkdir -p "${APP_DIR}/usr/share/applications"
mkdir -p "${APP_DIR}/usr/share/icons/hicolor/256x256/apps"
mkdir -p "${APP_DIR}/usr/share/trinity-genesis"

echo "📋 Copying binaries..."
cp ../target/release/trinity-ui "${APP_DIR}/usr/bin/"
cp ../target/release/trinity "${APP_DIR}/usr/bin/"

echo "🎨 Creating desktop integration..."

# Desktop entry
cat > "${APP_DIR}/usr/share/applications/trinity-genesis.desktop" << 'EOF'
[Desktop Entry]
Name=Trinity Genesis
Comment=AI-Powered Learning Companion
Exec=trinity-ui
Icon=trinity-genesis
Type=Application
Categories=Education;Development;Game;
Terminal=false
EOF

cp "${APP_DIR}/usr/share/applications/trinity-genesis.desktop" "${APP_DIR}/"

# Icon (placeholder - you should add a real PNG icon)
echo "⚠️  Note: Add a real 256x256 PNG icon at assets/trinity-icon.png"
if [ -f ../assets/icons/trinity-icon.png ]; then
    cp ../assets/icons/trinity-icon.png "${APP_DIR}/usr/share/icons/hicolor/256x256/apps/trinity-genesis.png"
    cp ../assets/icons/trinity-icon.png "${APP_DIR}/trinity-genesis.png"
else
    # Create a simple placeholder icon
    echo "Creating placeholder icon..."
    convert -size 256x256 xc:blue "${APP_DIR}/trinity-genesis.png" 2>/dev/null || \
    echo "Please install ImageMagick or add a real icon"
fi

# AppRun script (entry point for AppImage)
cat > "${APP_DIR}/AppRun" << 'EOF'
#!/bin/bash
# AppImage entry point

# Get the directory where the AppImage is located
APPDIR="$(dirname "$(readlink -f "$0")")"

# Set up environment
export PATH="${APPDIR}/usr/bin:${PATH}"
export LD_LIBRARY_PATH="${APPDIR}/usr/lib:${LD_LIBRARY_PATH}"

# Check for LM Studio and notify user
if curl -s http://localhost:1234/v1/models > /dev/null 2>&1; then
    echo "✅ LM Studio detected - AI features enabled" >&2
else
    echo "💡 Tip: Install LM Studio for AI features (https://lmstudio.ai/)" >&2
fi

# Launch the application
exec "${APPDIR}/usr/bin/trinity-ui" "$@"
EOF

chmod +x "${APP_DIR}/AppRun"

echo "📄 Creating metadata..."

# Create metadata for appimage-builder or linuxdeploy
mkdir -p "${APP_DIR}/META-INF"
cat > "${APP_DIR}/META-INF/metadata.xml" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<application>
  <id>com.trinity-genesis.app</id>
  <name>Trinity Genesis</name>
  <version>${VERSION}</version>
  <description>AI-Powered Learning Companion</description>
  <icon>trinity-genesis.png</icon>
</application>
EOF

echo "🔧 Checking for AppImage tools..."

# Check for linuxdeploy
if ! command -v linuxdeploy &> /dev/null; then
    echo "📥 Downloading linuxdeploy..."
    wget -q https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage -O linuxdeploy
    chmod +x linuxdeploy
    LINUXDEPLOY="./linuxdeploy"
else
    LINUXDEPLOY="linuxdeploy"
fi

echo "📦 Building AppImage..."

# Build the AppImage
${LINUXDEPLOY} \
    --appdir "${APP_DIR}" \
    --desktop-file="${APP_DIR}/trinity-genesis.desktop" \
    --icon-file="${APP_DIR}/trinity-genesis.png" \
    --output appimage \
    --executable="${APP_DIR}/usr/bin/trinity-ui"

# Rename to include version
if [ -f Trinity-Genesis-x86_64.AppImage ]; then
    mv Trinity-Genesis-x86_64.AppImage "../Trinity-Genesis-${VERSION}-x86_64.AppImage"
    echo "✅ AppImage created: Trinity-Genesis-${VERSION}-x86_64.AppImage"
elif [ -f ./*.AppImage ]; then
    mv ./*.AppImage "../Trinity-Genesis-${VERSION}-x86_64.AppImage"
    echo "✅ AppImage created: Trinity-Genesis-${VERSION}-x86_64.AppImage"
else
    echo "⚠️  AppImage build may have failed or created unexpected filename"
    ls -la *.AppImage 2>/dev/null || echo "No AppImage found"
fi

cd ..

echo ""
echo "🎉 AppImage build complete!"
echo ""
echo "📦 Output: Trinity-Genesis-${VERSION}-x86_64.AppImage"
echo ""
echo "🎯 Usage:"
echo "  chmod +x Trinity-Genesis-${VERSION}-x86_64.AppImage"
echo "  ./Trinity-Genesis-${VERSION}-x86_64.AppImage"
echo ""
echo "📤 To distribute:"
echo "  1. Upload to GitHub Releases"
echo "  2. Share direct download link"
echo "  3. Users just download, chmod +x, and run!"
echo ""
echo "💡 For even easier sharing, use:"
echo "  ./scripts/package-for-friend.sh"
