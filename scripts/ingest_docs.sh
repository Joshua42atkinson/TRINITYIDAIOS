#!/bin/bash
# Ingest Trinity documents into the headless server's RAG database
# Usage: ./scripts/ingest_docs.sh

SERVER="http://localhost:3000"

ingest_file() {
    local file="$1"
    local category="$2"
    local title=$(basename "$file" .md)
    
    echo "📄 Ingesting: $title ($category)"
    
    # Use python3 to properly JSON-escape the content and POST it
    python3 -c "
import json, sys, urllib.request

with open('$file', 'r') as f:
    content = f.read()

data = json.dumps({
    'title': '$title',
    'content': content,
    'category': '$category'
}).encode('utf-8')

req = urllib.request.Request('$SERVER/api/ingest', data=data, headers={'Content-Type': 'application/json'})
try:
    resp = urllib.request.urlopen(req, timeout=30)
    result = json.loads(resp.read())
    print(f'  ✅ {result.get(\"chunks_created\", 0)} chunks')
except Exception as e:
    print(f'  ❌ Failed: {e}')
"
}

echo "🚀 Ingesting Trinity documents into RAG..."
echo ""

# Core documents
ingest_file "TRINITY_TECHNICAL_BIBLE.md" "architecture"
ingest_file "context.md" "architecture"

# Research reports
for f in docs/reports/*.md; do
    [ -f "$f" ] && ingest_file "$f" "research"
done

echo ""
echo "✅ Ingestion complete"
