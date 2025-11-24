/** Cardinal movement directions. */
export type Direction = 'up' | 'down' | 'left' | 'right';

/** Snapshot of current input state. */
export interface InputState {
    movement: Direction | null;
    interact: boolean;
    escape: boolean;
    submit: boolean;
}

/** Public input handler API. */
export interface InputHandler {
    getState(): InputState;
    isPressed(key: string): boolean;
    onInteract(callback: () => void): () => void;
    onEscape(callback: () => void): () => void;
    destroy(): void;
}

const movementKeys: Record<string, Direction> = {
    w: 'up',
    arrowup: 'up',
    a: 'left',
    arrowleft: 'left',
    s: 'down',
    arrowdown: 'down',
    d: 'right',
    arrowright: 'right',
};

/** Create input handler that listens to keyboard events. */
export function createInputHandler(): InputHandler {
    const pressed = new Set<string>();
    let movement: Direction | null = null;
    let interact = false;
    let escape = false;
    let submit = false;

    const interactListeners = new Set<() => void>();
    const escapeListeners = new Set<() => void>();

    const keydown = (event: KeyboardEvent) => {
        const key = event.key.toLowerCase();
        pressed.add(key);

        if (movementKeys[key]) {
            event.preventDefault();
            movement = movementKeys[key];
        }

        if (key === 'e' || key === ' ') {
            event.preventDefault();
            interact = true;
            interactListeners.forEach((cb) => cb());
        }

        if (key === 'escape') {
            escape = true;
            escapeListeners.forEach((cb) => cb());
        }

        if (key === 'enter') {
            submit = true;
        }
    };

    const keyup = (event: KeyboardEvent) => {
        const key = event.key.toLowerCase();
        pressed.delete(key);

        if (movementKeys[key] && movement === movementKeys[key]) {
            // find another held movement key if any
            const fallback = Array.from(pressed).find((k) => movementKeys[k]);
            movement = fallback ? movementKeys[fallback] : null;
        }
    };

    window.addEventListener('keydown', keydown);
    window.addEventListener('keyup', keyup);

    return {
        getState() {
            const snapshot: InputState = { movement, interact, escape, submit };
            // reset edge-triggered flags after read
            interact = false;
            escape = false;
            submit = false;
            return snapshot;
        },
        isPressed(key: string) {
            return pressed.has(key.toLowerCase());
        },
        onInteract(callback: () => void) {
            interactListeners.add(callback);
            return () => interactListeners.delete(callback);
        },
        onEscape(callback: () => void) {
            escapeListeners.add(callback);
            return () => escapeListeners.delete(callback);
        },
        destroy() {
            window.removeEventListener('keydown', keydown);
            window.removeEventListener('keyup', keyup);
            interactListeners.clear();
            escapeListeners.clear();
            pressed.clear();
        },
    };
}
