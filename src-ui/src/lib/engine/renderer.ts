import type { RenderState, Tile } from '$lib/types';
import type { LoadedAssets, SpriteSheetConfig } from './assets';
import type { ParticleSystem } from './particles';
import { getCurrentFrame, type AnimationState } from './animation';

export interface RenderConfig {
    tileSize: number;
    backendTileSize: number;
    debug?: boolean;
}

export class GameRenderer {
    private canvas: HTMLCanvasElement;
    private ctx: CanvasRenderingContext2D;
    private assets: LoadedAssets | null = null;
    private config: RenderConfig;
    private animationTime: number = 0;

    // Tile layer caching for performance
    private tileCacheCanvas: HTMLCanvasElement | null = null;
    private tileCacheCtx: CanvasRenderingContext2D | null = null;
    private tileCacheDirty: boolean = true;
    private lastCacheWidth: number = 0;
    private lastCacheHeight: number = 0;
    private lastViewportX: number = -1;
    private lastViewportY: number = -1;
    private lastLevelId: string | null = null;

    constructor(canvas: HTMLCanvasElement, config: Partial<RenderConfig> = {}) {
        this.canvas = canvas;
        const ctx = canvas.getContext('2d', { alpha: false });
        if (!ctx) throw new Error('Could not get 2D context');
        this.ctx = ctx;
        this.ctx.imageSmoothingEnabled = false;

        this.config = {
            tileSize: 64,
            backendTileSize: 32,
            debug: false,
            ...config
        };
    }

    public setAssets(assets: LoadedAssets) {
        this.assets = assets;
    }

    public resize(width: number, height: number) {
        if (this.canvas.width !== width || this.canvas.height !== height) {
            this.canvas.width = width;
            this.canvas.height = height;
            this.ctx.imageSmoothingEnabled = false; // Reset on resize
        }
    }

    /**
     * Render the game scene.
     * @param state - Current render state from backend
     * @param particles - Particle system for effects
     * @param dt - Delta time since last frame (ms)
     * @param currentTime - Current timestamp for animations
     * @param animState - Animation states for player/terminal
     * @param stateChanged - Hint: true if game state changed since last render (for cache invalidation)
     */
    public render(
        state: RenderState | null,
        particles: ParticleSystem | null,
        dt: number,
        currentTime: number,
        animState?: { player: AnimationState | null; terminal: AnimationState | null },
        stateChanged: boolean = true
    ) {
        this.animationTime = currentTime;

        // Clear screen
        this.ctx.fillStyle = '#0a0a14';
        this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);

        if (!state) {
            this.renderLoading();
            return;
        }

        if (!this.assets) {
            this.renderFallback(state);
            return;
        }

        this.renderScene(state, animState, stateChanged);

