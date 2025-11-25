/** File paths for sprites, tiles, and audio keyed by logical name. */
export interface AssetManifest {
    sprites: Record<string, string>;
    tiles: Record<string, string>;
    audio: Record<string, string>;
}

/** Loaded DOM assets keyed by name. */
export interface LoadedAssets {
    sprites: Map<string, HTMLImageElement>;
    tiles: Map<string, HTMLImageElement>;
    audio: Map<string, HTMLAudioElement>;
}

/** Default asset manifest (paths served from static dir). */
export const DEFAULT_MANIFEST: AssetManifest = {
    sprites: {
        player_down: '/sprites/player_down.png',
        player_up: '/sprites/player_up.png',
        player_left: '/sprites/player_left.png',
        player_right: '/sprites/player_right.png',
        npc_mentor: '/sprites/npc_mentor.png',
    },
    tiles: {
        floor: '/tiles/floor_stone.png',
        floor_tech: '/tiles/floor_tech.png',
        wall: '/tiles/wall.png',
        wall_top: '/tiles/wall_top.png',
        terminal: '/tiles/terminal.png',
        door_locked: '/tiles/door_locked.png',
        door_open: '/tiles/door_open.png',
        void: '/tiles/void.png',
        water: '/tiles/floor_tech.png', // Reuse tech floor for water
    },
    audio: {},
};

/** Load all assets defined in a manifest. */
export async function loadAssets(manifest: AssetManifest): Promise<LoadedAssets> {
    const sprites = new Map<string, HTMLImageElement>();
    const tiles = new Map<string, HTMLImageElement>();
    const audio = new Map<string, HTMLAudioElement>();

    await Promise.all([
        ...Object.entries(manifest.sprites).map(async ([name, path]) => {
            const img = await loadImage(path).catch(() => null);
            if (img) sprites.set(name, img);
        }),
        ...Object.entries(manifest.tiles).map(async ([name, path]) => {
            const img = await loadImage(path).catch(() => null);
            if (img) tiles.set(name, img);
        }),
        ...Object.entries(manifest.audio).map(async ([name, path]) => {
            const audioEl = await loadAudio(path).catch(() => null);
            if (audioEl) audio.set(name, audioEl);
        }),
    ]);

    return { sprites, tiles, audio };
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

/** Preload images for a level (placeholder uses default manifest). */
export async function preloadLevel(levelId: string): Promise<void> {
    // For now, reuse the default manifest. Extend when per-level assets are defined.
    await loadAssets(DEFAULT_MANIFEST).catch(() => undefined);
    console.debug(`[assets] preload complete for level ${levelId}`);
}
