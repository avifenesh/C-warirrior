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
    CodeOutput,
    LevelCompleteEvent,
    GameError,
    SaveSlot,
    PlayerProgress,
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
    private pollIntervals: Map<string, number> = new Map();
    private callbacks: Map<string, Set<Function>> = new Map();
    private lastStates: Map<string, any> = new Map();
    private shouldPoll: Map<string, () => boolean> = new Map();
    private fetchers: Map<string, () => Promise<any>> = new Map();
    private paused = false;
    private removeVisibilityListener: (() => void) | null = null;

    constructor() {
        if (typeof document !== 'undefined') {
            const handler = () => {
                if (document.hidden) {
                    this.pauseAll();
                } else {
                    this.resumeAll();
                }
            };
            document.addEventListener('visibilitychange', handler);
            this.removeVisibilityListener = () => document.removeEventListener('visibilitychange', handler);
        }
    }

    subscribe(
        eventType: string,
        callback: Function,
        pollInterval: number = 500,
        shouldPoll?: () => boolean,
        fetcher?: () => Promise<any>
    ): UnsubscribeFn {
        // Add callback to the set
        if (!this.callbacks.has(eventType)) {
            this.callbacks.set(eventType, new Set());
        }
        this.callbacks.get(eventType)!.add(callback);

        if (shouldPoll) this.shouldPoll.set(eventType, shouldPoll);
        if (fetcher) this.fetchers.set(eventType, fetcher);
        this.pollIntervals.set(eventType, pollInterval);

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

    private isPollingAllowed(eventType: string): boolean {
        if (this.paused) return false;
        if (typeof document !== 'undefined' && document.hidden) return false;
        const shouldPoll = this.shouldPoll.get(eventType);
        return shouldPoll ? shouldPoll() : true;
    }

    private startPolling(eventType: string, interval: number) {
        const pollFn = async () => {
            if (!this.isPollingAllowed(eventType)) {
                return;
            }

            try {
                const callbacks = this.callbacks.get(eventType);
                if (!callbacks || callbacks.size === 0) {
                    this.stopPolling(eventType);
                    return;
                }

                // Poll based on event type or custom fetcher
                let data: any = null;
                const fetcher = this.fetchers.get(eventType);
                if (fetcher) {
                    data = await fetcher();
                } else {
                    switch (eventType) {
                        case 'game-tick':
                            data = await apiRequest<RenderState>('/api/game/render-state');
                            break;
                        default:
                            break;
                    }
                }

                if (data) {
                    if (eventType === 'game-tick') {
                        const lastState = this.lastStates.get(eventType);
                        const stateStr = JSON.stringify(data);
                        if (lastState === stateStr) {
                            return; // No change, skip callbacks
                        }
                        this.lastStates.set(eventType, stateStr);
                    }

                    callbacks.forEach(cb => {
                        try {
                            cb(data);
                        } catch (err) {
                            console.error(`Error in ${eventType} callback:`, err);
                        }
                    });
                }
            } catch {
                // Silently ignore polling errors (server might not be ready)
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

    private pauseAll() {
        this.paused = true;
        this.intervals.forEach(intervalId => window.clearInterval(intervalId));
        this.intervals.clear();
    }

    private resumeAll() {
        if (!this.paused) return;
        this.paused = false;
        this.callbacks.forEach((set, eventType) => {
            if (set.size === 0 || this.intervals.has(eventType)) return;
            const interval = this.pollIntervals.get(eventType) ?? 500;
            this.startPolling(eventType, interval);
        });
    }

    cleanup() {
        this.intervals.forEach(intervalId => window.clearInterval(intervalId));
        this.intervals.clear();
        this.callbacks.clear();
        this.lastStates.clear();
        this.fetchers.clear();
        this.shouldPoll.clear();
        if (this.removeVisibilityListener) {
            this.removeVisibilityListener();
            this.removeVisibilityListener = null;
        }
    }
}

// Polling configuration
const POLLING_CONFIG = {
    /** Polling interval (ms) */
    INTERVAL: 500,
    /** Time without activity before stopping polling entirely (ms) */
    IDLE_STOP_THRESHOLD: 30000,
};

// HTTP Backend implementation
class HttpBackend implements Backend {
    private poller = new EventPoller();
    private currentLevelData: LevelData | null = null;
    private renderStateCache: { state: RenderState; timestamp: number } | null = null;

    // Activity tracking for smart polling
    private lastActivityTime: number = Date.now();

    /** Mark user activity to keep polling active */
    private markActivity(): void {
        this.lastActivityTime = Date.now();
    }

    /** Check if polling should be active */
    private shouldContinuePolling(): boolean {
        // Don't poll if no level is loaded
        if (this.currentLevelData === null) return false;

        // Stop polling after extended idle period to save resources
        const idleTime = Date.now() - this.lastActivityTime;
        if (idleTime > POLLING_CONFIG.IDLE_STOP_THRESHOLD) return false;

        return true;
    }

    private cacheRenderState(state: RenderState) {
        this.renderStateCache = { state, timestamp: Date.now() };
    }

    private getCachedRenderState(maxAgeMs = 500): RenderState | null {
        if (!this.renderStateCache) return null;
        if (Date.now() - this.renderStateCache.timestamp <= maxAgeMs) {
            return this.renderStateCache.state;
        }
        return null;
    }

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
        const cached = this.getCachedRenderState();
        if (cached) return cached;

        const state = await apiRequest<RenderState>('/api/game/render-state');
        this.cacheRenderState(state);
        return state;
    }

    async processAction(action: PlayerAction): Promise<RenderState> {
        this.markActivity(); // User is actively playing
        const state = await apiRequest<RenderState>('/api/game/action', {
            method: 'POST',
            body: JSON.stringify(action),
        });
        this.cacheRenderState(state);
        return state;
    }

    // Levels
    async getAvailableLevels(): Promise<LevelInfo[]> {
        return apiRequest<LevelInfo[]>('/api/levels');
    }

    async loadLevel(levelId: string): Promise<void> {
        this.markActivity(); // User started a level
        const payload = await apiRequest<{ level_data: LevelData; render_state: RenderState }>(`/api/levels/${levelId}/load`, {
            method: 'POST',
        });
        this.currentLevelData = payload.level_data;
        this.cacheRenderState(payload.render_state);
        // Keep render state current so callers can optionally refresh without another GET
        // but still return void to satisfy interface.
    }

    async getLevelData(): Promise<LevelData> {
        // Prefer fetching from API for source of truth
        try {
            const levelData = await apiRequest<LevelData>('/api/levels/current');
            this.currentLevelData = levelData;
            return levelData;
        } catch {
            // Fallback to cache if API fails
            if (this.currentLevelData) {
                return this.currentLevelData;
            }
            throw new Error('No level loaded yet');
        }
    }

    // Code
    async submitCode(code: string, testOnly: boolean = false): Promise<CodeResult> {
        this.markActivity(); // User submitted code
        const result = await apiRequest<CodeResult & { render_state?: RenderState; xp_earned?: number }>(
            '/api/code/submit',
            {
                method: 'POST',
                body: JSON.stringify({ code, test_only: testOnly }),
            }
        );
        // If backend included an updated render_state, cache it for the next tick
        if (result && 'render_state' in result && result.render_state) {
            this.cacheRenderState(result.render_state);
        }
        return result;
    }

    async getHint(hintIndex: number): Promise<string> {
        // Fetch hint from API
        try {
            const hint = await apiRequest<string>(`/api/code/hint/${hintIndex}`);
            return hint;
        } catch {
            // Fallback to local cache if API fails
            if (this.currentLevelData && this.currentLevelData.hints[hintIndex]) {
                return this.currentLevelData.hints[hintIndex];
            }
            throw new Error('No more hints available');
        }
    }

    // Save/Load
    async listSaves(): Promise<SaveSlot[]> {
        return apiRequest<SaveSlot[]>('/api/saves');
    }

    async saveGame(slotId: string): Promise<void> {
        await apiRequest(`/api/saves/${slotId}`, { method: 'POST' });
    }

    async loadGame(slotId: string): Promise<RenderState> {
        const result = await apiRequest<{ render_state: RenderState }>(`/api/saves/${slotId}`);
        if (result.render_state) {
            this.cacheRenderState(result.render_state);
        }
        return result.render_state;
    }

    async deleteSave(slotId: string): Promise<void> {
        await apiRequest(`/api/saves/${slotId}`, { method: 'DELETE' });
    }

    // Progress
    async getProgress(): Promise<PlayerProgress> {
        return apiRequest<PlayerProgress>('/api/player/progress');
    }

    // Events (using polling)
    // Polling stops after 30s of inactivity to save resources; resumes on user action
    async onGameTick(cb: (state: RenderState) => void): Promise<UnsubscribeFn> {
        return this.poller.subscribe(
            'game-tick',
            cb,
            POLLING_CONFIG.INTERVAL,
            () => this.shouldContinuePolling(),
            () => this.getRenderState()
        );
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
        this.renderStateCache = null;
    }
}

// Factory function
export function createHttpBackend(): Backend {
    return new HttpBackend();
}
