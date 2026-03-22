#!/bin/bash
# PostgreSQL + pgvector Setup for Trinity

echo "🐘 Setting up PostgreSQL with pgvector for Trinity..."

# Check if we can use sudo
if ! sudo -n true 2>/dev/null; then
    echo "🔐 Need sudo access. Please enter password:"
    sudo -v
fi

# Update package list
echo "📦 Updating package list..."
sudo apt update

# Install PostgreSQL and development headers
echo "📦 Installing PostgreSQL..."
sudo apt install -y postgresql postgresql-contrib libpq-dev

# Start and enable PostgreSQL
echo "🚀 Starting PostgreSQL service..."
sudo systemctl start postgresql
sudo systemctl enable postgresql

# Install pgvector extension
echo "📊 Installing pgvector extension..."
cd /tmp
git clone --branch v0.5.1 https://github.com/pgvector/pgvector.git
cd pgvector
sudo make install

# Restart PostgreSQL to load pgvector
echo "🔄 Restarting PostgreSQL..."
sudo systemctl restart postgresql

# Create Trinity database and user
echo "👤 Creating Trinity database and user..."
sudo -u postgres psql -c "CREATE USER trinity WITH PASSWORD 'trinity6226';"
sudo -u postgres psql -c "CREATE DATABASE trinity OWNER trinity;"
sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE trinity TO trinity;"

# Enable pgvector in Trinity database
echo "🔧 Enabling pgvector extension..."
sudo -u postgres psql -d trinity -c "CREATE EXTENSION IF NOT EXISTS vector;"

# Create connection URL
echo "✅ PostgreSQL setup complete!"
echo ""
echo "📋 Connection Details:"
echo "  Host: localhost"
echo "  Port: 5432"
echo "  Database: trinity"
echo "  User: trinity"
echo "  Password: trinity6226"
echo ""
echo "🔗 Connection URL: postgresql://trinity:trinity6226@localhost:5432/trinity"

# Save to .env file
echo "DATABASE_URL=postgresql://trinity:trinity6226@localhost:5432/trinity" > .env.local
echo "✅ Saved to .env.local"
