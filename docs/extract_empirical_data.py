import os
import re

workspace = "/home/joshua/Workflow/desktop_trinity/trinity-genesis"
output_file = "/home/joshua/Workflow/desktop_trinity/trinity-genesis/docs/grep_synthesis_temp.md"

target_keywords = re.compile(r'(longcat|vllm)', re.IGNORECASE)

# Find all markdown files
md_files = []
for root, dirs, files in os.walk(workspace):
    if '.venv' in root or 'node_modules' in root or 'fluentllm' in root:
        continue
    for f in files:
        if f.endswith('.md'):
            md_files.append(os.path.join(root, f))

# Sort by modification time
md_files.sort(key=lambda x: os.path.getmtime(x))

extracted_data = []

for file_path in md_files:
    # Skip the document we are currently building
    if "EVERYTHING_WE_KNOW_ABOUT_LONGCAT_ON_AMD.md" in file_path or "grep_synthesis_temp.md" in file_path:
        continue
    
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
            
            # Split into paragraphs
            paragraphs = content.split('\n\n')
            relevant_paragraphs = []
            
            for p in paragraphs:
                if target_keywords.search(p):
                    # Clean up flavorful text mechanically
                    clean_p = p.strip()
                    # Skip overly generic headers or empty
                    if len(clean_p) > 20:
                        relevant_paragraphs.append(clean_p)
                        
            if relevant_paragraphs:
                extracted_data.append(f"### Source: {os.path.basename(file_path)}")
                for rp in relevant_paragraphs:
                    extracted_data.append(f"- {rp}")
                    
    except Exception as e:
        continue

with open(output_file, 'w', encoding='utf-8') as out:
    out.write("\n".join(extracted_data))

print(f"Extraction complete. Found data in {len(extracted_data)} chunks.")
