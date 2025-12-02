#!/usr/bin/env python3
"""
Generate themed NPC and terminal sprites for Code Warrior using Gemini Image Generation API.
Each level gets a unique NPC character and terminal/pedestal matching its theme.
"""

import os
import base64
import json
import time
import urllib.request
import urllib.error
from pathlib import Path

# Gemini API configuration
GEMINI_API_KEY = "AIzaSyDGJFit3LUO-9mNkrqF1xTZVC71-yjGQlU"

# Use the image generation model
GEMINI_API_URL = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-exp-image-generation:generateContent"

# Paths
THEMES_DIR = Path("/Users/avifen/C-warirrior/src-ui/static/tiles/themes")

# Theme definitions with NPC and terminal descriptions
THEMES = {
    "L01_village": {
        "npc": "friendly village elder with wooden staff, grey beard, brown robes",
        "terminal": "wooden notice board with quest scroll pinned"
    },
    "L02_market": {
        "npc": "merchant with apron and coin pouch, friendly face",
        "terminal": "market stall counter with goods displayed"
    },
    "L03_tower": {
        "npc": "armored tower guard with spear, helmet",
        "terminal": "stone command pedestal with scrolls"
    },
    "L04_forest": {
        "npc": "forest spirit with glowing green aura, ethereal",
        "terminal": "mushroom altar with glowing crystals"
    },
    "L05_darkwoods": {
        "npc": "hooded ranger in dark cloak, mysterious",
        "terminal": "corrupted tree stump altar with dark energy"
    },
    "L06_river": {
        "npc": "fisherman with rod and straw hat",
        "terminal": "wooden dock post with rope tied"
    },
    "L07_cavern": {
        "npc": "cave hermit with lantern, long beard",
        "terminal": "crystal formation pedestal glowing cyan"
    },
    "L08_mountain": {
        "npc": "mountain climber with pick, warm clothes",
        "terminal": "stone cairn marker stacked rocks"
    },
    "L09_ice": {
        "npc": "frost mage in blue robes, icy staff",
        "terminal": "frozen ice pedestal with crystals"
    },
    "L10_temple": {
        "npc": "temple monk in saffron robes, peaceful",
        "terminal": "sacred stone altar with candles"
    },
    "L11_library": {
        "npc": "scholar ghost with floating book, transparent",
        "terminal": "magical lectern with glowing tome"
    },
    "L12_crypt": {
        "npc": "gravekeeper with lantern, hunched",
        "terminal": "tombstone altar with carved runes"
    },
    "L13_lake": {
        "npc": "water spirit with glowing blue form, ethereal",
        "terminal": "lily pad shrine floating on water"
    },
    "L14_forge": {
        "npc": "blacksmith with hammer, muscular, apron",
        "terminal": "anvil workstation with hot metal"
    },
    "L15_lair": {
        "npc": "treasure hunter with map and torch",
        "terminal": "golden treasure pile with gems"
    },
    "L16_courtyard": {
        "npc": "royal gardener with shears, green apron",
        "terminal": "garden fountain pedestal with water"
    },
    "L17_throne": {
        "npc": "royal herald with scroll, fancy clothes",
        "terminal": "golden throne pedestal ornate"
    },
    "L18_treasury": {
        "npc": "royal treasurer with keys, counting",
        "terminal": "locked treasure chest golden"
    },
    "L19_dungeon": {
        "npc": "dungeon guard with torch, rugged",
        "terminal": "iron cage display with chains"
    },
    "L20_passage": {
        "npc": "hooded spy with dagger, shadowy",
        "terminal": "hidden lever mechanism on wall"
    },
    "L21_stairs": {
        "npc": "tower watchman with horn, alert",
        "terminal": "spiral stone marker carved"
    },
    "L22_alchemy": {
        "npc": "alchemist with bubbling flask, goggles",
        "terminal": "bubbling cauldron station with potions"
    },
    "L23_observatory": {
        "npc": "stargazer with telescope, robes",
        "terminal": "astral orrery display with planets"
    },
    "L24_portal": {
        "npc": "portal guardian in arcane armor, glowing",
        "terminal": "active portal frame with energy"
    },
    "L25_sanctum": {
        "npc": "grand archmage with staff, powerful aura",
        "terminal": "nexus power crystal on pedestal"
    }
}

