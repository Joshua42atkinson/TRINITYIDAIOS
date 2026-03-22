#!/bin/bash
# Strictly Bash: No Python
# Populates the Books of the Bible with actual rustdoc and Cargo.toml data

mkdir -p docs/books_of_the_bible/crates

for crate_dir in $(find crates -mindepth 1 -maxdepth 2 -name "Cargo.toml" -exec dirname {} \; | sort); do
    CRATE_NAME=$(basename $crate_dir)
    BOOK="docs/books_of_the_bible/crates/${CRATE_NAME}.md"
    
    echo "# 📖 Book of ${CRATE_NAME}" > "$BOOK"
    echo "" >> "$BOOK"
    
    # Extract version and description from Cargo.toml
    VERSION=$(grep "^version =" "$crate_dir/Cargo.toml" | cut -d'"' -f2 || echo "0.1.0")
    DESC=$(grep "^description =" "$crate_dir/Cargo.toml" | cut -d'"' -f2 || echo "Core Trinity component")
    
    echo "## Specifications" >> "$BOOK"
    echo "- **Crate**: \`$CRATE_NAME\`" >> "$BOOK"
    echo "- **Version**: $VERSION" >> "$BOOK"
    echo "- **Description**: $DESC" >> "$BOOK"
    echo "- **Location**: \`$crate_dir\`" >> "$BOOK"
    echo "" >> "$BOOK"
    
    echo "## Technical Architecture" >> "$BOOK"
    echo "*(Auto-extracted from module level documentation)*" >> "$BOOK"
    echo "" >> "$BOOK"
    
    # Try to extract module docs from lib.rs or main.rs
    for src_file in "$crate_dir/src/lib.rs" "$crate_dir/src/main.rs"; do
        if [ -f "$src_file" ]; then
            echo "\`\`\`rust" >> "$BOOK"
            echo "// Source: $src_file" >> "$BOOK"
            # Get lines starting with //! (module docs)
            grep "^//!" "$src_file" | sed 's|^//! ||' | head -n 50 >> "$BOOK"
            echo "\`\`\`" >> "$BOOK"
            echo "" >> "$BOOK"
        fi
    done
    
done

echo "Books populated with ground-truth code data."
