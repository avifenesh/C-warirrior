import { writable, derived, get } from 'svelte/store';
import { getBackend, type Backend, type UnsubscribeFn } from '$lib/backend';
import type {
    Direction,
    GamePhase,
    Position,
    TileType,
    Tile,
    Player,
    RenderState,
    TileMapRender,
    ObjectRender,
    ObjectType,
    PlayerAction,
    LevelInfo,
    CodeOutput,
    CodeResult,
    LevelCompleteEvent,
    GameError,
    Challenge,
    LevelData
} from '$lib/types';

interface UIState {
    loading: boolean;
    status: string;
    error: string | null;
}

const DEFAULT_STATUS = 'Waiting for backend tick...';

function createGameStore() {
    const renderState = writable<RenderState | null>(null);
    const ui = writable<UIState>({ loading: true, status: 'Booting Code Warrior...', error: null });
    const levels = writable<LevelInfo[]>([]);
    const lastCodeOutput = writable<CodeOutput | null>(null);
    const lastLevelComplete = writable<LevelCompleteEvent | null>(null);
    const codeSubmitting = writable(false);
    const lastCodeResult = writable<CodeResult | null>(null);
    const currentLevelData = writable<LevelData | null>(null);
    const currentHintIndex = writable(0);

    // Ensure submission flag clears whenever a result arrives
    lastCodeResult.subscribe(() => codeSubmitting.set(false));

    let backend: Backend | null = null;
    let tickUnsub: UnsubscribeFn | null = null;
    let errorUnsub: UnsubscribeFn | null = null;
    let codeUnsub: UnsubscribeFn | null = null;
    let levelCompleteUnsub: UnsubscribeFn | null = null;

    async function boot() {
        console.log('[BOOT] Starting boot sequence...');
        ui.update(u => ({ ...u, loading: true, status: 'Initializing backend...', error: null }));

        try {
            // Get backend instance first
            console.log('[BOOT] Getting backend...');
            backend = await getBackend();
            console.log('[BOOT] Backend acquired');

            console.log('[BOOT] Calling initGame...');
            const initialState = await backend.initGame();
            console.log('[BOOT] initGame returned:', initialState);
            renderState.set(initialState);
            console.log('[BOOT] renderState set, current value:', initialState);
            // Force UI update immediately so user sees the game
            ui.update(u => ({ ...u, loading: false }));

            console.log('[BOOT] Loading levels...');
            ui.update(u => ({ ...u, status: 'Loading levels...' }));
            await hydrateLevels();
            console.log('[BOOT] Levels loaded');

            console.log('[BOOT] Binding events...');
            await bindEvents();
            console.log('[BOOT] Events bound');

            // Don't auto-load a level - show main menu instead
            // Level will be loaded when user clicks "NEW QUEST" or "CONTINUE"
            ui.update(u => ({ ...u, status: 'Main Menu' }));
            console.log('[BOOT] Boot complete!');
        } catch (err) {
            console.error('[BOOT] Boot error:', err);
            ui.update(u => ({ ...u, error: normalizeError(err) }));
        } finally {
            ui.update(u => ({ ...u, loading: false }));
        }
    }

    async function hydrateLevels() {
        if (!backend) return;
        try {
            const data = await backend.getAvailableLevels();
            levels.set(data);
        } catch (err) {
            ui.update(u => ({ ...u, error: u.error ?? normalizeError(err) }));
        }
    }

    async function bindEvents() {
        if (!backend) return;

        if (!tickUnsub) {
            tickUnsub = await backend.onGameTick((state) => {
                renderState.set(state);
                ui.update(u => ({ ...u, status: 'Live' }));
            });
        }

        if (!errorUnsub) {
            errorUnsub = await backend.onGameError((error) => {
                ui.update(u => ({ ...u, error: error.message }));
            });
        }

        if (!codeUnsub) {
            codeUnsub = await backend.onCodeOutput((output) => {
                lastCodeOutput.set(output);
                if (output.is_final) {
                    codeSubmitting.set(false);
                }
            });
        }

        if (!levelCompleteUnsub) {
            levelCompleteUnsub = await backend.onLevelComplete((event) => {
                lastLevelComplete.set(event);
            });
        }
    }

    async function startLevel(levelId: string) {
        if (!backend) return;
        ui.update(u => ({ ...u, loading: true, status: `Loading ${levelId}...`, error: null }));

        try {
            // Load the level
            await backend.loadLevel(levelId);

            // Get level data and render state in parallel
            const [levelData, updatedState] = await Promise.all([
                backend.getLevelData(),
                backend.getRenderState(),
            ]);

            renderState.set(updatedState);
            currentLevelData.set(levelData);
            currentHintIndex.set(0); // Reset hints for new level
            lastCodeResult.set(null); // Clear previous results
            ui.update(u => ({ ...u, status: `Level ${levelId} loaded` }));
        } catch (err) {
            ui.update(u => ({ ...u, error: normalizeError(err) }));
        } finally {
            ui.update(u => ({ ...u, loading: false }));
        }
    }

    async function getNextHint(): Promise<string | null> {
        if (!backend) return null;
        const idx = get(currentHintIndex);
        try {
            const hint = await backend.getHint(idx);
            currentHintIndex.set(idx + 1);
            return hint;
        } catch {
            return null; // No more hints
        }
    }

    async function sendAction(action: PlayerAction) {
        if (!backend) return;
        ui.update(u => ({ ...u, error: null }));
        try {
            const next = await backend.processAction(action);
            renderState.set(next);
        } catch (err) {
            ui.update(u => ({ ...u, error: normalizeError(err) }));
        }
    }

    async function submitCode(code: string) {
        if (!backend) return;
        ui.update(u => ({ ...u, error: null }));
        codeSubmitting.set(true);
        try {
            const result = await backend.submitCode(code);
            lastCodeResult.set(result);

            // If successful, fetch updated render state (level complete, doors unlocked)
            if (result.success) {
                console.log('[submitCode] Success! Fetching updated state...');
                const updatedState = await backend.getRenderState();
                renderState.set(updatedState);
                console.log('[submitCode] State updated, game_phase:', updatedState.game_phase);
            }

            codeSubmitting.set(false);
        } catch (err) {
            console.error('Code submission error:', err);
            ui.update(u => ({ ...u, error: normalizeError(err) }));
        } finally {
            codeSubmitting.set(false);
        }
    }

    const phase = derived(renderState, $rs => $rs?.game_phase ?? 'main_menu');
    const currentLevelId = derived(renderState, $rs => $rs?.current_level_id ?? null);

    function cleanup() {
        tickUnsub?.();
        errorUnsub?.();
        codeUnsub?.();
        levelCompleteUnsub?.();
        backend?.cleanup();
    }

    return {
        renderState,
        ui,
        levels,
        lastCodeOutput,
        lastLevelComplete,
        phase,
        currentLevelId,
        codeSubmitting,
        lastCodeResult,
        currentLevelData,
        boot,
        startLevel,
        sendAction,
        submitCode,
        getNextHint,
        refreshLevels: hydrateLevels,
        cleanup,
    };
}

function normalizeError(err: unknown): string {
    if (err instanceof Error) return err.message;
    return typeof err === 'string' ? err : 'Unknown error';
}

export const gameStore = createGameStore();
