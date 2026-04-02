#!/bin/bash
# Strictly Bash: No Python Snakes in the Grass.
# Updates the Crate Matrix in the TRINITY_TECHNICAL_BIBLE.md

BIBLE="TRINITY_TECHNICAL_BIBLE.md"

# We will generate a new section for the available crates based on actual folders
NEW_SECTION=$(mktemp)

echo "## 🎮 **Current Active Crate Matrix**" > $NEW_SECTION
echo "" >> $NEW_SECTION
echo "This matrix reflects the literal ground truth of the repository." >> $NEW_SECTION
echo "" >> $NEW_SECTION

for crate_dir in $(find crates -mindepth 1 -maxdepth 2 -name "Cargo.toml" -exec dirname {} \; | sort); do
    CRATE_NAME=$(basename $crate_dir)
    # Extract description from Cargo.toml if it exists
    DESC=$(grep "^description =" "$crate_dir/Cargo.toml" | cut -d'"' -f2 || echo "Core Trinity component")
    if [ -z "$DESC" ]; then DESC="Core Trinity component"; fi
    
    echo "### **$CRATE_NAME**" >> $NEW_SECTION
    echo "- **Path**: \`$crate_dir\`" >> $NEW_SECTION
    echo "- **Description**: $DESC" >> $NEW_SECTION
    echo "- **Technical Manual**: [\`docs/books_of_the_bible/crates/$CRATE_NAME.md\`](docs/books_of_the_bible/crates/$CRATE_NAME.md)" >> $NEW_SECTION
    echo "" >> $NEW_SECTION
done

# Find where the old Available Crates section starts and ends
# The section starts at '## 🎮 **Available Crates**'
# And ends at the next '## '

awk -v new_content="$(cat $NEW_SECTION)" '
    /## 🎮 \*\*Available Crates\*\*/ {
        print new_content;
        skip = 1;
        next;
    }
    /^## / && skip {
        if ($0 !~ /## 🎮/) {
            skip = 0;
            print $0;
        }
        next;
    }
    !skip {
        print $0;
    }
' "$BIBLE" > "${BIBLE}.tmp"

mv "${BIBLE}.tmp" "$BIBLE"
rm $NEW_SECTION

echo "Crate matrix updated successfully."
