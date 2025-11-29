/**
 * Backend Abstraction Layer
 *
 * This interface allows the frontend to work with both:
 * - Tauri (desktop): Uses invoke/listen
 * - HTTP (web): Uses fetch to Railway API
 */

import type {
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
} from '../types';

export type UnsubscribeFn = () => void;

export interface Backend {
    // Game lifecycle
    initGame(): Promise<RenderState>;
    getGameState(): Promise<GameState>;
    getRenderState(): Promise<RenderState>;
    processAction(action: PlayerAction): Promise<RenderState>;

    // Levels
    getAvailableLevels(): Promise<LevelInfo[]>;
    loadLevel(levelId: string): Promise<void>;
    getLevelData(): Promise<LevelData>;

    // Code
    submitCode(code: string): Promise<CodeResult>;
    getHint(hintIndex: number): Promise<string>;

    // Save/Load
    listSaves(): Promise<SaveSlot[]>;
    saveGame(slotId: string): Promise<void>;
    loadGame(slotId: string): Promise<RenderState>;
    deleteSave(slotId: string): Promise<void>;

    // Progress
    getProgress(): Promise<PlayerProgress>;

    // Events (returns unsubscribe function)
    onGameTick(cb: (state: RenderState) => void): Promise<UnsubscribeFn>;
    onCodeOutput(cb: (output: CodeOutput) => void): Promise<UnsubscribeFn>;
    onLevelComplete(cb: (event: LevelCompleteEvent) => void): Promise<UnsubscribeFn>;
    onGameError(cb: (error: GameError) => void): Promise<UnsubscribeFn>;

    // Cleanup
    cleanup(): void;
}

// Re-export types for convenience
export type {
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
};
