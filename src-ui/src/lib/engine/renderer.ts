/** Sprite rendering helpers. */
export interface Sprite {
    image: HTMLImageElement;
    width: number;
    height: number;
    frames: number;
    currentFrame: number;
}

/** Context passed to render helpers. */
export interface RenderContext {
    ctx: CanvasRenderingContext2D;
    tileSize: number;
    camera: { x: number; y: number };
}

/** Draw a sprite at world position. */
export function drawSprite(
    context: RenderContext,
    sprite: Sprite,
    worldX: number,
    worldY: number
): void {
    const { ctx, tileSize, camera } = context;
    if (!sprite.image || sprite.image.complete === false) return;

    const { x, y } = worldToScreen(worldX, worldY, camera, tileSize);
    const frameWidth = sprite.width;
    const frameHeight = sprite.height;
    ctx.drawImage(
        sprite.image,
        sprite.currentFrame * frameWidth,
        0,
        frameWidth,
        frameHeight,
        x,
        y,
        frameWidth,
        frameHeight
    );
}

/** Draw a tile from a tileset atlas using 1D index (row-major). */
export function drawTile(
    context: RenderContext,
    tileset: HTMLImageElement,
    tileIndex: number,
    worldX: number,
    worldY: number
): void {
    const { ctx, tileSize, camera } = context;
    if (!tileset || tileset.complete === false) return;
    if (tileIndex < 0) return;

    const tilesPerRow = Math.floor(tileset.width / tileSize) || 1;
    const sx = (tileIndex % tilesPerRow) * tileSize;
    const sy = Math.floor(tileIndex / tilesPerRow) * tileSize;

    const { x, y } = worldToScreen(worldX, worldY, camera, tileSize);
    ctx.drawImage(tileset, sx, sy, tileSize, tileSize, x, y, tileSize, tileSize);
}

/** Draw animated sprite (frame selection based on elapsed frameTime in ms). */
export function drawAnimatedSprite(
    context: RenderContext,
    sprite: Sprite,
    worldX: number,
    worldY: number,
    frameTime: number
): void {
    if (sprite.frames <= 0) return drawSprite(context, sprite, worldX, worldY);
    const frameWidth = sprite.width;
    const { ctx } = context;
    if (!sprite.image || sprite.image.complete === false) return;

    const defaultFrameDuration = 150; // ms per frame
    const frameIndex = Math.floor(frameTime / defaultFrameDuration) % sprite.frames;
    sprite.currentFrame = frameIndex;
    const { x, y } = worldToScreen(worldX, worldY, context.camera, context.tileSize);
    ctx.drawImage(
        sprite.image,
        frameIndex * frameWidth,
        0,
        frameWidth,
        sprite.height,
        x,
        y,
        frameWidth,
        sprite.height
    );
}

/** Convert world coordinates (in pixels) to screen coordinates. */
export function worldToScreen(
    worldX: number,
    worldY: number,
    camera: { x: number; y: number },
    tileSize: number
): { x: number; y: number } {
    return {
        x: Math.round(worldX - camera.x * tileSize),
        y: Math.round(worldY - camera.y * tileSize),
    };
}

/** Convert screen coordinates to world coordinates. */
export function screenToWorld(
    screenX: number,
    screenY: number,
    camera: { x: number; y: number },
    tileSize: number
): { x: number; y: number } {
    return {
        x: screenX + camera.x * tileSize,
        y: screenY + camera.y * tileSize,
    };
}
