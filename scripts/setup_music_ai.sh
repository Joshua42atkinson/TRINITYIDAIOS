#!/bin/bash

# 🎵 Trinity Music AI Subagent Setup Script
# 
# This script sets up the complete Music AI subagent environment
# including model downloads, dependencies, and OBS integration

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# ASCII Art Banner
cat << 'EOF'
 _____ _   _ _   _    _    _   _  ____ _____ ____  
|_   _| | | | \ | |  / \  | \ | |/ ___| ____|  _ \ 
  | | | |_| |  \| | / _ \ |  \| | |   |  _| | | | |
  | | |  _  | |\  |/ ___ \| |\  | |___| |___| |_| |
  |_| |_| |_|_| \_/_/   \_\_| \_|\____|_____|____/ 
     ___  _   _ ____  ____  _   _  ____ _____ ____  
    / _ \| | | |  _ \|  _ \| | | |/ ___| ____|  _ \ 
   | | | | | | | |_) | |_) | |_| | |   |  _| | | | |
   | |_| | |_| |  _ <|  _ <|  _  | |___| |___| |_| |
    \___/ \___/|_| \_\_| \_\_|_| |_|\____|_____|____/ 
    
    🎵 Educational Music Generation with OBS Integration
    🎯 Research-Based Learning Enhancement
    📹 Multisensory Educational Experience
EOF

echo ""
echo -e "${CYAN}Welcome to Trinity Music AI Subagent Setup!${NC}"
echo ""
echo -e "${WHITE}This script will set up the complete music generation environment${NC}"
echo -e "${WHITE}including AI models, OBS integration, and educational optimization.${NC}"
echo ""

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}❌ Error: Please run this script from the Trinity root directory${NC}"
    exit 1
fi

# Check system requirements
echo -e "${BLUE}=== Checking System Requirements ===${NC}"
echo ""

# Check Rust
if command -v rustc >/dev/null 2>&1; then
    echo -e "${GREEN}✅ Rust found: $(rustc --version)${NC}"
else
    echo -e "${RED}❌ Rust not found. Please install Rust first.${NC}"
    exit 1
fi

# Check Cargo
if command -v cargo >/dev/null 2>&1; then
    echo -e "${GREEN}✅ Cargo found: $(cargo --version)${NC}"
else
    echo -e "${RED}❌ Cargo not found. Please install Rust/Cargo first.${NC}"
    exit 1
fi

# Check available disk space
echo -e "${BLUE}=== Checking Available Disk Space ===${NC}"
AVAILABLE_SPACE=$(df -h . | awk 'NR==2 {print $4}')
echo -e "${WHITE}Available disk space: ${AVAILABLE_SPACE}${NC}"

# Check if we have enough space for music models (~12GB)
if [[ "$AVAILABLE_SPACE" =~ ([0-9]+)G ]]; then
    SPACE_GB=${BASH_REMATCH[1]}
    if [ "$SPACE_GB" -lt 15 ]; then
        echo -e "${YELLOW}⚠️  Warning: Less than 15GB available. Music models require ~12GB.${NC}"
        read -p "Continue anyway? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            echo -e "${BLUE}Setup cancelled. Please free up disk space.${NC}"
            exit 1
        fi
    else
        echo -e "${GREEN}✅ Sufficient disk space available${NC}"
    fi
fi

echo ""

# Check HuggingFace CLI status
echo -e "${BLUE}=== Checking HuggingFace CLI Status ===${NC}"
if command -v huggingface-cli >/dev/null 2>&1; then
    echo -e "${GREEN}✅ HuggingFace CLI found${NC}"
    
    # Check if there's an active download
    if pgrep -f "huggingface-cli" >/dev/null; then
        echo -e "${YELLOW}⚠️  HuggingFace CLI is currently running${NC}"
        echo -e "${YELLOW}   Waiting for current download to complete...${NC}"
        
        # Wait for download to complete
        while pgrep -f "huggingface-cli" >/dev/null; do
            echo -e "${YELLOW}   Still downloading... (waiting 30 seconds)${NC}"
            sleep 30
        done
        
        echo -e "${GREEN}✅ HuggingFace download completed${NC}"
    fi
else
    echo -e "${YELLOW}⚠️  HuggingFace CLI not found${NC}"
    echo -e "${WHITE}Installing HuggingFace CLI...${NC}"
    pip install -U "huggingface_hub[cli]"
fi

echo ""

# Build Trinity Music AI subagent
echo -e "${BLUE}=== Building Trinity Music AI Subagent ===${NC}"
echo ""

cd crates/trinity-music-ai
echo -e "${WHITE}Building Music AI subagent...${NC}"
cargo build --release

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Music AI subagent built successfully${NC}"
else
    echo -e "${RED}❌ Failed to build Music AI subagent${NC}"
    exit 1
fi

