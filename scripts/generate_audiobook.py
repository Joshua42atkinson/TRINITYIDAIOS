import os
import re
import io
import wave
import numpy as np
import scipy.io.wavfile
from kokoro import KPipeline

def clean_markdown(text):
    # Remove images
    text = re.sub(r'!\[.*?\]\(.*?\)', '', text)
    # Remove links but keep the text
    text = re.sub(r'\[(.*?)\]\(.*?\)', r'\1', text)
    # Remove Markdown headers
    text = re.sub(r'^#+\s+', '', text, flags=re.MULTILINE)
    # Remove bold/italics
    text = re.sub(r'\*\*(.*?)\*\*', r'\1', text)
    text = re.sub(r'\*(.*?)\*', r'\1', text)
    # Remove blockquotes
    text = re.sub(r'^>\s+', '', text, flags=re.MULTILINE)
    # Remove table formatting
    text = re.sub(r'\|.*?\|', '', text)
    text = re.sub(r'---+', '', text)
    # Replace em dashes with a pause comma or space
    text = text.replace('—', ', ')
    return text.strip()

def main():
    print("Loading Kokoro TTS Pipeline...")
    pipeline = KPipeline(lang_code='a')
    DEFAULT_VOICE = "am_adam" # Professional narrator voice

    input_file = "PLAYERS_HANDBOOK.md"
    output_dir = "audiobook_output"
    os.makedirs(output_dir, exist_ok=True)

    with open(input_file, 'r', encoding='utf-8') as f:
        content = f.read()

    # Split roughly by major sections or chapters to avoid huge memory spikes and to provide track chunks
    # Searching for H1 or H2
    sections = re.split(r'\n(##? .*?)\n', content)
    
    # sections will be: [preamble, header1, content1, header2, content2, ...]
    if not sections:
        return
    
    current_chapter = 0
    
    # Process preamble (which might be Table of Contents, we can skip if small or just process it)
    preamble = clean_markdown(sections[0])
    # To save time, we might skip the table of contents completely. We'll look for "Table of Contents" in text and discard.
    
    for i in range(1, len(sections), 2):
        header = sections[i].strip()
        body = sections[i+1].strip()
        
        if "Table of Contents" in header:
            continue
            
        current_chapter += 1
        
        full_text = f"{header}\n\n{body}"
        clean_text = clean_markdown(full_text)
        
        if not clean_text:
            continue
            
        print(f"\nProcessing Chapter {current_chapter}: {header}...")
        
        audio_chunks = []
        try:
            # Generate audio chunks. We split by sentences or paragraphs (default Kokoro regex)
            for gs, ps, audio_np in pipeline(clean_text, voice=DEFAULT_VOICE, speed=1.0, split_pattern=r'\n+'):
                if hasattr(audio_np, 'numpy'):
                    if hasattr(audio_np, 'cpu'):
                        audio_np = audio_np.cpu()
                    audio_np = audio_np.numpy()
                audio_chunks.append(audio_np)
                
            if audio_chunks:
                final_audio = np.concatenate(audio_chunks)
                audio_int16 = (final_audio * 32767).astype(np.int16)
                
                safe_title = re.sub(r'[^a-zA-Z0-9_\-]', '_', header.replace(" ", "_"))
                safe_title = re.sub(r'_+', '_', safe_title)
                filename = os.path.join(output_dir, f"{current_chapter:02d}_{safe_title}.wav")
                
                scipy.io.wavfile.write(filename, 24000, audio_int16)
                print(f"Saved {filename}")
        except Exception as e:
            print(f"Failed on Chapter {current_chapter}: {e}")

    print("\nAudiobook generation complete!")

if __name__ == "__main__":
    main()
