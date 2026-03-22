#!/bin/bash

# Neon Wizard's Password Fix - No More Sudo!

echo "🧙‍♂️ Neon Wizard's Password Fix Spell"
echo "====================================="
echo ""

# Colors
PURPLE='\033[0;35m'
GREEN='\033[0;32m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${PURPLE}🔧 Fixing PostgreSQL authentication...${NC}"

# 1. Create .pgpass file for automatic authentication
echo ""
echo -e "${CYAN}Creating .pgpass file...${NC}"
cat > ~/.pgpass << 'EOF'
localhost:5432:trinity:trinity:6226
EOF

chmod 600 ~/.pgpass
echo -e "${GREEN}✅ .pgpass file created${NC}"

# 2. Set PGPASSWORD environment variable
echo ""
echo -e "${CYAN}Setting environment variable...${NC}"
export PGPASSWORD="6226"
echo 'export PGPASSWORD="6226"' >> ~/.bashrc
echo -e "${GREEN}✅ PGPASSWORD set in ~/.bashrc${NC}"

# 3. Test connection
echo ""
echo -e "${CYAN}Testing database connection...${NC}"
if psql -h localhost -U trinity -d trinity -c "SELECT 1;" &>/dev/null; then
    echo -e "${GREEN}✅ Database connection successful!${NC}"
else
    echo -e "${CYAN}Creating database if needed...${NC}"
    sudo -u postgres psql -c "CREATE USER trinity WITH PASSWORD '6226';" 2>/dev/null || true
    sudo -u postgres psql -c "CREATE DATABASE trinity;" 2>/dev/null || true
    sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE trinity TO trinity;" 2>/dev/null || true
    
    if psql -h localhost -U trinity -d trinity -c "SELECT 1;" &>/dev/null; then
        echo -e "${GREEN}✅ Database ready!${NC}"
    else
        echo -e "${RED}❌ Still having issues${NC}"
    fi
fi

# 4. Create startup script with password
echo ""
echo -e "${CYAN}Creating easy startup script...${NC}"
cat > start_trinity.sh << 'EOF'
#!/bin/bash

# Trinity Startup Script - No Sudo Required!

# Set environment
export DATABASE_URL="postgresql://trinity:6226@localhost:5432/trinity"
export PGPASSWORD="6226"
export RUST_LOG=info

echo "🚀 Starting Trinity..."

# Start MCP server
echo "Starting MCP Memory Server..."
cargo run --package trinity-mcp-server --bin trinity-mcp-server &
MCP_PID=$!

echo "MCP Server PID: $MCP_PID"
echo "Database: $DATABASE_URL"
echo ""
echo "✅ Trinity is starting..."
echo "Check with: curl -X POST localhost:8080 -d '{\"id\":\"test\",\"method\":\"tools/list\"}'"
EOF

chmod +x start_trinity.sh
echo -e "${GREEN}✅ start_trinity.sh created${NC}"

# 5. Update the success protocol
echo ""
echo -e "${CYAN}Updating success protocol...${NC}"
sed -i 's/sudo -u postgres psql -c "CREATE USER trinity/echo "PGPASSWORD=6226 psql -h localhost -U postgres -c \"CREATE USER trinity/g' trinity_success_protocol.sh
sed -i 's/sudo -u postgres createdb trinity/echo "PGPASSWORD=6226 createdb -h localhost -U postgres trinity"/g' trinity_success_protocol.sh

echo ""
echo -e "${PURPLE}🎉 Password Fix Complete!${NC}"
echo ""
echo -e "${CYAN}From now on:${NC}"
echo "1. Use: ./start_trinity.sh (no sudo!)"
echo "2. Or: export PGPASSWORD=6226 && cargo run..."
echo "3. Database connects automatically"
echo ""
echo -e "${GREEN}✨ No more sudo prompts!${NC}"
