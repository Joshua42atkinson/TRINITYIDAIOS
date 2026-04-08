#!/usr/bin/env python3
"""
═══════════════════════════════════════════════════════════════════════════════
TRINITY ID AI OS — generate_handbook_art.py
═══════════════════════════════════════════════════════════════════════════════

PURPOSE: Generate all missing chapter splash art and spot illustrations for
         the Player Handbook ELearning viewer using the LongCat-Next Omni
         image generation API (/v1/images/generations on port 8010).

USAGE:
    python3 scripts/generate_handbook_art.py           # Generate all missing
    python3 scripts/generate_handbook_art.py --dry-run  # Preview prompts only
    python3 scripts/generate_handbook_art.py --force    # Regenerate everything
    python3 scripts/generate_handbook_art.py --spot-only # Only spot illustrations
    python3 scripts/generate_handbook_art.py --splash-only # Only splash art

OUTPUT:
    crates/trinity/frontend/public/audiobook_art/chapter_N.jpg      (splash)
    crates/trinity/frontend/public/audiobook_art/chapter_N_spot.jpg  (spot)

═══════════════════════════════════════════════════════════════════════════════
"""

import os
import sys
import json
import time
import base64
import argparse
import requests
from pathlib import Path

# ─── Configuration ───────────────────────────────────────────────────────────
LONGCAT_URL = "http://127.0.0.1:8010"
API_ENDPOINT = f"{LONGCAT_URL}/v1/images/generations"
HEALTH_ENDPOINT = f"{LONGCAT_URL}/health"

# Output directory for audiobook art
SCRIPT_DIR = Path(__file__).resolve().parent.parent
OUTPUT_DIR = SCRIPT_DIR / "crates" / "trinity" / "frontend" / "public" / "audiobook_art"

# Image sizes
SPLASH_SIZE = "768x768"   # Full-page chapter splash art
SPOT_SIZE = "512x512"     # In-text circular spot illustrations

# Unified style suffix for Trinity/Purdue aesthetic
STYLE_SUFFIX = (
    "oil painting style, warm sepia and burnished gold tones, "
    "19th century industrial romanticism, atmospheric lighting, "
    "rich textures, Purdue boilermaker aesthetic, steam and iron, "
    "high detail, dramatic chiaroscuro, no text, no words, no letters"
)

SPOT_STYLE_SUFFIX = (
    "small circular vignette illustration, pen and ink with gold wash, "
    "warm sepia tones, vintage engraving quality, isolated object, "
    "clean white background edges fading out, no text, no words"
)

