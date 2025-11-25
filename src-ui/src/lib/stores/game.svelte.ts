import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { writable, derived, get } from 'svelte/store';

// --- Shared Types ---
export type Direction = 'up' | 'down' | 'left' | 'right';
export type GamePhase = 'main_menu' | 'playing' | 'coding' | 'paused' | 'level_complete';

export interface Position {
    x: number;
    y: number;
}

export type TileType = 'floor' | 'wall' | 'water' | 'void' | 'door' | 'terminal';

export interface Tile {
    tile_type: TileType;
    walkable: boolean;
    interactable: boolean;
}

export interface Player {
    position: Position;
    health: number;
    max_health: number;
    xp: number;
    level: number;
    facing: Direction;
}

export interface RenderState {
    player: Player;
    visible_tiles: Tile[][];
    viewport_offset: Position;
    game_phase: GamePhase;
    current_level_id: string | null;
    map?: TileMapRender | null;
    objects: ObjectRender[];
    show_terminal: boolean;
    active_dialogue: string | null;
}

export interface TileMapRender {
    width: number;
    height: number;
    tiles: TileType[][];
}

export interface ObjectRender {
    object_type: ObjectType;
    position: Position;
    sprite_id?: string | null;
}

export type ObjectType = 'terminal' | 'door' | 'npc' | 'collectible';

export type PlayerAction =
    | { type: 'move'; direction: Direction }
    | { type: 'interact' }
    | { type: 'submit_code'; code: string }
    | { type: 'open_inventory' }
    | { type: 'use_item'; item_id: string }
    | { type: 'pause' }
    | { type: 'resume' };

export interface LevelInfo {
    id: string;
    title: string;
    concept: string;
    completed: boolean;
    locked: boolean;
}

export interface CodeOutput {
    stream: 'stdout' | 'stderr';
    content: string;
    is_final: boolean;
}

export interface CodeResult {
    success: boolean;
    stdout: string;
    stderr: string;
    compile_error: string | null;
    execution_time_ms: number;
    feedback?: string;
    hint?: string | null;
}

export interface LevelCompleteEvent {
    level_id: string;
    xp_earned: number;
    next_level_id: string | null;
}

export interface GameError {
    code: string;
    message: string;
    recoverable: boolean;
}

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

    // Ensure submission flag clears whenever a result arrives
    lastCodeResult.subscribe(() => codeSubmitting.set(false));

    let tickUnsub: UnlistenFn | null = null;
    let errorUnsub: UnlistenFn | null = null;
    let codeUnsub: UnlistenFn | null = null;
    let levelCompleteUnsub: UnlistenFn | null = null;

    async function boot() {
        ui.update(u => ({ ...u, loading: true, status: 'Resetting backend state...', error: null }));

        try {
            const initialState = await invoke<RenderState>('init_game');
            renderState.set(initialState);
            // Force UI update immediately so user sees the game
            ui.update(u => ({ ...u, loading: false }));

            ui.update(u => ({ ...u, status: 'Loading levels...' }));
            await hydrateLevels();

            await bindEvents();

            // Auto-load Level 1 so code submission works
            ui.update(u => ({ ...u, status: 'Loading Level 1...' }));
            try {
                await invoke('load_level', { levelId: 'L01' });
                // Get updated render state via a no-op action
                const stateAfterLoad = await invoke<RenderState>('process_action', { action: { type: 'resume' } });
                renderState.set(stateAfterLoad);
            } catch (loadErr) {
                console.warn('Failed to auto-load L01:', loadErr);
                // Continue anyway - game will work without level loaded for movement
            }
            ui.update(u => ({ ...u, status: 'Live' }));
        } catch (err) {
            console.error('Boot error:', err);
            ui.update(u => ({ ...u, error: normalizeError(err) }));
        } finally {
            ui.update(u => ({ ...u, loading: false }));
        }
    }

    async function hydrateLevels() {
        try {
            const data = await invoke<LevelInfo[]>('get_available_levels');
            levels.set(data);
        } catch (err) {
            ui.update(u => ({ ...u, error: u.error ?? normalizeError(err) }));
        }
    }

    async function bindEvents() {
        if (!tickUnsub) {
            tickUnsub = await listen<RenderState>('game_tick', (event) => {
                renderState.set(event.payload);
                ui.update(u => ({ ...u, status: 'Live' }));
            });
        }

        if (!errorUnsub) {
            errorUnsub = await listen<GameError>('game_error', (event) => {
                ui.update(u => ({ ...u, error: event.payload.message }));
            });
        }

        if (!codeUnsub) {
            codeUnsub = await listen<CodeOutput>('code_output', (event) => {
                lastCodeOutput.set(event.payload);
                if (event.payload.is_final) {
                    codeSubmitting.set(false);
                }
            });
        }

        if (!levelCompleteUnsub) {
            levelCompleteUnsub = await listen<LevelCompleteEvent>('level_complete', (event) => {
                lastLevelComplete.set(event.payload);
            });
        }
    }

    async function startLevel(levelId: string) {
        ui.update(u => ({ ...u, loading: true, status: `Loading ${levelId}...`, error: null }));

        try {
            await invoke('load_level', { levelId });
            ui.update(u => ({ ...u, status: `Level ${levelId} loaded` }));
        } catch (err) {
            ui.update(u => ({ ...u, error: normalizeError(err) }));
        } finally {
            ui.update(u => ({ ...u, loading: false }));
        }
    }

    async function sendAction(action: PlayerAction) {
        ui.update(u => ({ ...u, error: null }));
        try {
            const next = await invoke<RenderState>('process_action', { action });
            renderState.set(next);
        } catch (err) {
            ui.update(u => ({ ...u, error: normalizeError(err) }));
        }
    }

    async function submitCode(code: string) {
        ui.update(u => ({ ...u, error: null }));
        codeSubmitting.set(true);
        try {
            const result = await invoke<CodeResult>('submit_code', { code });
            lastCodeResult.set(result);
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
        boot,
        startLevel,
        sendAction,
        submitCode,
        refreshLevels: hydrateLevels,
        cleanup,
    };
}

function normalizeError(err: unknown): string {
    if (err instanceof Error) return err.message;
    return typeof err === 'string' ? err : 'Unknown error';
}

export const gameStore = createGameStore();
