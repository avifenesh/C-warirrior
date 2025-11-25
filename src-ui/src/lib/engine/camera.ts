/** Camera system with smooth following and screen shake. */

import { lerp, easeOutQuad } from './animation';

export interface CameraConfig {
    smoothing: number; // 0-1, higher = smoother but slower
    deadzone: { x: number; y: number }; // pixels before camera starts moving
    bounds?: {
        minX: number;
        minY: number;
        maxX: number;
        maxY: number;
    };
}

export class Camera {
    x: number = 0;
    y: number = 0;

    private targetX: number = 0;
    private targetY: number = 0;
    private config: CameraConfig;

    // Screen shake
    private shakeIntensity: number = 0;
    private shakeDecay: number = 0.9;
    private shakeOffsetX: number = 0;
    private shakeOffsetY: number = 0;

    constructor(config?: Partial<CameraConfig>) {
        this.config = {
            smoothing: 0.1,
            deadzone: { x: 32, y: 32 },
            ...config,
        };
    }

    /** Set target position for camera to follow. */
    setTarget(x: number, y: number): void {
        this.targetX = x;
        this.targetY = y;
    }

    /** Update camera position with smooth following. */
    update(deltaTime: number, viewportWidth: number, viewportHeight: number): void {
        const dt = Math.min(deltaTime / 16.67, 2); // Normalize to 60fps, cap at 2x

        // Calculate offset from current position to target
        const dx = this.targetX - this.x;
        const dy = this.targetY - this.y;

        // Apply deadzone
        let moveX = 0;
        let moveY = 0;

        if (Math.abs(dx) > this.config.deadzone.x) {
            moveX = dx - Math.sign(dx) * this.config.deadzone.x;
        }

        if (Math.abs(dy) > this.config.deadzone.y) {
            moveY = dy - Math.sign(dy) * this.config.deadzone.y;
        }

        // Smooth follow with lerp
        this.x += moveX * this.config.smoothing * dt;
        this.y += moveY * this.config.smoothing * dt;

        // Apply bounds if set
        if (this.config.bounds) {
            const halfViewportX = viewportWidth / 2;
            const halfViewportY = viewportHeight / 2;

            this.x = Math.max(this.config.bounds.minX + halfViewportX, this.x);
            this.x = Math.min(this.config.bounds.maxX - halfViewportX, this.x);
            this.y = Math.max(this.config.bounds.minY + halfViewportY, this.y);
            this.y = Math.min(this.config.bounds.maxY - halfViewportY, this.y);
        }

        // Update screen shake
        if (this.shakeIntensity > 0.1) {
            this.shakeOffsetX = (Math.random() - 0.5) * this.shakeIntensity;
            this.shakeOffsetY = (Math.random() - 0.5) * this.shakeIntensity;
            this.shakeIntensity *= this.shakeDecay;
        } else {
            this.shakeIntensity = 0;
            this.shakeOffsetX = 0;
            this.shakeOffsetY = 0;
        }
    }

    /** Get current camera position including shake offset. */
    getPosition(): { x: number; y: number } {
        return {
            x: this.x + this.shakeOffsetX,
            y: this.y + this.shakeOffsetY,
        };
    }

    /** Trigger screen shake effect. */
    shake(intensity: number = 10, decay: number = 0.9): void {
        this.shakeIntensity = Math.max(this.shakeIntensity, intensity);
        this.shakeDecay = decay;
    }

    /** Instantly move camera to position (no smoothing). */
    snapTo(x: number, y: number): void {
        this.x = x;
        this.y = y;
        this.targetX = x;
        this.targetY = y;
    }

    /** Set camera bounds (in world coordinates). */
    setBounds(minX: number, minY: number, maxX: number, maxY: number): void {
        this.config.bounds = { minX, minY, maxX, maxY };
    }

    /** Clear camera bounds. */
    clearBounds(): void {
        this.config.bounds = undefined;
    }

    /** Get camera as tile coordinates. */
    getTilePosition(tileSize: number): { x: number; y: number } {
        const pos = this.getPosition();
        return {
            x: Math.floor(pos.x / tileSize),
            y: Math.floor(pos.y / tileSize),
        };
    }
}

/** Apply camera transform to canvas context. */
export function applyCameraTransform(
    ctx: CanvasRenderingContext2D,
    camera: Camera,
    canvasWidth: number,
    canvasHeight: number
): void {
    const pos = camera.getPosition();
    ctx.setTransform(1, 0, 0, 1, canvasWidth / 2 - pos.x, canvasHeight / 2 - pos.y);
}

/** Reset camera transform on canvas context. */
export function resetCameraTransform(ctx: CanvasRenderingContext2D): void {
    ctx.setTransform(1, 0, 0, 1, 0, 0);
}