# ─── Chapter Art Prompts ─────────────────────────────────────────────────────
CHAPTER_ART = [
    (1,
     "A massive leather-bound book glowing with inner golden light, floating above an iron railroad track that stretches into a luminous horizon, steam rising from the rails, constellations visible in the sky above",
     "An ornate golden key resting on aged parchment with faint railroad track lines"),
    (2,
     "A solitary figure standing at a crossroads of iron railroad tracks under a vast starlit Indiana sky, one path leading to a glowing medieval castle, the other to a towering industrial forge, golden light emanating from the figure",
     "A compass rose made of interlocking railroad tracks and golden gears"),
    (3,
     "A Marine in dress blues standing in a small house, his reflection in the window showing a luminous golden silhouette detaching from his body, representing metacognitive awakening, warm amber light flooding the room",
     "A cracked military dog tag with golden light streaming through the fracture"),
    (4,
     "A massive ornate telescope made of brass and iron pointing at the night sky, the lens is a human eye, the view through the eyepiece shows two different worlds simultaneously rendered in warm gold",
     "A prism splitting a single beam of light into multiple colored paths"),
    (5,
     "An enormous unfurled character sheet scroll made of hammered copper and gold leaf, floating above a blacksmith anvil, four glowing radar chart axes emanating from the center, each axis a different colored flame",
     "A golden radar chart with four glowing axes on aged parchment"),
    (6,
     "A father standing in a warm kitchen at dusk, six children around him, his shadow on the wall behind him showing a towering armored knight, the contrast between the gentle domestic scene and warrior shadow, warm golden lamplight",
     "A geometric diamond shape with four uneven points, one axis shorter than others, glowing gold"),
    (7,
     "A Marine standing at the edge of a cliff at sunset reviewing his discharge papers, below him a vast network of golden threads connecting to distant figures, showing the tension between autonomy and relatedness, dramatic sky",
     "Three interlocking circles in red blue and gold representing Autonomy Relatedness Competence"),
    (8,
     "A massive pendulum swinging between two towering iron pillars in a cathedral-like industrial hall, one pillar carved as a heroic knight, the other as a raging outlaw, golden light pooling in the center where the pendulum reaches equilibrium",
     "A pendulum at rest perfectly centered casting a golden shadow"),
    (9,
     "A glass cannon made of crystal and brass exploding in beautiful slow motion, shattering into a thousand luminous fragments against a dark backdrop, each shard reflecting a different scene, warm amber explosions",
     "A cracked glass sphere with golden light leaking from every fracture"),
    (10,
     "A solitary figure sitting in a vast library, the books around them have opened and their pages have become floating golden streams of light, the figure observes their own thoughts as luminous threads",
     "An open eye made of golden clockwork gears, iris glowing with warm light"),
    (11,
     "A wide golden river flowing through a canyon of rust-colored iron cliffs, the water is liquid gold, smooth boulders in the river have been polished by centuries of flow, a single figure stands in the shallows, warm sunset light",
     "A smooth river stone with golden veins running through it resting in clear water"),
    (12,
     "A grand forge workshop with four stations arranged in a cross, one with scrolls and quills, one with a golden recycling symbol, one with a cracked bowl repaired with gold, one with a blazing furnace, warm forge-light",
     "A set of four golden tools arranged in a cross pattern on dark leather"),
    (13,
     "A bartender behind a magnificent mahogany bar, pouring liquid golden light from an ornate decanter into a crystal glass, the patron across the counter is illuminated by the glow, warm tavern atmosphere with brass fixtures",
     "A crystal glass filled with golden luminous liquid casting warm light on the bar surface"),
    (14,
     "A massive golden recycling machine in an industrial cathedral, military medals and uniforms go in one end, pure golden tools and skills emerge from the other end, the operator stands at the controls looking forward, steam and sparks",
     "A golden gear with the recycling symbol etched into its face spinning"),
    (15,
     "A Japanese kintsugi bowl being repaired by invisible hands, golden lacquer flowing into the cracks like rivers of molten light, the broken pieces reassembling into something more beautiful than original, dark atmospheric background",
     "A cracked ceramic bowl with golden light streaming through repaired seams"),
    (16,
     "A massive blacksmith forge carved into a mountain, the furnace mouth glowing with white-hot heat, an anvil of impossible scale catching sparks that rise like fireflies, a single figure silhouetted against the forge light",
     "A glowing anvil with a single hammer resting on top sparks frozen in time"),
    (17,
     "A human silhouette made entirely of glowing forge-metal, the gut region containing a visible firebox with clean-burning flames, the chest containing a pressure vessel of luminous mist, the head crowned with clockwork gears",
     "A small glowing furnace shaped like a human torso clean flames inside"),
    (18,
     "A figure standing under a waterfall of freezing water in a granite cave, steam rising from their body, their hands clenched jaw set, the cold water turning to golden mist on contact with the figure, dramatic lighting",
     "A shower handle turned to cold frost crystals forming on the metal golden light behind"),
    (19,
     "A grand banquet hall with long tables of warm dark wood, golden chandeliers, dozens of figures seated together each with a glowing character sheet hovering above them, warm communal light through tall windows",
     "Two clasped hands with golden light at the point of connection"),
    (20,
     "Two faces in profile looking at each other across a golden mirror that floats between them, each face reflects the other but with the same inner golden light shining through both eyes, intimate and profound warm gold tones",
     "A small golden hand mirror reflecting warm light back at the viewer"),
    (21,
     "A circle of glowing lanterns arranged in a quad at night, each lantern represents a person in the circle of trust, thin golden threads connecting them, a dark campus bell tower rises behind, intimate warm light against cold night",
     "A single lantern with golden flame casting warm light in a dark space"),
    (22,
     "Four archetypal RPG character silhouettes standing at the four cardinal directions, The Sage north in blue-gold, The Hero east in red-gold, The Jester west in green-gold, The Caregiver south in amber-gold, a glowing D20 die at center",
     "A glowing D20 die made of golden metal each face etched with a different symbol"),
    (23,
     "A vast Iron Road stretching to the horizon under a sunrise sky, the tracks are made of gold, a single locomotive approaches emitting not smoke but luminous golden light, the tracks behind it transform from iron to gold as it passes",
     "A golden railroad spike freshly driven into a wooden tie gleaming"),
    (24,
     "A humble craftsman workbench in a warm home workshop, scattered blueprints of the Trinity system, a Purdue pennant on the wall, six small wooden figures on a shelf, a laptop running code, crayons on the desk, warm domestic light",
     "A simple wooden nameplate with a golden pen resting on it"),
]


