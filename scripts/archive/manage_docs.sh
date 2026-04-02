#!/bin/bash
# Trinity Document Management Script
# Maintains the strict document organization system

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_header() {
    echo -e "${BLUE}=== $1 ===${NC}"
}

# Check for unauthorized root-level documents
check_root_docs() {
    print_header "Checking Root Level Documents"
    
    local unauthorized=()
    for file in "$PROJECT_ROOT"/*.md; do
        if [ -f "$file" ]; then
            basename=$(basename "$file")
            if [[ "$basename" != "README.md" && "$basename" != "TRINITY_TECHNICAL_BIBLE.md" ]]; then
                unauthorized+=("$basename")
            fi
        fi
    done
    
    if [ ${#unauthorized[@]} -eq 0 ]; then
        print_status "✅ All root-level documents are authorized"
    else
        print_warning "⚠️  Found unauthorized root-level documents:"
        for doc in "${unauthorized[@]}"; do
            echo "  - $doc"
        done
        echo
        print_status "Use '$0 archive' to move them to appropriate archives"
    fi
}

# Archive unauthorized documents
archive_docs() {
    print_header "Archiving Documents"
    
    local archived=0
    for file in "$PROJECT_ROOT"/*.md; do
        if [ -f "$file" ]; then
            basename=$(basename "$file")
            if [[ "$basename" != "README.md" && "$basename" != "TRINITY_TECHNICAL_BIBLE.md" ]]; then
                # Determine archive category based on content
                if grep -qi "day\|progress\|session\|weekly" "$file" 2>/dev/null; then
                    mv "$file" "$PROJECT_ROOT/docs/archive/legacy/"
                    print_status "Archived $basename → docs/archive/legacy/"
                    ((archived++))
                elif grep -qi "memory\|hardware\|verification\|specs\|technical" "$file" 2>/dev/null; then
                    mv "$file" "$PROJECT_ROOT/docs/archive/technical/"
                    print_status "Archived $basename → docs/archive/technical/"
                    ((archived++))
                else
                    mv "$file" "$PROJECT_ROOT/docs/archive/reports/"
                    print_status "Archived $basename → docs/archive/reports/"
                    ((archived++))
                fi
            fi
        fi
    done
    
    if [ $archived -eq 0 ]; then
        print_status "✅ No documents needed archiving"
    else
        print_status "✅ Archived $archived documents"
    fi
}

# Show archive statistics
show_stats() {
    print_header "Document Statistics"
    
    echo "Root Level Documents:"
    ls -1 "$PROJECT_ROOT"/*.md 2>/dev/null | wc -l | xargs echo "  Total:"
    
    echo
    echo "Archive Contents:"
    for dir in legacy technical reports; do
        count=$(ls -1 "$PROJECT_ROOT/docs/archive/$dir"/*.md 2>/dev/null | wc -l)
        echo "  docs/archive/$dir/: $count documents"
    done
    
    echo
    echo "Technical Bible Size:"
    if [ -f "$PROJECT_ROOT/TRINITY_TECHNICAL_BIBLE.md" ]; then
        wc -l "$PROJECT_ROOT/TRINITY_TECHNICAL_BIBLE.md" | awk '{print "  " $1 " lines"}'
        du -h "$PROJECT_ROOT/TRINITY_TECHNICAL_BIBLE.md" | awk '{print "  " $1}'
    fi
}

# Quick search across all documents
search_docs() {
    local query="$1"
    print_header "Searching for: $query"
    
    # Search in Technical Bible first
    if grep -n "$query" "$PROJECT_ROOT/TRINITY_TECHNICAL_BIBLE.md" 2>/dev/null; then
        echo
    fi
    
    # Then search in archives
    echo "Archive Results:"
    find "$PROJECT_ROOT/docs/archive" -name "*.md" -exec grep -l "$query" {} \; 2>/dev/null | while read file; do
        echo "  $(echo "$file" | sed "s|$PROJECT_ROOT/||")"
    done
}

# Show help
show_help() {
    echo "Trinity Document Management Script"
    echo
    echo "Usage: $0 [command]"
    echo
    echo "Commands:"
    echo "  check      Check for unauthorized root-level documents"
    echo "  archive    Move unauthorized documents to archives"
    echo "  stats      Show document statistics"
    echo "  search     Search across all documents"
    echo "  help       Show this help message"
    echo
    echo "Examples:"
    echo "  $0 check                    # Check document organization"
    echo "  $0 archive                  # Clean up root directory"
    echo "  $0 search 'memory'          # Search for 'memory' in all docs"
}

# Main execution
case "${1:-check}" in
    check)
        check_root_docs
        ;;
    archive)
        archive_docs
        ;;
    stats)
        show_stats
        ;;
    search)
        if [ -z "$2" ]; then
            print_error "Please provide a search term"
            echo "Usage: $0 search <term>"
            exit 1
        fi
        search_docs "$2"
        ;;
    help|--help|-h)
        show_help
        ;;
    *)
        print_error "Unknown command: $1"
        show_help
        exit 1
        ;;
esac
