#!/usr/bin/env python3
"""
Avatar Pipeline — Full NPC Character Creation
==============================================
Creates a complete humanoid NPC with backstory, portrait, 3D mesh,
voice, theme music, and Bevy ECS entity definition.

Uses Mistral Small 4 (brain) + ComfyUI (images) + Kokoro (voice) + Blender (3D).
Gracefully skips unavailable tools (Trellis, ACE-Step, HunyuanVideo).

Usage:
  python3 avatar_pipeline.py "a grizzled steam engineer who maintains the Iron Road"
  python3 avatar_pipeline.py --style cyberpunk "a neon-lit bounty hunter with chrome implants"
"""

import os
import sys
import json
import time
import argparse
import requests
import subprocess
from pathlib import Path

MISTRAL_URL = os.environ.get("TRINITY_LLM_URL", "http://localhost:8080")
COMFYUI_URL = os.environ.get("COMFYUI_URL", "http://localhost:8188")
VOICE_URL = os.environ.get("TRINITY_VOICE_URL", "http://localhost:7777")
OUTPUT_BASE = Path(os.environ.get("AVATAR_OUTPUT", os.path.expanduser(
    "~/Workflow/desktop_trinity/trinity-genesis/assets/avatars")))

def llm(prompt, max_tokens=2048):
    """Call Mistral Small 4 via OpenAI-compatible API."""
    resp = requests.post(f"{MISTRAL_URL}/v1/chat/completions", json={
        "model": "default",
        "messages": [{"role": "user", "content": prompt}],
        "max_tokens": max_tokens,
        "temperature": 0.7,
    }, timeout=120)
    resp.raise_for_status()
    return resp.json()["choices"][0]["message"]["content"].strip()

def comfyui_image(prompt, negative="blurry, low quality, distorted, watermark, text",
                  width=1024, height=1024, seed=None):
    """Generate image via ComfyUI SDXL Turbo."""
    import random
    if seed is None:
        seed = random.randint(0, 2**32)

    workflow = {"prompt": {
        "1": {"class_type": "CheckpointLoaderSimple",
              "inputs": {"ckpt_name": "sd_xl_turbo_1.0_fp16.safetensors"}},
        "2": {"class_type": "CLIPTextEncode",
              "inputs": {"text": prompt, "clip": ["1", 1]}},
        "3": {"class_type": "CLIPTextEncode",
              "inputs": {"text": negative, "clip": ["1", 1]}},
        "4": {"class_type": "EmptyLatentImage",
              "inputs": {"width": width, "height": height, "batch_size": 1}},
        "5": {"class_type": "KSampler",
              "inputs": {"model": ["1", 0], "positive": ["2", 0], "negative": ["3", 0],
                         "latent_image": ["4", 0], "seed": seed, "steps": 4,
                         "cfg": 1.0, "sampler_name": "euler", "scheduler": "normal",
                         "denoise": 1.0}},
        "6": {"class_type": "VAEDecode",
              "inputs": {"samples": ["5", 0], "vae": ["1", 2]}},
        "7": {"class_type": "SaveImage",
              "inputs": {"images": ["6", 0], "filename_prefix": "avatar"}},
    }}

    resp = requests.post(f"{COMFYUI_URL}/prompt", json=workflow, timeout=30)
    resp.raise_for_status()
    prompt_id = resp.json()["prompt_id"]

    # Poll for completion
    for _ in range(120):
        time.sleep(1)
        try:
            hist = requests.get(f"{COMFYUI_URL}/history/{prompt_id}", timeout=5).json()
            if prompt_id in hist and "outputs" in hist[prompt_id]:
                for node_out in hist[prompt_id]["outputs"].values():
                    if "images" in node_out:
                        img = node_out["images"][0]
                        fname = img["filename"]
                        subfolder = img.get("subfolder", "")
                        if subfolder:
                            return f"{os.path.expanduser('~/ComfyUI/output')}/{subfolder}/{fname}"
                        return f"{os.path.expanduser('~/ComfyUI/output')}/{fname}"
        except Exception:
            pass
    raise TimeoutError("Image generation timed out")

def kokoro_tts(text, voice="am_fenrir"):
    """Generate TTS audio via Trinity voice server."""
    resp = requests.post(f"{VOICE_URL}/api/tts/generate",
                         json={"text": text, "voice": voice}, timeout=30)
    resp.raise_for_status()
    return resp.content  # WAV bytes

def step(name, fn):
    """Run a pipeline step with status reporting."""
    print(f"  ▶ {name}...", end=" ", flush=True)
    t0 = time.time()
    try:
        result = fn()
        dt = time.time() - t0
        print(f"✓ ({dt:.1f}s)")
        return result
    except Exception as e:
        dt = time.time() - t0
        print(f"✗ ({dt:.1f}s) — {e}")
        return None


