/** Sprite sheet metadata for frame-based animation */
export interface SpriteSheetConfig {
    path: string;
    frameWidth: number;
    frameHeight: number;
    frameCount: number;
    columns?: number; // If not specified, assumes horizontal strip
    animations?: Record<string, { start: number; end: number; loop?: boolean }>;
}

/** Tileset configuration for loading multiple tiles from one image */
export interface TilesetConfig {
    path: string;
    tileWidth: number;
    tileHeight: number;
    columns: number;
    rows: number;
    tiles: Record<string, number>; // tile name -> index in tileset
}

/** Simple sprite (single image) */
export type SpriteConfig = string | SpriteSheetConfig;

/** Tile config can be simple path or part of tileset */
export type TileConfig = string | { tileset: string; index: number };

/** File paths for sprites, tiles, and audio keyed by logical name. */
export interface AssetManifest {
    sprites: Record<string, SpriteConfig>;
    tiles: Record<string, TileConfig>;
    tilesets?: Record<string, TilesetConfig>;
    audio: Record<string, string>;
}

/** Loaded sprite with optional sheet metadata */
export interface LoadedSprite {
    image: HTMLImageElement;
    config?: SpriteSheetConfig;
}

/** Loaded DOM assets keyed by name. */
export interface LoadedAssets {
    sprites: Map<string, HTMLImageElement>;
    spriteConfigs: Map<string, SpriteSheetConfig>; // metadata for sprite sheets
    tiles: Map<string, HTMLImageElement>;
    tilesets: Map<string, { image: HTMLImageElement; config: TilesetConfig }>;
    audio: Map<string, HTMLAudioElement>;
}

/** Default asset manifest (paths served from static dir). */
export const DEFAULT_MANIFEST: AssetManifest = {
    sprites: {
        // Player walk cycles (4 frames each: neutral, step-left, neutral, step-right)
        player_down: {
            path: '/sprites/player_down_walk.png',
            frameWidth: 32,
            frameHeight: 32,
            frameCount: 4,
            animations: {
                walk: { start: 0, end: 3, loop: true },
                idle: { start: 0, end: 0, loop: true },
            },
        },
        player_up: {
            path: '/sprites/player_up_walk.png',
            frameWidth: 32,
            frameHeight: 32,
            frameCount: 4,
            animations: {
                walk: { start: 0, end: 3, loop: true },
                idle: { start: 0, end: 0, loop: true },
            },
        },
        player_left: {
            path: '/sprites/player_left_walk.png',
            frameWidth: 32,
            frameHeight: 32,
            frameCount: 4,
            animations: {
                walk: { start: 0, end: 3, loop: true },
                idle: { start: 0, end: 0, loop: true },
            },
        },
        player_right: {
            path: '/sprites/player_right_walk.png',
            frameWidth: 32,
            frameHeight: 32,
            frameCount: 4,
            animations: {
                walk: { start: 0, end: 3, loop: true },
                idle: { start: 0, end: 0, loop: true },
            },
        },
        npc_mentor: '/sprites/npc_mentor.png',
    },
    tiles: {
        floor: '/tiles/terrain/stone.png',
        floor_tech: '/tiles/floor_tech.png',
        wall: '/tiles/wall.png',
        wall_top: '/tiles/wall_top.png',
        terminal: '/tiles/interactive/terminal_animated.png', // 4-frame glow pulse
        door_locked: '/tiles/interactive/door_locked.png',
        door_open: '/tiles/interactive/door_open.png',
        void: '/tiles/interactive/void.png',
        water: '/tiles/terrain/water_animated.png', // 4-frame sprite sheet
        // Additional terrain tiles
        grass: '/tiles/terrain/grass.png',
        path: '/tiles/terrain/path.png',
        dirt: '/tiles/terrain/dirt.png',
        marsh: '/tiles/terrain/marsh.png',
    },
    audio: {},
};