        if (particles) {
            particles.render(this.ctx, state.viewport_offset, this.config.tileSize);
        }
    }

    private renderLoading() {
        const { width, height } = this.canvas;
        this.ctx.fillStyle = '#fbbf24';
        this.ctx.font = '12px "Press Start 2P", "Courier New", monospace';
        this.ctx.textAlign = 'center';
        this.ctx.fillText(this.assets ? 'Entering world...' : 'Loading assets...', width / 2, height / 2);
    }

    private renderFallback(state: RenderState) {
        if (!state.visible_tiles || !state.player) return;

        const { tileSize, backendTileSize } = this.config;
        const viewport = state.viewport_offset;

        // Draw tiles
        for (let y = 0; y < state.visible_tiles.length; y++) {
            for (let x = 0; x < state.visible_tiles[y].length; x++) {
                const tile = state.visible_tiles[y][x];
                const screenX = x * tileSize;
                const screenY = y * tileSize;

                // Grid style fallback
                this.ctx.fillStyle = this.getFallbackColor(tile.tile_type);
                this.ctx.fillRect(screenX, screenY, tileSize, tileSize);
                this.ctx.strokeStyle = 'rgba(255,255,255,0.1)';
                this.ctx.strokeRect(screenX, screenY, tileSize, tileSize);

                // Special highlight for interactables
                if (tile.tile_type === 'terminal') {
                    const playerTileX = Math.floor(state.player.position.x / backendTileSize) - viewport.x;
                    const playerTileY = Math.floor(state.player.position.y / backendTileSize) - viewport.y;
                    const dist = Math.abs(playerTileX - x) + Math.abs(playerTileY - y);
                    
                    if (dist <= 1) {
                        this.ctx.strokeStyle = '#fbbf24';
                        this.ctx.lineWidth = 2;
                        this.ctx.strokeRect(screenX + 2, screenY + 2, tileSize - 4, tileSize - 4);
                    }
                }
            }
        }

        // Draw player
        const scale = tileSize / backendTileSize;
        const px = (state.player.position.x * scale) - (viewport.x * tileSize);
        const py = (state.player.position.y * scale) - (viewport.y * tileSize);

        this.ctx.fillStyle = '#708090';
        this.ctx.fillRect(px - tileSize * 0.35, py - tileSize * 0.4, tileSize * 0.7, tileSize * 0.8);
        this.ctx.fillStyle = '#fbbf24'; // gold trim
        this.ctx.fillRect(px - tileSize * 0.2, py - tileSize * 0.35, tileSize * 0.4, tileSize * 0.1);
    }

    private renderScene(
        state: RenderState,
        animState?: { player: AnimationState | null; terminal: AnimationState | null },
        stateChanged: boolean = true
    ) {
        if (!state.visible_tiles || !state.player || !this.assets) return;

        const { tileSize, backendTileSize } = this.config;
        const viewport = state.viewport_offset;
        const playerBackendX = Math.floor(state.player.position.x / backendTileSize);
        const playerBackendY = Math.floor(state.player.position.y / backendTileSize);

        // Check if tile cache needs rebuild
        const gridWidth = state.visible_tiles[0]?.length ?? 0;
        const gridHeight = state.visible_tiles.length;
        const cachePixelWidth = gridWidth * tileSize;
        const cachePixelHeight = gridHeight * tileSize;

        // Invalidate cache when: dimensions change, viewport moves, level changes, or state changed
        if (
            stateChanged ||
            cachePixelWidth !== this.lastCacheWidth ||
            cachePixelHeight !== this.lastCacheHeight ||
            viewport.x !== this.lastViewportX ||
            viewport.y !== this.lastViewportY ||
            state.current_level_id !== this.lastLevelId
        ) {
            this.tileCacheDirty = true;
        }

        // 1. Draw static tiles (cached when possible)
        if (this.tileCacheDirty) {
            this.rebuildTileCache(state, cachePixelWidth, cachePixelHeight);
            this.lastCacheWidth = cachePixelWidth;
            this.lastCacheHeight = cachePixelHeight;
            this.lastViewportX = viewport.x;
            this.lastViewportY = viewport.y;
            this.lastLevelId = state.current_level_id;
            this.tileCacheDirty = false;
        }

        // Draw cached tiles
        if (this.tileCacheCanvas) {
            this.ctx.drawImage(this.tileCacheCanvas, 0, 0);
        }

        // 2. Draw animated tiles (water, terminal) - these need per-frame updates
        this.renderAnimatedTiles(state);

        // 3. Draw dynamic overlays (terminal highlights, door indicators)
        this.renderDynamicOverlays(state, playerBackendX, playerBackendY, viewport);

        // 2. Draw Player
        const scale = tileSize / backendTileSize;
        const px = (state.player.position.x * scale) - (viewport.x * tileSize);
        const py = (state.player.position.y * scale) - (viewport.y * tileSize);

        const playerSpriteName = `player_${state.player.facing}`;
        const playerSprite = this.assets.sprites.get(playerSpriteName);

        if (playerSprite) {
            // Determine animation frame
            let frameIndex = 0;
            if (animState?.player) {
                frameIndex = getCurrentFrame(animState.player, this.animationTime);
            }

            const halfSize = tileSize / 2;
            let drawX = px - halfSize;
            let drawY = py - halfSize;

            // Sprite Sheet Logic
            const isSpriteSheet = playerSprite.width > playerSprite.height || playerSprite.height > playerSprite.width;
            
            if (isSpriteSheet) {
                // Assume horizontal strip
                const frameWidth = playerSprite.height; // Assume square frames
                const frameX = frameIndex * frameWidth;
                
                this.ctx.drawImage(
                    playerSprite,
                    frameX, 0, frameWidth, playerSprite.height,
                    drawX, drawY, tileSize, tileSize
                );
            } else {
                // Single frame + Bobbing
                if (animState?.player?.animation?.id?.includes('walk')) {
                    const bob = Math.sin(this.animationTime / 50) * 2;
                    drawY += bob;
                }
                this.ctx.drawImage(playerSprite, drawX, drawY, tileSize, tileSize);
            }
        } else {
            // Fallback Player
            this.ctx.fillStyle = '#708090';
            this.ctx.fillRect(px - tileSize * 0.35, py - tileSize * 0.4, tileSize * 0.7, tileSize * 0.8);
            this.ctx.fillStyle = '#fbbf24';
            this.ctx.fillRect(px - tileSize * 0.2, py - tileSize * 0.35, tileSize * 0.4, tileSize * 0.1);
        }
    }

    /**
     * Rebuild the offscreen tile cache canvas with static tiles.
     * Animated tiles (water, terminal) are drawn separately each frame.
     */
    private rebuildTileCache(state: RenderState, width: number, height: number) {
        if (!this.assets || !state.visible_tiles) return;

        const { tileSize } = this.config;

        // Create or resize offscreen canvas
        if (!this.tileCacheCanvas || this.tileCacheCanvas.width !== width || this.tileCacheCanvas.height !== height) {
            this.tileCacheCanvas = document.createElement('canvas');
            this.tileCacheCanvas.width = width;
            this.tileCacheCanvas.height = height;
            this.tileCacheCtx = this.tileCacheCanvas.getContext('2d');
            if (this.tileCacheCtx) {
                this.tileCacheCtx.imageSmoothingEnabled = false;
            }
        }

        if (!this.tileCacheCtx) return;

        // Clear cache
        this.tileCacheCtx.fillStyle = '#0a0a14';
        this.tileCacheCtx.fillRect(0, 0, width, height);

        // Draw static tiles to cache
        for (let y = 0; y < state.visible_tiles.length; y++) {
            for (let x = 0; x < state.visible_tiles[y].length; x++) {
                const tile = state.visible_tiles[y][x];
                const screenX = x * tileSize;
                const screenY = y * tileSize;

                // Draw background color
                this.tileCacheCtx.fillStyle = this.getFallbackColor(tile.tile_type);
                this.tileCacheCtx.fillRect(screenX, screenY, tileSize, tileSize);

                // If terminal, draw grass base first
                if (tile.tile_type === 'terminal') {
                    const grass = this.assets.tiles.get('grass');
                    if (grass) this.tileCacheCtx.drawImage(grass, screenX, screenY, tileSize, tileSize);
                }

                // Skip animated tiles - they're drawn separately
                if (tile.tile_type === 'water' || tile.tile_type === 'terminal') {
                    continue;
                }

                // Draw static tile sprite
                const sprite = this.getTileSprite(tile);
                if (sprite) {
                    this.tileCacheCtx.drawImage(sprite, screenX, screenY, tileSize, tileSize);
                }
            }
        }
    }

    /**
     * Render animated tiles (water, terminal) that need per-frame updates.
     */
    private renderAnimatedTiles(state: RenderState) {
        if (!this.assets || !state.visible_tiles) return;

        const { tileSize } = this.config;

        for (let y = 0; y < state.visible_tiles.length; y++) {
            for (let x = 0; x < state.visible_tiles[y].length; x++) {
                const tile = state.visible_tiles[y][x];

                // Only handle animated tiles
                if (tile.tile_type !== 'water' && tile.tile_type !== 'terminal') {
                    continue;
                }

                const sprite = this.getTileSprite(tile);
                if (!sprite) continue;

                const screenX = x * tileSize;
                const screenY = y * tileSize;

                // Check if sprite sheet (wider than tall = multiple frames)
                const isAnimated = sprite.width > sprite.height;

                if (isAnimated) {
                    const frameCount = 4;
                    const duration = tile.tile_type === 'terminal' ? 300 : 200;
                    const frameIdx = Math.floor(this.animationTime / duration) % frameCount;
                    const frameW = sprite.width / frameCount;

                    this.ctx.drawImage(
                        sprite,
                        frameIdx * frameW, 0, frameW, sprite.height,
                        screenX, screenY, tileSize, tileSize
                    );
                } else {
                    this.ctx.drawImage(sprite, screenX, screenY, tileSize, tileSize);
                }
            }
        }
    }

    /**
     * Render dynamic overlays: terminal highlights, door indicators.
     */
    private renderDynamicOverlays(
        state: RenderState,
        playerBackendX: number,
        playerBackendY: number,
        viewport: { x: number; y: number }
    ) {
        if (!state.visible_tiles) return;

        const { tileSize } = this.config;

        for (let y = 0; y < state.visible_tiles.length; y++) {
            for (let x = 0; x < state.visible_tiles[y].length; x++) {
                const tile = state.visible_tiles[y][x];
                const screenX = x * tileSize;
                const screenY = y * tileSize;

                // Terminal highlight when player is nearby
                if (tile.tile_type === 'terminal') {
                    const dist = Math.abs((playerBackendX - viewport.x) - x) + Math.abs((playerBackendY - viewport.y) - y);
                    if (dist <= 1) {
                        this.ctx.strokeStyle = '#fbbf24';
                        this.ctx.lineWidth = 2;
                        this.ctx.strokeRect(screenX + 1, screenY + 1, tileSize - 2, tileSize - 2);
                    }
                }

                // Locked door indicator
                if (tile.tile_type === 'door' && !tile.walkable) {
                    this.ctx.fillStyle = '#dc2626';
                    this.ctx.fillRect(screenX + tileSize - 8, screenY + 4, 4, 4);
                }
            }
        }
    }

    private getTileSprite(tile: Tile): HTMLImageElement | undefined {
        if (!this.assets) return undefined;

        let key = 'grass'; // default
        switch (tile.tile_type) {
            case 'floor': key = 'grass'; break;
            case 'wall': key = 'wall'; break;
            case 'water': key = 'water'; break;
            case 'void': key = 'void'; break;
            case 'terminal': key = 'terminal'; break;
            case 'door': key = tile.walkable ? 'door_open' : 'door_locked'; break;
        }
        return this.assets.tiles.get(key);
    }

    private getFallbackColor(type: string): string {
        switch (type) {
            case 'wall': return '#4a4a4a';
            case 'water': return '#1e6091';
            case 'void': return '#0a0a14';
            case 'door': return '#8b5a2b';
            case 'terminal': return '#3d7a37';
            default: return '#3d7a37'; // grass
        }
    }
}