def run_pipeline(concept, style="steampunk", voice="am_fenrir"):
    print(f"\n{'='*60}")
    print(f"  AVATAR PIPELINE — {concept[:50]}")
    print(f"  Style: {style} | Voice: {voice}")
    print(f"{'='*60}\n")

    # 1. Character Sheet
    def make_character():
        raw = llm(
            f"Create a complete NPC character sheet as JSON for: {concept}\n"
            f"Style: {style}\n"
            f"JSON fields: name, age, backstory, personality_traits (list), "
            f"visual_description (detailed for image generation), voice_style, "
            f"theme_music_mood, dialogue_lines (list of 3).\n"
            f"Output ONLY valid JSON, no markdown."
        )
        # Extract JSON from response (Mistral may wrap in thinking or markdown)
        import re
        # Try direct parse
        try:
            return json.loads(raw)
        except json.JSONDecodeError:
            pass
        # Try extracting JSON block from markdown
        m = re.search(r'```(?:json)?\s*(\{.*?\})\s*```', raw, re.DOTALL)
        if m:
            return json.loads(m.group(1))
        # Try finding first { to last }
        start = raw.find('{')
        end = raw.rfind('}')
        if start >= 0 and end > start:
            return json.loads(raw[start:end+1])
        raise ValueError("No JSON found in LLM response")

    character = step("Write character sheet", make_character)

    if not character:
        # Fallback: try without strict JSON
        raw = llm(f"Create a short NPC character for: {concept}. Style: {style}. "
                   "Give name, backstory, visual description, 3 dialogue lines.")
        character = {
            "name": concept.split()[-1].title(),
            "backstory": raw,
            "visual_description": f"{concept}, {style} aesthetic",
            "dialogue_lines": ["Greetings, traveler.", "The road is long.", "Stay sharp."],
            "theme_music_mood": f"{style} adventure",
            "voice_style": "deep and weathered"
        }
        print(f"  (using fallback character sheet)")

    name = character.get("name", "Unknown").replace(" ", "_")
    outdir = OUTPUT_BASE / name
    outdir.mkdir(parents=True, exist_ok=True)

    # Save character sheet
    (outdir / "character.json").write_text(json.dumps(character, indent=2))
    print(f"  📋 Character: {character.get('name', 'Unknown')}")

    # 2. Portrait
    vis = character.get("visual_description", concept)
    portrait_path = step("Generate portrait", lambda: comfyui_image(
        f"{vis}, {style} aesthetic, character portrait, detailed face, upper body, high quality",
        width=1024, height=1024
    ))

    if portrait_path:
        import shutil
        dest = outdir / "portrait.png"
        shutil.copy2(portrait_path, dest)
        print(f"  🖼️  Portrait: {dest}")

    # 3. Voice sample
    dialogue = character.get("dialogue_lines", ["Greetings, traveler."])
    voice_bytes = step("Generate voice sample", lambda: kokoro_tts(dialogue[0], voice))

    if voice_bytes:
        voice_path = outdir / "voice_sample.wav"
        voice_path.write_bytes(voice_bytes)
        print(f"  🎤 Voice: {voice_path}")

    # 4. Bevy ECS entity
    entity_code = step("Generate Bevy ECS entity", lambda: llm(
        f"Generate a Rust Bevy 0.18 ECS entity spawn function for NPC '{character.get('name', 'NPC')}'.\n"
        f"Include: Transform, mesh handle for 'model.glb', AudioPlayer for 'theme.wav',\n"
        f"a Dialogue component with lines: {json.dumps(dialogue)},\n"
        f"and a PersonalityTraits component with: {json.dumps(character.get('personality_traits', []))}.\n"
        f"Output ONLY Rust code, no markdown fences."
    ))

    if entity_code:
        (outdir / "entity.rs").write_text(entity_code)
        print(f"  🦀 Entity: {outdir / 'entity.rs'}")

    # 5. Trellis 3D (skip if not available)
    # TODO: Wire when trellis pip package is installable
    print(f"  ⏭️  3D mesh: skipped (Trellis not installed)")

    # 6. Music (skip if not available)
    # TODO: Wire when ACE-Step local server is available
    print(f"  ⏭️  Theme music: skipped (ACE-Step not installed)")

    # 7. Video (skip if compute-heavy)
    print(f"  ⏭️  Presentation video: skipped (HunyuanVideo on-demand)")

    # Summary
    files = list(outdir.iterdir())
    print(f"\n{'='*60}")
    print(f"  ✅ AVATAR COMPLETE: {character.get('name', 'Unknown')}")
    print(f"  📁 {outdir}/")
    for f in sorted(files):
        print(f"     {f.name} ({f.stat().st_size:,} bytes)")
    print(f"{'='*60}\n")

    return str(outdir)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Avatar Pipeline — NPC Character Creation")
    parser.add_argument("concept", help="Character concept description")
    parser.add_argument("--style", default="steampunk", help="Visual style (default: steampunk)")
    parser.add_argument("--voice", default="am_fenrir", help="Kokoro voice ID (default: am_fenrir)")
    args = parser.parse_args()

    run_pipeline(args.concept, args.style, args.voice)
