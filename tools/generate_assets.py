import os
import zlib
import struct

def make_png(width, height, data):
    # Truecolor (RGB) - ColorType 2
    # 8 bits per channel
    
    # Signature
    png = b'\x89PNG\r\n\x1a\n'
    
    # IHDR
    ihdr = struct.pack('!IIBBBBB', width, height, 8, 2, 0, 0, 0)
    crc = zlib.crc32(b'IHDR' + ihdr) & 0xffffffff
    png += struct.pack('!I', len(ihdr)) + b'IHDR' + ihdr + struct.pack('!I', crc)
    
    # IDAT
    raw_data = b''
    for y in range(height):
        raw_data += b'\x00' # Filter type 0 (None)
        raw_data += data[y * width * 3 : (y + 1) * width * 3]
        
    compressed = zlib.compress(raw_data)
    crc = zlib.crc32(b'IDAT' + compressed) & 0xffffffff
    png += struct.pack('!I', len(compressed)) + b'IDAT' + compressed + struct.pack('!I', crc)
    
    # IEND
    png += struct.pack('!I', 0) + b'IEND' + struct.pack('!I', zlib.crc32(b'IEND'))
    
    return png

# Simple 5x7 font map (A-Z, 0-9, space)
# 1 = dot, 0 = empty
FONT = {
    'A': [0x70, 0x88, 0xf8, 0x88, 0x88],
    'B': [0xf0, 0x88, 0xf0, 0x88, 0xf0],
    'C': [0x70, 0x88, 0x80, 0x88, 0x70],
    'D': [0xe0, 0x90, 0x90, 0x90, 0xe0],
    'E': [0xf8, 0x80, 0xf0, 0x80, 0xf8],
    'F': [0xf8, 0x80, 0xf0, 0x80, 0x80],
    'G': [0x70, 0x88, 0xb8, 0x88, 0x78],
    'H': [0x88, 0x88, 0xf8, 0x88, 0x88],
    'I': [0x70, 0x20, 0x20, 0x20, 0x70],
    'J': [0x38, 0x10, 0x10, 0x90, 0x60],
    'K': [0x90, 0xa0, 0xc0, 0xa0, 0x90],
    'L': [0x80, 0x80, 0x80, 0x80, 0xf8],
    'M': [0x88, 0xd8, 0xa8, 0x88, 0x88],
    'N': [0x88, 0xc8, 0xa8, 0x98, 0x88],
    'O': [0x70, 0x88, 0x88, 0x88, 0x70],
    'P': [0xf0, 0x88, 0xf0, 0x80, 0x80],
    'Q': [0x70, 0x88, 0x88, 0xa8, 0x78],
    'R': [0xf0, 0x88, 0xf0, 0xa0, 0x90],
    'S': [0x78, 0x80, 0x70, 0x08, 0xf0],
    'T': [0xf8, 0x20, 0x20, 0x20, 0x20],
    'U': [0x88, 0x88, 0x88, 0x88, 0x70],
    'V': [0x88, 0x88, 0x88, 0x50, 0x20],
    'W': [0x88, 0x88, 0xa8, 0xd8, 0x88],
    'X': [0x88, 0x50, 0x20, 0x50, 0x88],
    'Y': [0x88, 0x88, 0x70, 0x20, 0x20],
    'Z': [0xf8, 0x10, 0x20, 0x40, 0xf8],
    ' ': [0x00, 0x00, 0x00, 0x00, 0x00],
    '_': [0x00, 0x00, 0x00, 0x00, 0xf8],
}

def draw_char(pixels, width, height, char, cx, cy, color):
    if char.upper() not in FONT: return
    rows = FONT[char.upper()]
    for r, row_byte in enumerate(rows):
        for c in range(5): # 5 bits wide
            if (row_byte >> (4 - c)) & 1:
                px = cx + c
                py = cy + r
                if 0 <= px < width and 0 <= py < height:
                    idx = (py * width + px) * 3
                    pixels[idx] = color[0]
                    pixels[idx+1] = color[1]
                    pixels[idx+2] = color[2]

