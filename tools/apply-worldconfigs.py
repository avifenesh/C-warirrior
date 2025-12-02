#!/usr/bin/env python3
"""
Apply generated world_configs to levels.json
"""

import json

# Load current levels.json
with open("src/assets/levels.json", "r") as f:
    levels = json.load(f)

# Load generated world configs
with open("/tmp/world_configs.json", "r") as f:
    world_configs = json.load(f)

# Update each level
updated_count = 0
for level in levels:
    level_id = level["id"]
    if level_id in world_configs:
        new_config = world_configs[level_id]

        # Preserve existing properties that we don't want to overwrite
        old_config = level.get("world_config", {})

        # Update with new values
        level["world_config"] = {
            "width": new_config["width"],
            "height": new_config["height"],
            "spawn_x": new_config["spawn_x"],
            "spawn_y": new_config["spawn_y"],
            "terminals": new_config["terminals"]
        }

        # Add tiles if present
        if "tiles" in new_config:
            level["world_config"]["tiles"] = new_config["tiles"]

        # Preserve preset if it existed
        if "preset" in old_config:
            level["world_config"]["preset"] = old_config["preset"]

        updated_count += 1
        print(f"Updated {level_id}: spawn=({new_config['spawn_x']}, {new_config['spawn_y']}), terminals={len(new_config['terminals'])}")

# Save updated levels.json
with open("src/assets/levels.json", "w") as f:
    json.dump(levels, f, indent=2)

print(f"\nUpdated {updated_count}/25 levels in src/assets/levels.json")
