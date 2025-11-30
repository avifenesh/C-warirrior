// World Map Configuration
// Level positions as percentages of map container (x, y)
// Positions mapped to actual map features

export interface LevelPosition {
    x: number;
    y: number;
}

export const LEVEL_POSITIONS: Record<string, LevelPosition> = {
    'L01': { x: 23, y: 77 },
    'L02': { x: 28, y: 74 },
    'L03': { x: 33, y: 70 },
    'L04': { x: 39, y: 64 },
    'L05': { x: 48, y: 67 },
    'L06': { x: 52, y: 72 },
    'L07': { x: 55, y: 80 },
    'L08': { x: 59, y: 83 },
    'L09': { x: 64, y: 84 },
    'L10': { x: 67, y: 77 },
    'L11': { x: 67, y: 67 },
    'L12': { x: 74, y: 54 },
    'L13': { x: 74, y: 42 },
    'L14': { x: 77, y: 31 },
    'L15': { x: 73, y: 26 },
    'L16': { x: 64, y: 60 },
    'L17': { x: 59, y: 58 },
    'L18': { x: 54, y: 52 },
    'L19': { x: 60, y: 43 },
    'L20': { x: 46, y: 55 },
    'L21': { x: 46, y: 46 },
    'L22': { x: 44, y: 40 },
    'L23': { x: 52, y: 30 },
    'L24': { x: 37, y: 33 },
    'L25': { x: 32, y: 24 },
};

export const MAP_CONFIG = {
    title: 'THE REALM OF C',
    subtitle: 'Choose Your Quest',
    // AI-generated 16-bit pixel art map (Chrono Trigger style)
    backgroundImage: '/ui/world_map.png',
    // Set to false to use the AI-generated image
    useFallbackBackground: false,
};