def generate_sprite(prompt: str, output_path: Path) -> bool:
    """Generate a 32x32 pixel art sprite using Gemini Image Generation API."""

    full_prompt = f"""Generate a 32x32 pixel art game sprite image.
Style: 16-bit SNES RPG, dark fantasy aesthetic, clean pixel art.
Subject: {prompt}
Requirements:
- Small 32x32 pixel sprite size
- Top-down RPG view perspective
- Clean pixel outlines, no anti-aliasing
- Limited color palette (16-32 colors)
- Single character/object centered in frame
- Game asset quality ready for tilemap
- Dark background or transparent

Generate the pixel art image now."""

    data = {
        "contents": [{
            "parts": [{
                "text": full_prompt
            }]
        }],
        "generationConfig": {
            "responseModalities": ["image", "text"]
        }
    }

    try:
        req = urllib.request.Request(
            f"{GEMINI_API_URL}?key={GEMINI_API_KEY}",
            data=json.dumps(data).encode('utf-8'),
            headers={"Content-Type": "application/json"},
            method="POST"
        )

        with urllib.request.urlopen(req, timeout=120) as response:
            result = json.loads(response.read().decode('utf-8'))

        # Extract image data from response
        if "candidates" in result and result["candidates"]:
            candidate = result["candidates"][0]
            if "content" in candidate and "parts" in candidate["content"]:
                for part in candidate["content"]["parts"]:
                    if "inlineData" in part:
                        image_data = part["inlineData"]["data"]
                        mime_type = part["inlineData"].get("mimeType", "image/png")
                        # Decode and save
                        image_bytes = base64.b64decode(image_data)
                        output_path.write_bytes(image_bytes)
                        print(f"  Saved: {output_path} ({mime_type}, {len(image_bytes)} bytes)")
                        return True

        print(f"  Error: No image in response")
        # Show response structure for debugging
        if "candidates" in result:
            print(f"  Candidates: {len(result['candidates'])}")
            if result["candidates"]:
                parts = result["candidates"][0].get("content", {}).get("parts", [])
                print(f"  Parts: {[list(p.keys()) for p in parts]}")
        return False

    except urllib.error.HTTPError as e:
        print(f"  HTTP Error: {e.code} - {e.reason}")
        error_body = e.read().decode('utf-8')[:500]
        print(f"  Details: {error_body}")
        return False
    except Exception as e:
        print(f"  Error: {e}")
        return False

def main():
    print("=" * 60)
    print("Generating themed sprites for Code Warrior")
    print("=" * 60)

    success_count = 0
    fail_count = 0

    for theme_id, descriptions in THEMES.items():
        theme_dir = THEMES_DIR / theme_id
        if not theme_dir.exists():
            print(f"\nSkipping {theme_id} - directory doesn't exist")
            continue

        print(f"\n[{theme_id}]")

        # Generate NPC sprite
        npc_path = theme_dir / "npc.png"
        if not npc_path.exists():
            print(f"  Generating NPC: {descriptions['npc']}")
            if generate_sprite(descriptions["npc"], npc_path):
                success_count += 1
            else:
                fail_count += 1
            time.sleep(3)  # Rate limiting
        else:
            print(f"  NPC already exists, skipping")

        # Generate terminal sprite
        terminal_path = theme_dir / "terminal.png"
        if not terminal_path.exists():
            print(f"  Generating terminal: {descriptions['terminal']}")
            if generate_sprite(descriptions["terminal"], terminal_path):
                success_count += 1
            else:
                fail_count += 1
            time.sleep(3)  # Rate limiting
        else:
            print(f"  Terminal already exists, skipping")

    print("\n" + "=" * 60)
    print(f"Done! Success: {success_count}, Failed: {fail_count}")
    print("=" * 60)

if __name__ == "__main__":
    main()
