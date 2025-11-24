/** Tile size in pixels. */
export const TILE_SIZE = 32;

/** Canvas width in pixels. */
export const CANVAS_WIDTH = 800;

/** Canvas height in pixels. */
export const CANVAS_HEIGHT = 600;

/** Horizontal viewport size in tiles. */
export const VIEWPORT_TILES_X = Math.ceil(CANVAS_WIDTH / TILE_SIZE);

/** Vertical viewport size in tiles. */
export const VIEWPORT_TILES_Y = Math.ceil(CANVAS_HEIGHT / TILE_SIZE);

/** Player speed in tiles per second (render-only hint). */
export const PLAYER_SPEED = 4;

/** Target frames per second for animation helpers. */
export const ANIMATION_FPS = 60;

/** Shared color palette for overlays and UI accents. */
export const COLORS = {
    background: '#0f172a',
    terminal: '#22d3ee',
    health: '#ef4444',
    xp: '#a855f7',
    success: '#10b981',
    error: '#f43f5e',
} as const;

/** Z-index map used by layered canvas/UI rendering. */
export const Z_INDEX = {
    background: 0,
    floor: 1,
    objects: 2,
    player: 3,
    effects: 4,
    ui: 10,
    modal: 20,
} as const;
