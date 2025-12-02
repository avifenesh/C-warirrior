// Level Theme Configuration
// Each theme defines visual identity and AI generation prompts for tile assets

export interface ThemeConfig {
    id: string;
    name: string;
    description: string;
    floorDesc: string;
    wallDesc: string;
    decoration1: string;
    decoration2: string;
    colorPalette: string;
    // NPC and terminal descriptions for themed sprites
    npcDesc: string;      // Quest giver character description
    terminalDesc: string; // Quest pedestal/altar description
}

export const LEVEL_THEMES: Record<string, ThemeConfig> = {
    // === Chapter 1: Village & Surroundings ===
    'L01_village': {
        id: 'L01_village',
        name: 'Starter Village',
        description: 'Cozy cottage village with cobblestone paths',
        floorDesc: 'worn cobblestone path',
        wallDesc: 'wooden fence with vines',
        decoration1: 'barrel',
        decoration2: 'wooden signpost',
        colorPalette: 'warm browns, soft greens, golden wood',
        npcDesc: 'friendly village elder with wooden staff',
        terminalDesc: 'wooden notice board with quest scroll'
    },
    'L02_market': {
        id: 'L02_market',
        name: 'Market Square',
        description: 'Busy merchant area with stalls',
        floorDesc: 'checkered market tiles',
        wallDesc: 'market stall wooden frame',
        decoration1: 'crate of goods',
        decoration2: 'hanging lantern',
        colorPalette: 'warm reds, yellows, market colors',
        npcDesc: 'merchant with apron and coin pouch',
        terminalDesc: 'market stall counter with goods'
    },
    'L03_tower': {
        id: 'L03_tower',
        name: 'Guard Tower',
        description: 'Stone military tower with iron gates',
        floorDesc: 'grey stone bricks',
        wallDesc: 'fortified stone wall',
        decoration1: 'weapon rack',
        decoration2: 'torch sconce',
        colorPalette: 'cold greys, iron black, torch orange',
        npcDesc: 'armored tower guard with spear',
        terminalDesc: 'stone command pedestal'
    },

    // === Chapter 2: Forest Region ===
    'L04_forest': {
        id: 'L04_forest',
        name: 'Enchanted Forest',
        description: 'Magical forest with glowing flora',
        floorDesc: 'mossy grass with small flowers',
        wallDesc: 'thick tree trunk',
        decoration1: 'glowing mushroom',
        decoration2: 'ancient tree stump',
        colorPalette: 'deep greens, magical cyan glow, earth browns',
        npcDesc: 'forest spirit with glowing aura',
        terminalDesc: 'mushroom altar with crystals'
    },
    'L05_darkwoods': {
        id: 'L05_darkwoods',
        name: 'Dark Woods',
        description: 'Corrupted forest with dead trees',
        floorDesc: 'dead leaves and dark soil',
        wallDesc: 'gnarled dead tree',
        decoration1: 'thorny bramble',
        decoration2: 'skull on stake',
        colorPalette: 'dark purples, dead browns, eerie green fog',
        npcDesc: 'hooded ranger in dark cloak',
        terminalDesc: 'corrupted tree stump altar'
    },

    // === Chapter 3: Water Region ===
    'L06_river': {
        id: 'L06_river',
        name: 'River Crossing',
        description: 'Wooden bridges over rushing water',
        floorDesc: 'wooden planks bridge',
        wallDesc: 'river reeds and rocks',
        decoration1: 'fishing net',
        decoration2: 'wooden post with rope',
        colorPalette: 'water blues, wood browns, green reeds',
        npcDesc: 'fisherman with rod and hat',
        terminalDesc: 'wooden dock post with rope'
    },
    'L07_cavern': {
        id: 'L07_cavern',
        name: 'Waterfall Cavern',
        description: 'Wet cave behind a waterfall',
        floorDesc: 'wet stone with puddles',
        wallDesc: 'dripping cave wall',
        decoration1: 'stalactite',
        decoration2: 'glowing crystal',
        colorPalette: 'dark blues, wet greys, crystal cyan',
        npcDesc: 'cave hermit with lantern',
        terminalDesc: 'crystal formation pedestal'
    },

    // === Chapter 4: Mountain Region ===
    'L08_mountain': {
        id: 'L08_mountain',
        name: 'Mountain Base',
        description: 'Rocky terrain with snow patches',
        floorDesc: 'rocky ground with pebbles',
        wallDesc: 'mountain boulder',
        decoration1: 'small pine tree',
        decoration2: 'snow pile',
        colorPalette: 'stone greys, snow white, evergreen',
        npcDesc: 'mountain climber with pick',
        terminalDesc: 'stone cairn marker'
    },
    'L09_ice': {
        id: 'L09_ice',
        name: 'Ice Cave',
        description: 'Frozen cavern with crystals',
        floorDesc: 'slippery ice floor',
        wallDesc: 'frozen ice wall',
        decoration1: 'ice crystal formation',
        decoration2: 'frozen pillar',
        colorPalette: 'icy blues, white, crystal reflections',
        npcDesc: 'frost mage in blue robes',
        terminalDesc: 'frozen ice pedestal'
    },
    'L10_temple': {
        id: 'L10_temple',
        name: 'Summit Temple',
        description: 'Ancient mountain shrine',
        floorDesc: 'ancient stone tiles',
        wallDesc: 'temple pillar',
        decoration1: 'prayer flag',
        decoration2: 'stone altar',
        colorPalette: 'ancient gold, faded red, weathered stone',
        npcDesc: 'temple monk in saffron robes',
        terminalDesc: 'sacred stone altar'
    },

    // === Chapter 5: Ruins Region ===
    'L11_library': {
        id: 'L11_library',
        name: 'Ruined Library',
        description: 'Collapsed archive with scattered books',
        floorDesc: 'broken tile with papers',
        wallDesc: 'collapsed bookshelf',
        decoration1: 'pile of books',
        decoration2: 'broken reading desk',
        colorPalette: 'dusty browns, faded parchment, wood',
        npcDesc: 'scholar ghost with floating book',
        terminalDesc: 'magical lectern with tome'
    },
    'L12_crypt': {
        id: 'L12_crypt',
        name: 'Crypt',
        description: 'Underground burial chamber',
        floorDesc: 'cracked stone tomb floor',
        wallDesc: 'stone crypt wall with niches',
        decoration1: 'cobweb',
        decoration2: 'bone pile',
        colorPalette: 'death greys, eerie green, bone white',
        npcDesc: 'gravekeeper with lantern',
        terminalDesc: 'tombstone altar'
    },
    'L13_lake': {
        id: 'L13_lake',
        name: 'Underground Lake',
        description: 'Bioluminescent cavern lake',
        floorDesc: 'wet cave stone near water',
        wallDesc: 'glowing fungus wall',
        decoration1: 'bioluminescent plant',
        decoration2: 'underground lily pad',
        colorPalette: 'deep blues, bioluminescent cyan/purple',
        npcDesc: 'water spirit with glowing form',
        terminalDesc: 'lily pad shrine'
    },
    'L14_forge': {
        id: 'L14_forge',
        name: 'Lava Forge',
        description: 'Volcanic smithing area',
        floorDesc: 'metal grate over lava',
        wallDesc: 'volcanic rock',
        decoration1: 'anvil',
        decoration2: 'lava crack',
        colorPalette: 'molten orange, metal grey, fire red',
        npcDesc: 'blacksmith with hammer',
        terminalDesc: 'anvil workstation'
    },
    'L15_lair': {
        id: 'L15_lair',
        name: "Dragon's Lair",
        description: 'Treasure-filled dragon den',
        floorDesc: 'gold coins floor',
        wallDesc: 'scorched cave wall',
        decoration1: 'treasure chest',
        decoration2: 'dragon scale',
        colorPalette: 'gold, ruby red, burnt black',
        npcDesc: 'treasure hunter with map',
        terminalDesc: 'golden treasure pile'
    },

    // === Chapter 6: Castle Region ===
    'L16_courtyard': {
        id: 'L16_courtyard',
        name: 'Castle Courtyard',
        description: 'Royal garden with fountains',
        floorDesc: 'ornate stone tiles',
        wallDesc: 'hedge wall',
        decoration1: 'fountain',
        decoration2: 'rose bush',
        colorPalette: 'royal purple, garden green, marble white',
        npcDesc: 'royal gardener with shears',
        terminalDesc: 'garden fountain pedestal'
    },
    'L17_throne': {
        id: 'L17_throne',
        name: 'Throne Room',
        description: 'Grand royal chamber',
        floorDesc: 'red carpet on marble',
        wallDesc: 'ornate pillar',
        decoration1: 'royal banner',
        decoration2: 'chandelier base',
        colorPalette: 'royal red, gold trim, marble white',
        npcDesc: 'royal herald with scroll',
        terminalDesc: 'golden throne pedestal'
    },
    'L18_treasury': {
        id: 'L18_treasury',
        name: 'Royal Treasury',
        description: 'Vault filled with riches',
        floorDesc: 'reinforced vault floor',
        wallDesc: 'vault wall with safe',
        decoration1: 'gold pile',
        decoration2: 'gem display',
        colorPalette: 'gold, silver, jewel tones',
        npcDesc: 'royal treasurer with keys',
        terminalDesc: 'locked treasure chest'
    },
    'L19_dungeon': {
        id: 'L19_dungeon',
        name: 'Dungeon Cells',
        description: 'Dark prison area',
        floorDesc: 'dirty stone with straw',
        wallDesc: 'iron bar cell',
        decoration1: 'chain shackles',
        decoration2: 'wooden bucket',
        colorPalette: 'dark grey, rusty iron, dim torch light',
        npcDesc: 'dungeon guard with torch',
        terminalDesc: 'iron cage display'
    },
    'L20_passage': {
        id: 'L20_passage',
        name: 'Secret Passage',
        description: 'Hidden corridor with traps',
        floorDesc: 'pressure plate floor',
        wallDesc: 'hidden door wall',
        decoration1: 'torch holder',
        decoration2: 'cobweb corner',
        colorPalette: 'shadow black, stone grey, warning red',
        npcDesc: 'hooded spy with dagger',
        terminalDesc: 'hidden lever mechanism'
    },

    // === Chapter 7: Tower Ascent ===
    'L21_stairs': {
        id: 'L21_stairs',
        name: 'Tower Stairs',
        description: 'Spiral staircase climb',
        floorDesc: 'curved stone step',
        wallDesc: 'curved tower wall',
        decoration1: 'wall torch',
        decoration2: 'arrow slit window',
        colorPalette: 'cold stone, torch warm, night blue',
        npcDesc: 'tower watchman with horn',
        terminalDesc: 'spiral stone marker'
    },
    'L22_alchemy': {
        id: 'L22_alchemy',
        name: 'Alchemy Lab',
        description: 'Magical experiment room',
        floorDesc: 'stained wooden floor',
        wallDesc: 'shelf with potions',
        decoration1: 'bubbling cauldron',
        decoration2: 'potion bottle',
        colorPalette: 'magical purple, potion green, wood brown',
        npcDesc: 'alchemist with bubbling flask',
        terminalDesc: 'bubbling cauldron station'
    },
    'L23_observatory': {
        id: 'L23_observatory',
        name: 'Astral Observatory',
        description: 'Star-gazing dome',
        floorDesc: 'star map floor tiles',
        wallDesc: 'telescope mount',
        decoration1: 'astrolabe',
        decoration2: 'floating orb',
        colorPalette: 'night purple, star gold, cosmic blue',
        npcDesc: 'stargazer with telescope',
        terminalDesc: 'astral orrery display'
    },
    'L24_portal': {
        id: 'L24_portal',
        name: 'Portal Chamber',
        description: 'Dimensional gateway room',
        floorDesc: 'rune circle floor',
        wallDesc: 'arcane pillar',
        decoration1: 'portal rift',
        decoration2: 'floating rock',
        colorPalette: 'void purple, energy cyan, rune gold',
        npcDesc: 'portal guardian in arcane armor',
        terminalDesc: 'active portal frame'
    },
    'L25_sanctum': {
        id: 'L25_sanctum',
        name: 'Final Sanctum',
        description: 'Epic boss arena',
        floorDesc: 'grand mosaic floor',
        wallDesc: 'towering ancient pillar',
        decoration1: 'power crystal',
        decoration2: 'magic circle',
        colorPalette: 'epic gold, power blue, divine white',
        npcDesc: 'grand archmage with staff',
        terminalDesc: 'nexus power crystal'
    }
};

