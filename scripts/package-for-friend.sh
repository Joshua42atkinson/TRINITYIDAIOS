#!/bin/bash
# 📦 Package Trinity for Friend Distribution
# Creates a self-contained package with everything needed

set -e

echo "🚀 Packaging Trinity for Friend Testing..."

# Configuration
VERSION="0.1.0"
PACKAGE_NAME="trinity-genesis-${VERSION}-friend-test"
BUILD_DIR="./package-build"
DIST_DIR="./dist"

echo "📁 Creating package structure..."

# Clean and create directories
rm -rf "${BUILD_DIR}" "${DIST_DIR}"
mkdir -p "${BUILD_DIR}/trinity-genesis"
mkdir -p "${DIST_DIR}"

echo "🔨 Building release binaries..."

# Build UI
cargo build --release --package trinity-ui
cp target/release/trinity-ui "${BUILD_DIR}/trinity-genesis/"

# Build CLI
cargo build --release --package trinity-cli
cp target/release/trinity "${BUILD_DIR}/trinity-genesis/"

echo "📋 Copying documentation..."

# Copy essential files
cp README-USER-FRIENDLY.md "${BUILD_DIR}/trinity-genesis/README.md"
cp TRINITY_TECHNICAL_BIBLE.md "${BUILD_DIR}/trinity-genesis/" 2>/dev/null || true

# Create install script
cat > "${BUILD_DIR}/trinity-genesis/install.sh" << 'EOF'
#!/bin/bash
# 🚀 Trinity Genesis - Quick Install Script

set -e

INSTALL_DIR="${HOME}/.local/share/trinity-genesis"
BIN_DIR="${HOME}/.local/bin"
DESKTOP_DIR="${HOME}/.local/share/applications"

echo "🌌 Installing Trinity Genesis..."

# Create directories
mkdir -p "${INSTALL_DIR}"
mkdir -p "${BIN_DIR}"
mkdir -p "${DESKTOP_DIR}"

# Copy binaries
cp trinity-ui "${INSTALL_DIR}/"
cp trinity "${INSTALL_DIR}/"

# Create symlinks
ln -sf "${INSTALL_DIR}/trinity-ui" "${BIN_DIR}/trinity-ui"
ln -sf "${INSTALL_DIR}/trinity" "${BIN_DIR}/trinity"

# Create desktop entry
cat > "${DESKTOP_DIR}/trinity-genesis.desktop" << EOL
[Desktop Entry]
Name=Trinity Genesis
Comment=AI-Powered Learning Companion
Exec=${INSTALL_DIR}/trinity-ui
Icon=${INSTALL_DIR}/icon.png
Type=Application
Categories=Education;Development;
Terminal=false
EOL

# Create simple icon (placeholder - replace with real icon)
echo "Creating icon..."
# TODO: Add real icon file

echo "✅ Trinity Genesis installed!"
echo ""
echo "🎯 Usage:"
echo "  trinity-ui     - Start the graphical interface"
echo "  trinity --help - Show CLI options"
echo ""
echo "📚 First time? Try:"
echo "  trinity create-curriculum 'Rust Programming'"
echo ""
echo "🎮 Launch from your applications menu or run: trinity-ui"
EOF

chmod +x "${BUILD_DIR}/trinity-genesis/install.sh"

# Create run script (for portable use without install)
cat > "${BUILD_DIR}/trinity-genesis/run-trinity.sh" << 'EOF'
#!/bin/bash
# 🚀 Run Trinity without installing

cd "$(dirname "$0")"

echo "🌌 Starting Trinity Genesis..."
echo ""

# Check for LM Studio
if curl -s http://localhost:1234/v1/models > /dev/null 2>&1; then
    echo "✅ LM Studio detected - AI features enabled"
else
    echo "⚠️  LM Studio not detected - AI features will be limited"
    echo "   Install from: https://lmstudio.ai/"
fi

echo ""
echo "🎯 Starting UI..."
./trinity-ui "$@"
EOF

chmod +x "${BUILD_DIR}/trinity-genesis/run-trinity.sh"

