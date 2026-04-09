import re

def process_docs():
    with open('TRINITY_FANCY_BIBLE.md', 'r') as f:
        bible = f.read()
        
    with open('docs/LONGCAT_ARCHITECTURE.md', 'r') as f:
        longcat_arch = f.read()

    with open('docs/LONGCAT_SIDECAR_SESSION.md', 'r') as f:
        longcat_session = f.read()
        
    with open('docs/archive_vllm_legacy/VLLM_LESSONS_LEARNED.md', 'r') as f:
        vllm_lessons = f.read()

    # Step 1: Adjust headings in the source files to fit under level 3 headings
    # Each source file starts with # (level 1), we want it to be level 4 (####) under our new ### headers.
    def adjust_headings(text):
        return re.sub(r'^(#+)\s', lambda m: '#' * (len(m.group(1)) + 2) + ' ', text, flags=re.MULTILINE)

    longcat_arch = adjust_headings(longcat_arch)
    longcat_session = adjust_headings(longcat_session)
    vllm_lessons = adjust_headings(vllm_lessons)

    # Step 2: Extract the parts of the bible
    # We want to replace from "### 12.1 The AMD Strix Halo Platform" up to "### 12.3 Server Architecture"
    pattern = r'(### 12\.1 The AMD Strix Halo Platform\n.*?)(?=### 12\.3 Server Architecture)'
    match = re.search(pattern, bible, flags=re.DOTALL)
    if not match:
        print("Could not find the section to replace in TRINITY_FANCY_BIBLE.md")
        return
        
    old_section = match.group(1)
    
    # We will keep 12.1 intact
    split_pattern = r'(### 12\.1 The AMD Strix Halo Platform\n.*?)(?=### 12\.2 The vLLM Omni Deployment)'
    split_match = re.search(split_pattern, old_section, flags=re.DOTALL)
    
    if split_match:
        section_12_1 = split_match.group(1)
    else:
        # If 12.2 is missing, just use what we found
        section_12_1 = old_section
        
    new_section = (
        section_12_1 + 
        "### 12.2 SGLang & LongCat-Next Omni-Brain Operations\n\n" +
        "> Incorporating the LongCat Omni Sidecar Session and Architecture details.\n\n" +
        longcat_arch + "\n\n" +
        longcat_session + "\n\n" +
        "### 12.3 vLLM A.R.T.Y. Hub Operations\n\n" +
        "> Incorporating lessons learned from vLLM deployments on Strix Halo.\n\n" +
        vllm_lessons + "\n\n"
    )
    
    new_bible = bible.replace(old_section, new_section)
    
    # Step 3: Renumber the subsequent headers (12.3 -> 12.4, 12.4 -> 12.5, 12.5 -> 12.6)
    new_bible = new_bible.replace('### 12.3 Server Architecture', '### 12.4 Server Architecture')
    new_bible = new_bible.replace('### 12.4 The Trinity Lexicon', '### 12.5 The Trinity Lexicon')
    new_bible = new_bible.replace('### 12.5 What\'s Next', '### 12.6 What\'s Next')
    
    # Step 4: Fix TOC in Bible
    old_toc = "- [Car 12: EVOLVE — Deployment, Hardware, and the Lexicon](#-car-12-evolve--deployment-hardware-and-the-lexicon)\n  - [12.1 Strix Halo](#121-the-amd-strix-halo-platform) · [12.2 KV Cache](#122-the-dual-kv-cache-architecture) · [12.3 Server](#123-server-architecture) · [12.4 Lexicon](#124-the-trinity-lexicon) · [12.5 What's Next](#125-whats-next)"
    
    new_toc = "- [Car 12: EVOLVE — Deployment, Hardware, and the Lexicon](#-car-12-evolve--deployment-hardware-and-the-lexicon)\n  - [12.1 Strix Halo](#121-the-amd-strix-halo-platform) · [12.2 SGLang & LongCat](#122-sglang--longcat-next-omni-brain-operations) · [12.3 vLLM Hub](#123-vllm-arty-hub-operations) · [12.4 Server](#124-server-architecture) · [12.5 Lexicon](#125-the-trinity-lexicon) · [12.6 What's Next](#126-whats-next)"
    
    new_bible = new_bible.replace(old_toc, new_toc)
    
    with open('TRINITY_FANCY_BIBLE.md', 'w') as f:
        f.write(new_bible)
        
    print("Documentation successfully merged into TRINITY_FANCY_BIBLE.md")

if __name__ == '__main__':
    process_docs()
