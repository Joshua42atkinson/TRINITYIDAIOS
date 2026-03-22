#!/bin/bash
# Trinity MCP Memory Setup Script

echo "🧠 Setting up Trinity's Memory MCP Server..."

# Check if PostgreSQL is running
if ! command -v psql &> /dev/null; then
    echo "❌ PostgreSQL not found. Please run ./setup_postgres.sh first"
    exit 1
fi

# Test database connection
echo "🔗 Testing database connection..."
if ! PGPASSWORD=trinity6226 psql -h localhost -U trinity -d trinity -c "SELECT 1;" &> /dev/null; then
    echo "❌ Cannot connect to database. Please check PostgreSQL setup"
    exit 1
fi

# Build MCP server
echo "🔨 Building MCP server..."
cargo build --package trinity-mcp-server

# Run documentation ingestion
echo "📚 Ingesting documentation..."
cargo run --bin ingest_documentation.rs

# Start MCP server
echo "🚀 Starting MCP server..."
echo "Server will run on localhost:8080"
echo "Use Ctrl+C to stop"
echo ""
echo "To test the server:"
echo "curl -X POST http://localhost:8080 \\"
echo "  -H 'Content-Type: application/json' \\"
echo "  -d '{\"id\":\"1\",\"method\":\"tools/list\",\"params\":{}}'"
echo ""

DATABASE_URL=postgresql://trinity:trinity6226@localhost:5432/trinity \
cargo run --package trinity-mcp-server
