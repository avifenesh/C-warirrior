/**
 * Backend Abstraction Layer
 *
 * This interface allows the frontend to work with both:
 * - HTTP (web): Uses fetch to Railway API
 */

import type {
    GameState,
    RenderState,
    PlayerAction,
    LevelData,
    LevelInfo,
    CodeResult,
    SaveSlot,
    PlayerProgress,
    QuestInfo,
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

    // Quests (for multi-quest levels)
    getLevelQuests(): Promise<QuestInfo[]>;
    loadQuest(questId: string): Promise<QuestInfo>;
    submitQuestCode(code: string, questId: string, testOnly?: boolean): Promise<CodeResult>;

    // Code
    submitCode(code: string, testOnly?: boolean): Promise<CodeResult>;
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
    SaveSlot,
    PlayerProgress,
    QuestInfo,
};
