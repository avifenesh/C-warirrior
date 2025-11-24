import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type {
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

// === Commands ===

export const api = {
    // Game
    initGame: () => invoke<GameState>('init_game'),
    getGameState: () => invoke<GameState>('get_game_state'),
    processAction: (action: PlayerAction) =>
        invoke<RenderState>('process_action', { action }),

    // Levels
    getAvailableLevels: () => invoke<LevelInfo[]>('get_available_levels'),
    loadLevel: (levelId: string) =>
        invoke<LevelData>('load_level', { levelId }),
    getLevelData: () => invoke<LevelData>('get_level_data'),

    // Code
    submitCode: (code: string) =>
        invoke<CodeResult>('submit_code', { code }),
    getHint: (hintIndex: number) =>
        invoke<string>('get_hint', { hintIndex }),

    // Progress
    completeLevel: () => invoke<LevelCompleteResult>('complete_level'),
};

// === Events ===

export const events = {
    onGameTick: (cb: (state: RenderState) => void) =>
        listen<RenderState>('game_tick', (e) => cb(e.payload)),

    onCodeOutput: (cb: (output: CodeOutput) => void) =>
        listen<CodeOutput>('code_output', (e) => cb(e.payload)),

    onLevelComplete: (cb: (event: LevelCompleteEvent) => void) =>
        listen<LevelCompleteEvent>('level_complete', (e) => cb(e.payload)),

    onGameError: (cb: (error: GameError) => void) =>
        listen<GameError>('game_error', (e) => cb(e.payload)),
};
