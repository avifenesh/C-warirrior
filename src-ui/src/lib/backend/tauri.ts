/**
 * Tauri Backend Implementation
 *
 * Uses Tauri's invoke and listen APIs for desktop app communication.
 */

import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
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
    QuestInfo,
} from './types';

export function createTauriBackend(): Backend {
    const unsubscribers: UnlistenFn[] = [];

    return {
        // Game lifecycle
        initGame: () => invoke<RenderState>('init_game'),
        getGameState: () => invoke<GameState>('get_game_state'),
        getRenderState: () => invoke<RenderState>('get_render_state'),
        processAction: (action: PlayerAction) =>
            invoke<RenderState>('process_action', { action }),

        // Levels
        getAvailableLevels: () => invoke<LevelInfo[]>('get_available_levels'),
        loadLevel: (levelId: string) => invoke('load_level', { levelId }),
        getLevelData: () => invoke<LevelData>('get_level_data'),

        // Quests (for multi-quest levels)
        getLevelQuests: () => invoke<QuestInfo[]>('get_level_quests'),
        loadQuest: (questId: string) => invoke<QuestInfo>('load_quest', { questId }),
        submitQuestCode: (code: string, questId: string, testOnly: boolean = false) =>
            invoke<CodeResult>('submit_quest_code', { code, questId, testOnly }),

        // Code
        submitCode: (code: string, testOnly: boolean = false) =>
            invoke<CodeResult>('submit_code', { code, testOnly }),
        getHint: (hintIndex: number) => invoke<string>('get_hint', { hintIndex }),

        // Save/Load
        listSaves: () => invoke<SaveSlot[]>('list_saves'),
        saveGame: (slotId: string) => invoke('save_game', { slotId }),
        loadGame: (slotId: string) => invoke<RenderState>('load_game', { slotId }),
        deleteSave: (slotId: string) => invoke('delete_save', { slotId }),

        // Progress
        getProgress: () => invoke<PlayerProgress>('get_progress'),

        // Events
        async onGameTick(cb: (state: RenderState) => void): Promise<UnsubscribeFn> {
            const unsub = await listen<RenderState>('game_tick', (e) => cb(e.payload));
            unsubscribers.push(unsub);
            return unsub;
        },

        async onCodeOutput(cb: (output: CodeOutput) => void): Promise<UnsubscribeFn> {
            const unsub = await listen<CodeOutput>('code_output', (e) => cb(e.payload));
            unsubscribers.push(unsub);
            return unsub;
        },

        async onLevelComplete(cb: (event: LevelCompleteEvent) => void): Promise<UnsubscribeFn> {
            const unsub = await listen<LevelCompleteEvent>('level_complete', (e) => cb(e.payload));
            unsubscribers.push(unsub);
            return unsub;
        },

        async onGameError(cb: (error: GameError) => void): Promise<UnsubscribeFn> {
            const unsub = await listen<GameError>('game_error', (e) => cb(e.payload));
            unsubscribers.push(unsub);
            return unsub;
        },

        cleanup() {
            unsubscribers.forEach((unsub) => unsub());
            unsubscribers.length = 0;
        },
    };
}