/**
 * Generate AI prompt for a theme's tileset
 */
export function getThemePrompt(themeId: string): string {
    const theme = LEVEL_THEMES[themeId];
    if (!theme) return '';

    return `Create a 32x32 pixel art game tileset for "${theme.name}" theme. 16-bit SNES style, dark fantasy RPG aesthetic.

Generate these 6 separate 32x32 tiles as individual images:

1. FLOOR TILE: ${theme.floorDesc} - main walkable surface, tileable
2. FLOOR ALT: variation of ${theme.floorDesc} - for visual variety
3. WALL TILE: ${theme.wallDesc} - solid blocking boundary
4. WALL TOP: top edge of ${theme.wallDesc} - shows wall depth
5. DECORATION 1: ${theme.decoration1} - small prop object
6. DECORATION 2: ${theme.decoration2} - another prop object

Color palette: ${theme.colorPalette}
Style: Clean pixel art, consistent lighting from top-left, game asset quality
NO text, NO labels, just pure pixel art tiles`;
}

/**
 * Get theme by level ID (e.g., "L01" -> "L01_village")
 */
export function getThemeForLevel(levelId: string): ThemeConfig | null {
    const themeKey = Object.keys(LEVEL_THEMES).find(k => k.startsWith(levelId));
    return themeKey ? LEVEL_THEMES[themeKey] : null;
}

// Theme groups for parallel generation
export const THEME_GROUPS = {
    group1: ['L01_village', 'L02_market', 'L03_tower', 'L04_forest', 'L05_darkwoods', 'L06_river', 'L07_cavern', 'L08_mountain'],
    group2: ['L09_ice', 'L10_temple', 'L11_library', 'L12_crypt', 'L13_lake', 'L14_forge', 'L15_lair', 'L16_courtyard'],
    group3: ['L17_throne', 'L18_treasury', 'L19_dungeon', 'L20_passage', 'L21_stairs', 'L22_alchemy', 'L23_observatory', 'L24_portal', 'L25_sanctum']
};