def create_asset(path, name, bg_color, text_color, text, width=32, height=32):
    # Create background
    pixels = bytearray()
    for _ in range(width * height):
        pixels.extend(bg_color)
        
    # Draw text (centered-ish)
    if text:
        char_width = 6
        total_width = len(text) * char_width
        start_x = (width - total_width) // 2
        start_y = (height - 5) // 2
        
        for i, char in enumerate(text):
            draw_char(pixels, width, height, char, start_x + i * char_width, start_y, text_color)
            
    # Add border
    border_color = (text_color[0]//2, text_color[1]//2, text_color[2]//2)
    for x in range(width):
        for y in range(height):
            if x == 0 or x == width-1 or y == 0 or y == height-1:
                idx = (y * width + x) * 3
                pixels[idx] = border_color[0]
                pixels[idx+1] = border_color[1]
                pixels[idx+2] = border_color[2]

    with open(path, 'wb') as f:
        f.write(make_png(width, height, pixels))
    print(f"Created {path}")

# Colors (R, G, B)
C_STONE = (100, 100, 110)
C_TECH = (50, 60, 80)
C_WALL = (150, 150, 160)
C_WALL_TOP = (180, 180, 190)
C_TERM = (0, 200, 100)
C_DOOR_L = (200, 50, 50)
C_DOOR_O = (50, 200, 50)
C_VOID = (20, 20, 30)

C_PLAYER = (100, 200, 255)
C_NPC = (200, 200, 100)

C_WHITE = (255, 255, 255)
C_BLACK = (0, 0, 0)
C_UI_BG = (40, 40, 50)
C_HP = (200, 50, 50)
C_XP = (50, 100, 200)

# Tiles
create_asset('src-ui/static/tiles/floor_stone.png', 'floor_stone', C_STONE, C_BLACK, 'FLR')
create_asset('src-ui/static/tiles/floor_tech.png', 'floor_tech', C_TECH, C_WHITE, 'TEC')
create_asset('src-ui/static/tiles/wall.png', 'wall', C_WALL, C_BLACK, 'WAL')
create_asset('src-ui/static/tiles/wall_top.png', 'wall_top', C_WALL_TOP, C_BLACK, 'TOP')
create_asset('src-ui/static/tiles/terminal.png', 'terminal', C_TERM, C_BLACK, 'CMD')
create_asset('src-ui/static/tiles/door_locked.png', 'door_locked', C_DOOR_L, C_WHITE, 'LCK')
create_asset('src-ui/static/tiles/door_open.png', 'door_open', C_DOOR_O, C_BLACK, 'OPN')
create_asset('src-ui/static/tiles/void.png', 'void', C_VOID, C_WHITE, 'VOID')

# Sprites
create_asset('src-ui/static/sprites/player_down.png', 'player_down', C_PLAYER, C_BLACK, 'P_D')
create_asset('src-ui/static/sprites/player_up.png', 'player_up', C_PLAYER, C_BLACK, 'P_U')
create_asset('src-ui/static/sprites/player_left.png', 'player_left', C_PLAYER, C_BLACK, 'P_L')
create_asset('src-ui/static/sprites/player_right.png', 'player_right', C_PLAYER, C_BLACK, 'P_R')
create_asset('src-ui/static/sprites/npc_mentor.png', 'npc_mentor', C_NPC, C_BLACK, 'NPC')

# UI
create_asset('src-ui/static/ui/terminal_frame.png', 'terminal_frame', C_UI_BG, C_TERM, '', width=400, height=300)
create_asset('src-ui/static/ui/health_bar_bg.png', 'health_bar_bg', C_BLACK, C_WHITE, '', width=100, height=20)
create_asset('src-ui/static/ui/health_bar_fill.png', 'health_bar_fill', C_HP, C_WHITE, '', width=100, height=20)
create_asset('src-ui/static/ui/xp_bar_bg.png', 'xp_bar_bg', C_BLACK, C_WHITE, '', width=100, height=20)
create_asset('src-ui/static/ui/xp_bar_fill.png', 'xp_bar_fill', C_XP, C_WHITE, '', width=100, height=20)