# Create binary symlink
cd ../..
ln -sf crates/trinity-music-ai/target/release/trinity-music-ai trinity-music-ai
echo -e "${GREEN}✅ Created trinity-music-ai symlink${NC}"

echo ""

# Install MusicGPT
echo -e "${BLUE}=== Installing MusicGPT ===${NC}"
echo ""

if command -v musicgpt >/dev/null 2>&1; then
    echo -e "${GREEN}✅ MusicGPT already found: $(musicgpt --version 2>/dev/null || echo 'version unknown')${NC}"
else
    echo -e "${WHITE}Installing MusicGPT...${NC}"
    
    # Clone MusicGPT repository
    if [ ! -d "MusicGPT" ]; then
        git clone https://github.com/gabotechs/MusicGPT.git
    fi
    
    cd MusicGPT
    
    # Build MusicGPT
    echo -e "${WHITE}Building MusicGPT...${NC}"
    cargo build --release
    
    if [ $? -eq 0 ]; then
        # Create symlink
        cd ..
        ln -sf MusicGPT/target/release/musicgpt musicgpt
        echo -e "${GREEN}✅ MusicGPT installed successfully${NC}"
    else
        echo -e "${RED}❌ Failed to build MusicGPT${NC}"
        exit 1
    fi
fi

echo ""

# Install OBS Studio
echo -e "${BLUE}=== Installing OBS Studio ===${NC}"
echo ""

if command -v obs >/dev/null 2>&1; then
    echo -e "${GREEN}✅ OBS Studio already found${NC}"
else
    echo -e "${WHITE}Installing OBS Studio...${NC}"
    
    # Add OBS repository
    sudo add-apt-repository ppa:obsproject/obs-studio -y
    sudo apt update
    
    # Install OBS Studio
    sudo apt install obs-studio -y
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✅ OBS Studio installed successfully${NC}"
    else
        echo -e "${RED}❌ Failed to install OBS Studio${NC}"
        exit 1
    fi
fi

echo ""

# Install audio processing dependencies
echo -e "${BLUE}=== Installing Audio Processing Dependencies ===${NC}"
echo ""

echo -e "${WHITE}Installing audio libraries...${NC}"
sudo apt install -y \
    libasound2-dev \
    libjack-jackd2-dev \
    libpulse-dev \
    portaudio19-dev \
    libsndfile1-dev \
    ffmpeg

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Audio dependencies installed successfully${NC}"
else
    echo -e "${RED}❌ Failed to install audio dependencies${NC}"
    exit 1
fi

echo ""

# Download Music Models
echo -e "${BLUE}=== Downloading Music Models ===${NC}"
echo ""

echo -e "${WHITE}This will download approximately 12GB of music models.${NC}"
echo -e "${WHITE}Models to download:${NC}"
echo -e "${WHITE}  - MusicGPT models (small, medium, large)${NC}"
echo -e "${WHITE}  - Stable Audio Open model${NC}"
echo -e "${WHITE}  - MusicGen models (small, medium, large)${NC}"
echo ""

read -p "Proceed with model downloads? (y/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${BLUE}Skipping model downloads. You can download them later with:${NC}"
    echo -e "${WHITE}  ./trinity-music-ai download-models all${NC}"
else
    echo -e "${WHITE}Downloading music models...${NC}"
    
    # Use our CLI to download models
    ./trinity-music-ai download-models all
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✅ Music models downloaded successfully${NC}"
    else
        echo -e "${YELLOW}⚠️  Some models may have failed to download${NC}"
        echo -e "${WHITE}You can retry individual models later.${NC}"
    fi
fi

echo ""

# Configure OBS Studio
echo -e "${BLUE}=== Configuring OBS Studio ===${NC}"
echo ""

echo -e "${WHITE}OBS Studio configuration required for Music AI integration:${NC}"
echo ""
echo -e "${YELLOW}1. Open OBS Studio${NC}"
echo -e "${YELLOW}2. Go to Settings > WebSocket${NC}"
echo -e "${YELLOW}3. Enable WebSocket Server${NC}"
echo -e "${YELLOW}4. Set Port: 4456${NC}"
echo -e "${YELLOW}5. Set Password (or leave blank for local use)${NC}"
echo -e "${YELLOW}6. Click 'Apply' and 'OK'${NC}"
echo ""

echo -e "${WHITE}Required OBS Scenes for Trinity:${NC}"
echo -e "${YELLOW}  - Educational_Intro${NC}"
echo -e "${YELLOW}  - Problem_Solving${NC}"
echo -e "${YELLOW}  - Creative_Exploration${NC}"
echo -e "${YELLOW}  - Practice_Mode${NC}"
echo -e "${YELLOW}  - Assessment_Mode${NC}"
echo -e "${YELLOW}  - Break_Time${NC}"
echo ""

