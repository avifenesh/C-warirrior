// World Map Configuration
// Level positions as percentages of map container (x, y)
// Positions mapped to actual map features

export interface LevelPosition {
    x: number;
    y: number;
}

export const LEVEL_POSITIONS: Record<string, LevelPosition> = {
    // === RPG PROGRESSION: Village -> Plains -> Forest -> Hills -> Mountains ===

    // Zone 1: The Village & Outskirts (The Beginning)
    'L01': { x: 18, y: 78 },  // The Village Center - Start
    'L02': { x: 26, y: 72 },  // Village Exit Road
    'L03': { x: 36, y: 68 },  // The First Grassy Plains
    'L04': { x: 42, y: 76 },  // Southern Forest Edge
    'L05': { x: 50, y: 82 },  // Southern Coastline Path

    // Zone 2: The Riverlands & Eastern Isle (Mid-Game)
    'L06': { x: 62, y: 75 },  // Approach to the River
    'L07': { x: 68, y: 66 },  // The Lower Bridge
    'L08': { x: 78, y: 62 },  // Eastern Forest South
    'L09': { x: 85, y: 52 },  // Eastern Forest North
    'L10': { x: 73, y: 48 },  // The Upper Bridge - Crossing back

    // Zone 3: Central Highlands & The Deep Woods
    'L11': { x: 58, y: 52 },  // The Central Crossroads
    'L12': { x: 48, y: 55 },  // Central Hills
    'L13': { x: 38, y: 48 },  // Path to the West
    'L14': { x: 28, y: 38 },  // The Blue Lake Shore
    'L15': { x: 18, y: 32 },  // The Deep Dark Forest - NW Corner

    // Zone 4: The Foothills (Ascent)
    'L16': { x: 32, y: 28 },  // Exiting the Deep Forest
    'L17': { x: 42, y: 32 },  // Northern Plains
    'L18': { x: 48, y: 25 },  // Base of the Mountains
    'L19': { x: 55, y: 35 },  // The Valley Path
    'L20': { x: 65, y: 32 },  // Mountain Cave Entrance

    // Zone 5: The Snowy Peaks (End Game)
    'L21': { x: 52, y: 18 },  // First Snowy Peak
    'L22': { x: 60, y: 22 },  // The Ridge
    'L23': { x: 68, y: 15 },  // High Altitude Pass
    'L24': { x: 75, y: 18 },  // The Guardian's Lair
    'L25': { x: 62, y: 10 },  // The Summit - Final Boss
};

export const MAP_CONFIG = {
    title: 'THE REALM OF C',
    subtitle: 'Choose Your Quest',
    // AI-generated 16-bit pixel art map (Chrono Trigger style)
    backgroundImage: '/ui/world_map.png',
    // Set to false to use the AI-generated image
    useFallbackBackground: false,
};