/** Load all assets defined in a manifest. */
export async function loadAssets(manifest: AssetManifest): Promise<LoadedAssets> {
    const sprites = new Map<string, HTMLImageElement>();
    const spriteConfigs = new Map<string, SpriteSheetConfig>();
    const tiles = new Map<string, HTMLImageElement>();
    const tilesets = new Map<string, { image: HTMLImageElement; config: TilesetConfig }>();
    const audio = new Map<string, HTMLAudioElement>();

    // Load tilesets first (tiles may reference them)
    if (manifest.tilesets) {
        await Promise.all(
            Object.entries(manifest.tilesets).map(async ([name, config]) => {
                const img = await loadImage(config.path).catch(() => null);
                if (img) tilesets.set(name, { image: img, config });
            })
        );
    }

    await Promise.all([
        // Load sprites (handle both simple paths and sprite sheet configs)
        ...Object.entries(manifest.sprites).map(async ([name, config]) => {
            if (typeof config === 'string') {
                const img = await loadImage(config).catch(() => null);
                if (img) sprites.set(name, img);
            } else {
                // Sprite sheet config
                const img = await loadImage(config.path).catch(() => null);
                if (img) {
                    sprites.set(name, img);
                    spriteConfigs.set(name, config);
                }
            }
        }),
        // Load tiles (handle both simple paths and tileset references)
        ...Object.entries(manifest.tiles).map(async ([name, config]) => {
            if (typeof config === 'string') {
                const img = await loadImage(config).catch(() => null);
                if (img) tiles.set(name, img);
            } else {
                // Tileset reference - extract tile from loaded tileset
                const tileset = tilesets.get(config.tileset);
                if (tileset) {
                    const tileImg = extractTileFromTileset(tileset.image, tileset.config, config.index);
                    if (tileImg) tiles.set(name, tileImg);
                }
            }
        }),
        // Load audio
        ...Object.entries(manifest.audio).map(async ([name, path]) => {
            const audioEl = await loadAudio(path).catch(() => null);
            if (audioEl) audio.set(name, audioEl);
        }),
    ]);

    return { sprites, spriteConfigs, tiles, tilesets, audio };
}

/** Extract a single tile from a tileset image */
function extractTileFromTileset(
    tilesetImage: HTMLImageElement,
    config: TilesetConfig,
    index: number
): HTMLImageElement | null {
    const col = index % config.columns;
    const row = Math.floor(index / config.columns);

    if (row >= config.rows) return null;

    const canvas = document.createElement('canvas');
    canvas.width = config.tileWidth;
    canvas.height = config.tileHeight;
    const ctx = canvas.getContext('2d');
    if (!ctx) return null;

    ctx.drawImage(
        tilesetImage,
        col * config.tileWidth,
        row * config.tileHeight,
        config.tileWidth,
        config.tileHeight,
        0,
        0,
        config.tileWidth,
        config.tileHeight
    );

    const img = new Image();
    img.src = canvas.toDataURL();
    return img;
}

/** Get frame coordinates from a sprite sheet */
export function getSpriteFrame(
    config: SpriteSheetConfig,
    frameIndex: number
): { x: number; y: number; width: number; height: number } {
    const columns = config.columns ?? config.frameCount; // Default to horizontal strip
    const col = frameIndex % columns;
    const row = Math.floor(frameIndex / columns);
    return {
        x: col * config.frameWidth,
        y: row * config.frameHeight,
        width: config.frameWidth,
        height: config.frameHeight,
    };
}

/** Load a single image element. */
export function loadImage(src: string): Promise<HTMLImageElement> {
    return new Promise((resolve, reject) => {
        const img = new Image();
        img.onload = () => resolve(img);
        img.onerror = () => reject(new Error(`Failed to load image: ${src}`));
        img.src = src;
    });
}

/** Load a single audio element (non-blocking if unavailable). */
function loadAudio(src: string): Promise<HTMLAudioElement> {
    return new Promise((resolve, reject) => {
        const audio = new Audio();
        const onLoad = () => resolve(audio);
        const onError = () => reject(new Error(`Failed to load audio: ${src}`));
        audio.addEventListener('canplaythrough', onLoad, { once: true });
        audio.addEventListener('error', onError, { once: true });
        audio.src = src;
        audio.load();
    });
}

/**
 * Preload images for a level.
 * @deprecated Use preloadLevelAssets from assets-cache.ts instead for cached loading.
 */
export async function preloadLevel(_levelId: string): Promise<void> {
    // Import dynamically to avoid circular dependency
    const { getGlobalAssets } = await import('./assets-cache');
    await getGlobalAssets().catch(() => undefined);
}
