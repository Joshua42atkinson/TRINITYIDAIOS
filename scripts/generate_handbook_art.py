import urllib.request
import urllib.error
import json
import os
import shutil
import time

PROMPTS = [
    "A beautifully illustrated fantasy tome, glowing arcane circuitry, pathfinder rpg style, digital painting, masterpiece",
    "A glowing magical mirror reflecting a young scholar, pathfinder rpg style, digital painting, rulebook art",
    "A wizard awakening in a stone bed surrounded by floating crystals, pathfinder rpg style, dramatic lighting",
    "A glowing monocle lens showing magical runes, pathfinder rpg style, wizard study, digital painting",
    "An ancient parchment character sheet glowing with blue magic, pathfinder rpg style, arcane table",
    "A glowing blue soul gem casting light on a wooden table, pathfinder rpg style, digital painting",
    "A compass of alignment glowing with radiant light, pathfinder rpg style, detailed cartography tools",
    "A heroic party leveling up in a beam of light, pathfinder rpg style, magical aura, masterpiece",
    "Two glowing magical spheres in perfect balance, light and dark, pathfinder rpg style, high fantasy",
    "An ethereal magical eye observing from the shadows, pathfinder rpg style, magical surveillance",
    "A glowing golden aura surrounding a paladin's armor, pathfinder rpg style, heroic lighting",
    "A magical toolkit overflowing with glowing tools and runes, pathfinder rpg style, digital illustration",
    "A ranger standing steward over a glowing magical forest, pathfinder rpg style, nature magic",
    "A magical forge recycling old weapons into glowing new ones, pathfinder rpg style, dwarven smithy",
    "A glowing magical antenna picking up ethereal whispers, pathfinder rpg style, wizard tower roof",
    "A glowing anvil being struck by a magical hammer, sparks flying, pathfinder rpg style, action shot",
    "A monk meditating perfectly surrounded by glowing magical energy, pathfinder rpg style, serene",
    "A glowing mind crystal being forged by invisible hammers, pathfinder rpg style, abstract magic",
    "A lively tavern with adventurers strategizing over a glowing map, pathfinder rpg style",
    "Two wizards casting identical spells into a reflecting pool, pathfinder rpg style, magical reflection",
    "A grand magical university floating in the sky, pathfinder rpg style, majestic architecture",
    "A glowing badge of prestige class mastery, pathfinder rpg style, intricate gold and jewels",
    "A glowing golden doorway at the end of a dark dungeon, pathfinder rpg style, epic conclusion",
    "A mystical author writing in a glowing book with a quill of light, pathfinder rpg style"
]

def generate():
    target_dir = os.path.join("images", "handbook_art")
    os.makedirs(target_dir, exist_ok=True)
    
    print(f"Generating {len(PROMPTS)} images to {target_dir}...")
    
    for idx, prompt in enumerate(PROMPTS):
        chapter_idx = idx + 1
        output_file = os.path.join(target_dir, f"chapter_{chapter_idx}.jpg")
        
        # Don't regenerate if it exists
        if os.path.exists(output_file):
            print(f"Skipping chapter_{chapter_idx}.jpg (already exists)")
            continue
            
        print(f"[{chapter_idx}/{len(PROMPTS)}] Hitting Trinity API: {prompt[:40]}...")
        
        data = json.dumps({
            "prompt": prompt,
            "style": "cinematic, 4k",
            "width": 1024,
            "height": 1024
        }).encode('utf-8')
        
        req = urllib.request.Request("http://127.0.0.1:3000/api/creative/image", data=data, headers={"Content-Type": "application/json"})
        
        try:
            with urllib.request.urlopen(req) as resp:
                result = json.loads(resp.read().decode('utf-8'))
                if result.get("success") and result.get("image_path"):
                    source_img = result["image_path"]
                    # Copy over to the proper destination
                    shutil.copy(source_img, output_file)
                    print(f"   -> Saved to {output_file}!")
                else:
                    print(f"   -> API Error: {result}")
        except urllib.error.URLError as e:
            if hasattr(e, 'read'):
                print(f"   -> HTTP Error: {e.code} / {e.read().decode('utf-8')}")
            else:
                print(f"   -> Connection Error: {e}")
        except Exception as e:
            print(f"   -> Critical Error: {e}")
            
        # Give ComfyUI/GPU a tiny breather
        time.sleep(1)
        
if __name__ == "__main__":
    generate()