def check_health():
    """Verify LongCat is loaded and ready."""
    try:
        resp = requests.get(HEALTH_ENDPOINT, timeout=5)
        data = resp.json()
        if data.get("loaded"):
            print(f"✅ LongCat-Next is ONLINE — mode: {data.get('mode')}")
            return True
        else:
            print("⚠️  LongCat is running but model not yet loaded")
            return False
    except Exception as e:
        print(f"❌ LongCat health check failed: {e}")
        return False


def generate_image(prompt, size, output_path):
    """Generate a single image via LongCat API and save to disk."""
    try:
        payload = {"prompt": prompt, "size": size}
        start = time.time()
        resp = requests.post(API_ENDPOINT, json=payload, timeout=300)
        elapsed = time.time() - start

        if resp.status_code != 200:
            print(f"   ❌ HTTP {resp.status_code}: {resp.text[:200]}")
            return False

        data = resp.json()
        img_data = data.get("data", [{}])[0].get("b64_json", "")
        if not img_data or len(img_data) < 100:
            print(f"   ⚠️  Got empty/mock image response ({len(img_data)} bytes)")
            return False

        img_bytes = base64.b64decode(img_data)
        output_path.parent.mkdir(parents=True, exist_ok=True)
        with open(output_path, "wb") as f:
            f.write(img_bytes)

        size_kb = len(img_bytes) / 1024
        print(f"   ✅ Saved {output_path.name} ({size_kb:.0f}KB, {elapsed:.1f}s)")
        return True
    except requests.Timeout:
        print(f"   ❌ Timeout (300s) generating image")
        return False
    except Exception as e:
        print(f"   ❌ Error: {e}")
        return False


def main():
    parser = argparse.ArgumentParser(description="Generate Player Handbook art via LongCat")
    parser.add_argument("--dry-run", action="store_true", help="Preview prompts without generating")
    parser.add_argument("--force", action="store_true", help="Regenerate even if files exist")
    parser.add_argument("--spot-only", action="store_true", help="Only generate spot illustrations")
    parser.add_argument("--splash-only", action="store_true", help="Only generate splash art")
    parser.add_argument("--chapter", type=int, help="Generate only a specific chapter number")
    args = parser.parse_args()

    print("═" * 60)
    print("  TRINITY ID AI OS — Handbook Art Generator")
    print("  Using LongCat-Next DiNA Visual Token Pipeline")
    print("═" * 60)
    print()

    if not args.dry_run:
        if not check_health():
            print("\n💀 Cannot proceed — LongCat is not available.")
            sys.exit(1)
        print()

    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

    queue = []
    for idx, splash_prompt, spot_prompt in CHAPTER_ART:
        if args.chapter and idx != args.chapter:
            continue
        splash_path = OUTPUT_DIR / f"chapter_{idx}.jpg"
        spot_path = OUTPUT_DIR / f"chapter_{idx}_spot.jpg"

        if not args.spot_only:
            if args.force or not splash_path.exists():
                queue.append(("SPLASH", idx, f"{splash_prompt}, {STYLE_SUFFIX}", SPLASH_SIZE, splash_path))
            else:
                print(f"   ⏭️  chapter_{idx}.jpg exists — skipping")

        if not args.splash_only:
            if args.force or not spot_path.exists():
                queue.append(("SPOT", idx, f"{spot_prompt}, {SPOT_STYLE_SUFFIX}", SPOT_SIZE, spot_path))
            else:
                print(f"   ⏭️  chapter_{idx}_spot.jpg exists — skipping")

    print(f"\n📋 Generation queue: {len(queue)} images")

    if args.dry_run:
        print("\n🔍 DRY RUN — Prompts preview:\n")
        for img_type, idx, prompt, size, path in queue:
            print(f"  [{img_type}] chapter_{idx} ({size})")
            print(f"    Prompt: {prompt[:120]}...")
            print(f"    Output: {path}")
            print()
        print("Done. Remove --dry-run to generate.")
        return

    total = len(queue)
    success = 0
    failed = 0
    start_all = time.time()

    for i, (img_type, idx, prompt, size, path) in enumerate(queue):
        print(f"\n[{i+1}/{total}] {img_type} chapter_{idx} ({size})")
        print(f"   Prompt: {prompt[:100]}...")
        if generate_image(prompt, size, path):
            success += 1
        else:
            failed += 1

    elapsed_all = time.time() - start_all
    print(f"\n{'═' * 60}")
    print(f"  DONE — {success}/{total} images generated ({elapsed_all:.0f}s total)")
    if failed:
        print(f"  ⚠️  {failed} images failed — re-run to retry (skips existing)")
    print(f"  Output: {OUTPUT_DIR}")
    print(f"{'═' * 60}")


if __name__ == "__main__":
    main()
