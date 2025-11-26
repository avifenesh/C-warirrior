/**
 * HTTP Backend Implementation
 *
 * Aligns with the Axum API defined in src-api/src/main.rs.
 * Keeps a thin cache of the current level data because the API
 * only returns full level data on load.
 */

// TypeScript declaration for build-time constant
declare const __API_URL__: string | undefined;

import type {
    Backend,
    UnsubscribeFn,
    GameState,
    RenderState,
    PlayerAction,
    LevelData,
    LevelInfo,
    CodeResult,
    LevelCompleteResult,
    CodeOutput,
    LevelCompleteEvent,
    GameError,
} from './types';

// Get API URL from environment or default to localhost
const API_URL = typeof __API_URL__ !== 'undefined' ? __API_URL__ : 'http://localhost:3000';

// Device ID management
const DEVICE_ID_KEY = 'code-warrior-device-id';

function getOrCreateDeviceId(): string {
    let deviceId = localStorage.getItem(DEVICE_ID_KEY);
    if (!deviceId) {
        deviceId = `device-${Date.now()}-${Math.random().toString(36).substring(2, 11)}`;
        localStorage.setItem(DEVICE_ID_KEY, deviceId);
    }
    return deviceId;
}

// HTTP request helper
async function apiRequest<T>(
    endpoint: string,
    options: RequestInit = {}
): Promise<T> {
    const deviceId = getOrCreateDeviceId();
    const url = `${API_URL}${endpoint}`;

    const response = await fetch(url, {
        ...options,
        headers: {
            'Content-Type': 'application/json',
            'X-Device-ID': deviceId,
            ...options.headers,
        },
    });

    if (!response.ok) {
        const errorText = await response.text();
        throw new Error(`HTTP ${response.status}: ${errorText}`);
    }

    return response.json();
}

// Polling manager for events
class EventPoller {
    private intervals: Map<string, number> = new Map();
    private callbacks: Map<string, Set<Function>> = new Map();
    private lastStates: Map<string, any> = new Map();

    subscribe(eventType: string, callback: Function, pollInterval: number = 100): UnsubscribeFn {
        // Add callback to the set
        if (!this.callbacks.has(eventType)) {
            this.callbacks.set(eventType, new Set());
        }
        this.callbacks.get(eventType)!.add(callback);

        // Start polling if not already running
        if (!this.intervals.has(eventType)) {
            this.startPolling(eventType, pollInterval);
        }

        // Return unsubscribe function
        return () => {
            const callbacks = this.callbacks.get(eventType);
            if (callbacks) {
                callbacks.delete(callback);
                if (callbacks.size === 0) {
                    this.stopPolling(eventType);
                }
            }
        };
    }

    private startPolling(eventType: string, interval: number) {
        const pollFn = async () => {
            try {
                const callbacks = this.callbacks.get(eventType);
                if (!callbacks || callbacks.size === 0) {
                    this.stopPolling(eventType);
                    return;
                }

                // Poll based on event type
                let data: any = null;
                switch (eventType) {
                    case 'game-tick':
                        data = await apiRequest<RenderState>('/api/game/render-state');
                        break;
                    default:
                        // Other events are not implemented on HTTP backend yet
                        break;
                }

                if (data) {
                    // Check if state actually changed (for game-tick)
                    if (eventType === 'game-tick') {
                        const lastState = this.lastStates.get(eventType);
                        const stateStr = JSON.stringify(data);
                        if (lastState === stateStr) {
                            return; // No change, skip callbacks
                        }
                        this.lastStates.set(eventType, stateStr);
                    }

                    // Trigger all callbacks
                    callbacks.forEach(cb => {
                        try {
                            cb(data);
                        } catch (err) {
                            console.error(`Error in ${eventType} callback:`, err);
                        }
                    });
                }
            } catch (err) {
                // Silently ignore polling errors (server might not be ready)
                console.debug(`Polling error for ${eventType}:`, err);
            }
        };

        const intervalId = window.setInterval(pollFn, interval);
        this.intervals.set(eventType, intervalId);

        // Do initial poll
        pollFn();
    }

