#!/bin/bash
# Fix huggingface-cli installation to persist

echo "🔧 Fixing huggingface-cli installation..."

# Install huggingface-hub properly
pip3 install --user huggingface-hub

# Add to PATH if not already there
HUGGINGFACE_PATH="$HOME/.local/bin"
if [[ ":$PATH:" != *":$HUGGINGFACE_PATH:"* ]]; then
    echo "export PATH=\"$HUGGINGFACE_PATH:\$PATH\"" >> ~/.bashrc
    echo "✅ Added huggingface-cli to PATH"
fi

echo "🔄 Reload shell or run: source ~/.bashrc"
echo "✅ huggingface-cli fixed!"