read -p "Have you configured OBS Studio? (y/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${BLUE}Skipping OBS configuration. You can configure it later.${NC}"
    echo -e "${WHITE}Run OBS Studio and follow the configuration steps above.${NC}"
else
    echo -e "${GREEN}✅ OBS Studio configuration noted${NC}"
fi

echo ""

# Test the installation
echo -e "${BLUE}=== Testing Installation ===${NC}"
echo ""

echo -e "${WHITE}Testing Trinity Music AI CLI...${NC}"
if ./trinity-music-ai --help >/dev/null 2>&1; then
    echo -e "${GREEN}✅ Trinity Music AI CLI working${NC}"
else
    echo -e "${RED}❌ Trinity Music AI CLI not working${NC}"
    exit 1
fi

echo -e "${WHITE}Testing MusicGPT...${NC}"
if musicgpt --version >/dev/null 2>&1; then
    echo -e "${GREEN}✅ MusicGPT working${NC}"
else
    echo -e "${YELLOW}⚠️  MusicGPT not working - may need additional setup${NC}"
fi

echo -e "${WHITE}Testing OBS connection...${NC}"
if ./trinity-music-ai test-obs >/dev/null 2>&1; then
    echo -e "${GREEN}✅ OBS connection working${NC}"
else
    echo -e "${YELLOW}⚠️  OBS connection not working - check OBS configuration${NC}"
fi

echo ""

# Create configuration file
echo -e "${BLUE}=== Creating Configuration File ===${NC}"
echo ""

CONFIG_DIR="$HOME/.config/trinity-music-ai"
mkdir -p "$CONFIG_DIR"

cat > "$CONFIG_DIR/config.toml" << 'EOF'
[general]
default_generator = "musicgpt"
default_duration = 30.0
default_volume = 0.7
enable_obs_integration = true

[obs]
websocket_url = "ws://localhost:4456"
password = ""
auto_connect = true

[audio]
sample_rate = 44100
channels = 2
format = "wav"
quality = "high"

[educational]
optimize_cognitive_load = true
induce_flow_state = true
context_aware = true
accessibility_enabled = true

[performance]
max_concurrent_generations = 2
gpu_acceleration = true
memory_limit_gb = 8.0
enable_caching = true
cache_size_mb = 1024.0
EOF

echo -e "${GREEN}✅ Configuration file created: $CONFIG_DIR/config.toml${NC}"

echo ""

# Final summary
echo -e "${GREEN}🎵 Trinity Music AI Subagent Setup Complete!${NC}"
echo ""
echo -e "${WHITE}What's been installed:${NC}"
echo -e "${GREEN}✅ Trinity Music AI CLI${NC}"
echo -e "${GREEN}✅ MusicGPT (if available)${NC}"
echo -e "${GREEN}✅ OBS Studio${NC}"
echo -e "${GREEN}✅ Audio processing dependencies${NC}"
echo -e "${GREEN}✅ Configuration file${NC}"
echo ""
echo -e "${WHITE}Next steps:${NC}"
echo -e "${BLUE}1. Test music generation:${NC}"
echo -e "${WHITE}   ./trinity-music-ai generate --context concept_introduction --prompt 'gentle educational music'${NC}"
echo ""
echo -e "${BLUE}2. Test OBS integration:${NC}"
echo -e "${WHITE}   ./trinity-music-ai generate --context problem_solving --prompt 'focused analytical music' --obs${NC}"
echo ""
echo -e "${BLUE}3. Check system status:${NC}"
echo -e "${WHITE}   ./trinity-music-ai check-system${NC}"
echo ""
echo -e "${BLUE}4. List available models:${NC}"
echo -e "${WHITE}   ./trinity-music-ai list-models${NC}"
echo ""
echo -e "${YELLOW}📚 Documentation:${NC}"
echo -e "${WHITE}   - MUSIC_AI_SUBAGENT_RESEARCH.md${NC}"
echo -e "${WHITE}   - MUSIC_AI_OBS_INTEGRATION.md${NC}"
echo -e "${WHITE}   - MUSIC_AI_RESEARCH_INTEGRATION.md${NC}"
echo ""
echo -e "${GREEN}🌟 Your Trinity Music AI Subagent is ready to create educational magic!${NC}"
echo ""

# Offer to run a test
read -p "Run a test music generation now? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${BLUE}Running test music generation...${NC}"
    ./trinity-music-ai generate \
        --context concept_introduction \
        --prompt "gentle educational music for learning new concepts" \
        --output test_music.wav
    
    if [ -f "test_music.wav" ]; then
        echo -e "${GREEN}✅ Test music generated successfully: test_music.wav${NC}"
    else
        echo -e "${YELLOW}⚠️  Test music generation may have failed${NC}"
    fi
fi

echo ""
echo -e "${GREEN}🎵 Setup complete! Enjoy your educational music journey!${NC}"
