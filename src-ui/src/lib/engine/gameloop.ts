export interface GameLoop {
    start(): void;
    stop(): void;
    isRunning(): boolean;
}

/** Function invoked for each fixed update step (deltaTime in ms). */
export type UpdateFn = (deltaTime: number) => void;

/** Function invoked for each render pass. */
export type RenderFn = () => void;

/** Create game loop with fixed timestep (deltaTime in ms). */
export function createGameLoop(update: UpdateFn, render: RenderFn, targetFps: number): GameLoop {
    let running = false;
    let rafId = 0;
    const step = 1000 / Math.max(1, targetFps);
    let last = 0;
    let accumulator = 0;

    const frame = (timestamp: number) => {
        if (!running) return;
        if (!last) last = timestamp;
        const delta = timestamp - last;
        last = timestamp;
        accumulator += delta;

        while (accumulator >= step) {
            update(step);
            accumulator -= step;
        }

        render();
        rafId = requestAnimationFrame(frame);
    };

    return {
        start() {
            if (running) return;
            running = true;
            last = 0;
            accumulator = 0;
            rafId = requestAnimationFrame(frame);
        },
        stop() {
            running = false;
            cancelAnimationFrame(rafId);
        },
        isRunning() {
            return running;
        },
    };
}

/** Create render-only loop using requestAnimationFrame. */
export function createRenderLoop(render: RenderFn): GameLoop {
    let running = false;
    let rafId = 0;

    const frame = () => {
        if (!running) return;
        render();
        rafId = requestAnimationFrame(frame);
    };

    return {
        start() {
            if (running) return;
            running = true;
            rafId = requestAnimationFrame(frame);
        },
        stop() {
            running = false;
            cancelAnimationFrame(rafId);
        },
        isRunning() {
            return running;
        },
    };
}
