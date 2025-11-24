export interface Animation {
    frames: number[];
    frameDuration: number; // ms per frame
    loop: boolean;
}

export interface AnimationState {
    animation: Animation;
    startTime: number;
    finished: boolean;
}

/** Common animation presets. */
export const ANIMATIONS = {
    playerIdle: { frames: [0], frameDuration: 1000, loop: true },
    playerWalk: { frames: [0, 1, 0, 2], frameDuration: 150, loop: true },
    terminalGlow: { frames: [0, 1, 2, 1], frameDuration: 200, loop: true },
} as const;

/** Start a new animation state. */
export function startAnimation(animation: Animation): AnimationState {
    return {
        animation,
        startTime: performance.now(),
        finished: false,
    };
}

/** Get current frame index from an animation state at the given time (ms). */
export function getCurrentFrame(state: AnimationState, currentTime: number): number {
    const { animation } = state;
    const elapsed = Math.max(0, currentTime - state.startTime);
    const totalDuration = animation.frames.length * animation.frameDuration;

    if (!animation.loop && elapsed >= totalDuration) {
        state.finished = true;
        return animation.frames[animation.frames.length - 1];
    }

    const frameIndex = Math.floor(elapsed / animation.frameDuration) % animation.frames.length;
    return animation.frames[frameIndex];
}

/** Linear interpolation helper. */
export function lerp(start: number, end: number, t: number): number {
    return start + (end - start) * t;
}

/** Quadratic ease-out. */
export function easeOutQuad(t: number): number {
    return 1 - (1 - t) * (1 - t);
}

/** Quadratic ease-in-out. */
export function easeInOutQuad(t: number): number {
    return t < 0.5 ? 2 * t * t : 1 - Math.pow(-2 * t + 2, 2) / 2;
}
