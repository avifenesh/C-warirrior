import json
import random

# Tiled Map Configuration
MAP_WIDTH = 40
MAP_HEIGHT = 30
TILE_SIZE = 32

# Tile IDs (matches your asset pack)
TILES = {
    "WATER": 1,
    "GRASS": 2,
    "PATH": 3,
    "WALL": 4
}

def generate_dungeon():
    # 1. Initialize Grid with Water
    grid = [[TILES["WATER"] for _ in range(MAP_WIDTH)] for _ in range(MAP_HEIGHT)]

    # 2. Drunkard's Walk Algorithm
    floor_cells = 0
    target_floor_count = int(MAP_WIDTH * MAP_HEIGHT * 0.4) # 40% land
    
    x, y = MAP_WIDTH // 2, MAP_HEIGHT // 2 # Start in middle
    
    while floor_cells < target_floor_count:
        # Move randomly
        direction = random.choice([(0,1), (0,-1), (1,0), (-1,0)])
        x += direction[0]
        y += direction[1]
        
        # Clamp to bounds (leave 1 tile border)
        x = max(1, min(MAP_WIDTH - 2, x))
        y = max(1, min(MAP_HEIGHT - 2, y))
        
        # Dig
        if grid[y][x] == TILES["WATER"]:
            grid[y][x] = TILES["GRASS"]
            floor_cells += 1

    # 3. Flatten for Tiled (1D Array)
    data = []
    for row in grid:
        data.extend(row)

    # 4. Construct Tiled JSON
    tiled_map = {
        "compressionlevel": -1,
        "height": MAP_HEIGHT,
        "infinite": False,
        "layers": [
            {
                "data": data,
                "height": MAP_HEIGHT,
                "width": MAP_WIDTH,
                "id": 1,
                "name": "Tile Layer 1",
                "opacity": 1,
                "type": "tilelayer",
                "visible": True,
                "x": 0,
                "y": 0
            }
        ],
        "nextlayerid": 2,
        "nextobjectid": 1,
        "orientation": "orthogonal",
        "renderorder": "right-down",
        "tiledversion": "1.10.2",
        "tileheight": TILE_SIZE,
        "tilesets": [],
        "tilewidth": TILE_SIZE,
        "type": "map",
        "version": "1.10",
        "width": MAP_WIDTH
    }

    return json.dumps(tiled_map, indent=2)

if __name__ == "__main__":
    print(generate_dungeon())