    private stopPolling(eventType: string) {
        const intervalId = this.intervals.get(eventType);
        if (intervalId !== undefined) {
            window.clearInterval(intervalId);
            this.intervals.delete(eventType);
            this.lastStates.delete(eventType);
        }
    }

    cleanup() {
        this.intervals.forEach(intervalId => window.clearInterval(intervalId));
        this.intervals.clear();
        this.callbacks.clear();
        this.lastStates.clear();
    }
}

// HTTP Backend implementation
class HttpBackend implements Backend {
    private poller = new EventPoller();
    private currentLevelData: LevelData | null = null;

    // Game lifecycle
    async initGame(): Promise<RenderState> {
        // API returns device_id + full GameState; we immediately fetch render state for UI.
        await apiRequest('/api/game/init', {
            method: 'POST',
            body: JSON.stringify({})
        });
        return this.getRenderState();
    }

    async getGameState(): Promise<GameState> {
        return apiRequest<GameState>('/api/game/state');
    }

    async getRenderState(): Promise<RenderState> {
        return apiRequest<RenderState>('/api/game/render-state');
    }

    async processAction(action: PlayerAction): Promise<RenderState> {
        return apiRequest<RenderState>('/api/game/action', {
            method: 'POST',
            body: JSON.stringify(action),
        });
    }

    // Levels
    async getAvailableLevels(): Promise<LevelInfo[]> {
        return apiRequest<LevelInfo[]>('/api/levels');
    }

    async loadLevel(levelId: string): Promise<void> {
        const payload = await apiRequest<{ level_data: LevelData; render_state: RenderState }>(`/api/levels/${levelId}/load`, {
            method: 'POST',
        });
        this.currentLevelData = payload.level_data;
        // Keep render state current so callers can optionally refresh without another GET
        // but still return void to satisfy interface.
    }

    async getLevelData(): Promise<LevelData> {
        if (!this.currentLevelData) {
            throw new Error('No level loaded yet');
        }
        return this.currentLevelData;
    }

    // Code
    async submitCode(code: string): Promise<CodeResult> {
        const result = await apiRequest<CodeResult & { render_state?: RenderState; xp_earned?: number }>(
            '/api/code/submit',
            {
                method: 'POST',
                body: JSON.stringify({ code }),
            }
        );
        // If backend included an updated render_state, cache it for the next tick
        if (result && 'render_state' in result && result.render_state) {
            // No dedicated cache here; callers can call getRenderState after submit
        }
        return result;
    }

    async getHint(hintIndex: number): Promise<string> {
        if (this.currentLevelData && this.currentLevelData.hints[hintIndex]) {
            return this.currentLevelData.hints[hintIndex];
        }
        throw new Error('No more hints available');
    }

    // Progress
    async completeLevel(): Promise<LevelCompleteResult> {
        throw new Error('completeLevel is not available on the HTTP backend');
    }

    // Events (using polling)
    async onGameTick(cb: (state: RenderState) => void): Promise<UnsubscribeFn> {
        return this.poller.subscribe('game-tick', cb, 150);
    }

    async onCodeOutput(_cb: (output: CodeOutput) => void): Promise<UnsubscribeFn> {
        // Not implemented for HTTP backend - code output comes from submit response
        return () => {};
    }

    async onLevelComplete(_cb: (event: LevelCompleteEvent) => void): Promise<UnsubscribeFn> {
        // Not implemented for HTTP backend - level complete comes from submit response
        return () => {};
    }

    async onGameError(_cb: (error: GameError) => void): Promise<UnsubscribeFn> {
        // Not implemented for HTTP backend - errors come from API responses
        return () => {};
    }

    // Cleanup
    cleanup(): void {
        this.poller.cleanup();
    }
}

// Factory function
export function createHttpBackend(): Backend {
    return new HttpBackend();
}