# Create quick start guide
cat > "${BUILD_DIR}/trinity-genesis/QUICKSTART.txt" << 'EOF'
🌌 TRINITY GENESIS - QUICK START
================================

METHOD 1: Install System-Wide (Recommended)
-------------------------------------------
./install.sh

This will:
  • Install to ~/.local/share/trinity-genesis/
  • Add to your PATH
  • Create application menu entry
  • Set up desktop integration

Then run: trinity-ui


METHOD 2: Portable (No Install)
------------------------------
./run-trinity.sh

Runs directly from this folder without installing.
Good for testing or USB drives.


METHOD 3: AppImage (Coming Soon)
-------------------------------
Double-click: Trinity-Genesis.AppImage


FIRST STEPS
-----------
1. Start the UI:
   ./trinity-ui

2. Or generate a curriculum via CLI:
   ./trinity create-curriculum "Rust Programming"

3. Open the generated curriculum folder and start learning!


NEED AI FEATURES?
-----------------
Install LM Studio from https://lmstudio.ai/
Trinity will auto-detect it on startup.


TROUBLESHOOTING
---------------
• Won't start? Install Rust dependencies:
  sudo apt install libgtk-3-0 libwebkit2gtk-4.0-37  (Ubuntu/Debian)
  sudo dnf install gtk3 webkit2gtk3                  (Fedora)

• Graphics issues? Update your GPU drivers

• Questions? Read README.md


HAPPY LEARNING! 🚀
EOF

echo "📦 Creating distribution archives..."

# Create tar.gz
cd "${BUILD_DIR}"
tar -czf "../${DIST_DIR}/${PACKAGE_NAME}.tar.gz" trinity-genesis
cd ..

# Create zip (for Windows friends)
cd "${BUILD_DIR}"
zip -r "../${DIST_DIR}/${PACKAGE_NAME}.zip" trinity-genesis
cd ..

echo "✅ Package created!"
echo ""
echo "📦 Distribution files:"
echo "  ${DIST_DIR}/${PACKAGE_NAME}.tar.gz"
echo "  ${DIST_DIR}/${PACKAGE_NAME}.zip"
echo ""
echo "🎯 To test:"
echo "  cd ${DIST_DIR}"
echo "  tar -xzf ${PACKAGE_NAME}.tar.gz"
echo "  cd trinity-genesis"
echo "  ./run-trinity.sh"
echo ""
echo "📤 Share these files with friends!"

# Create install script for one-line install
cat > "${DIST_DIR}/install-trinity.sh" << 'EOF'
#!/bin/bash
# 🚀 One-Line Trinity Installer
# Usage: curl -sSL https://yourdomain.com/install-trinity.sh | bash

set -e

echo "🌌 Installing Trinity Genesis..."

# Download latest release
DOWNLOAD_URL="https://github.com/yourusername/trinity-genesis/releases/latest/download/trinity-genesis-0.1.0-friend-test.tar.gz"
INSTALL_DIR="${HOME}/.local/share/trinity-genesis"

echo "📥 Downloading..."
curl -L -o /tmp/trinity-genesis.tar.gz "${DOWNLOAD_URL}"

echo "📦 Extracting..."
mkdir -p "${INSTALL_DIR}"
tar -xzf /tmp/trinity-genesis.tar.gz -C /tmp/
cp -r /tmp/trinity-genesis/* "${INSTALL_DIR}/"

echo "🔗 Setting up..."
mkdir -p "${HOME}/.local/bin"
ln -sf "${INSTALL_DIR}/trinity-ui" "${HOME}/.local/bin/trinity-ui"
ln -sf "${INSTALL_DIR}/trinity" "${HOME}/.local/bin/trinity"

echo "✅ Installation complete!"
echo ""
echo "🎯 Run: trinity-ui"
echo "📚 Or:   trinity create-curriculum 'Your Topic'"
EOF

chmod +x "${DIST_DIR}/install-trinity.sh"

echo ""
echo "🎉 All done! Share these files:"
ls -lh "${DIST_DIR}/"
