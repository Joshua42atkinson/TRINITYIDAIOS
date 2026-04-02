#!/bin/bash
# WASM Build Script for Trinity Game Templates

set -e

TEMPLATE_DIR="${1:-templates/first-game}"
OUTPUT_DIR="$TEMPLATE_DIR/dist"

echo "Building WASM for $TEMPLATE_DIR..."

# Check for wasm32 target
if ! rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
    echo "Installing wasm32 target..."
    rustup target add wasm32-unknown-unknown
fi

# Check for wasm-bindgen-cli
if ! command -v wasm-bindgen &> /dev/null; then
    echo "Installing wasm-bindgen-cli..."
    cargo install wasm-bindgen-cli
fi

# Build WASM
cd "$TEMPLATE_DIR"
cargo build --release --target wasm32-unknown-unknown

# Generate bindings
mkdir -p dist
wasm-bindgen --out-dir dist \
    --target web \
    target/wasm32-unknown-unknown/release/*.wasm

# Create index.html
cat > dist/index.html << 'EOF'
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Trinity Game</title>
    <style>
        body { margin: 0; overflow: hidden; background: #1a1a2e; }
        canvas { display: block; width: 100vw; height: 100vh; }
    </style>
</head>
<body>
    <script type="module">
        import init from './trinity_first_game.js';
        init();
    </script>
</body>
</html>
EOF

echo "✅ WASM build complete: $OUTPUT_DIR"
echo "   Serve with: python -m http.server -d $OUTPUT_DIR 8000"
