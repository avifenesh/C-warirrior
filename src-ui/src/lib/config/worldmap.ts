// World Map Configuration
// Level positions as percentages of map container (x, y)
// Positions mapped to actual map features

export interface LevelPosition {
    x: number;
    y: number;
}

export const LEVEL_POSITIONS: Record<string, LevelPosition> = {
    'L01': { x: 22, y: 81 },
    'L02': { x: 26, y: 78 },
    'L03': { x: 34, y: 72 },
    'L04': { x: 39, y: 65 },
    'L05': { x: 47, y: 70 },
    'L06': { x: 52, y: 74 },
    'L07': { x: 55, y: 83 },
    'L08': { x: 59, y: 87 },
    'L09': { x: 63, y: 87 },
    'L10': { x: 67.5, y: 75 },
    'L11': { x: 68, y: 64 },
    'L12': { x: 74, y: 54 },
    'L13': { x: 74, y: 46 },
    'L14': { x: 76, y: 32 },
    'L15': { x: 74, y: 19 },
    'L16': { x: 65, y: 62 },
    'L17': { x: 59, y: 59 },
    'L18': { x: 54, y: 52 },
    'L19': { x: 59, y: 42 },
    'L20': { x: 47, y: 54 },
    'L21': { x: 46, y: 47 },
    'L22': { x: 44, y: 41 },
    'L23': { x: 51, y: 30 },
    'L24': { x: 36, y: 29 },
    'L25': { x: 33, y: 19 },
};

export const MAP_CONFIG = {
    title: 'THE REALM OF C',
    subtitle: 'Choose Your Quest',
    // AI-generated 16-bit pixel art map (Chrono Trigger style)
    backgroundImage: '/ui/world_map.png',
    // Set to false to use the AI-generated image
    useFallbackBackground: false,
};
