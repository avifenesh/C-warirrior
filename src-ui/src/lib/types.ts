// ============================================================================
// Core Types (from docs/interfaces/game-state.md)
// ============================================================================

export interface Position {
    x: number;
    y: number;
}

export type Direction = 'up' | 'down' | 'left' | 'right';

export interface Player {
    position: Position;
    health: number;
    max_health: number;
    xp: number;
    level: number;
    facing: Direction;
}

export type ItemType = 'key' | 'weapon' | 'consumable' | 'quest_item';

export interface Item {
    id: string;
    name: string;
    item_type: ItemType;
    description: string;
    quantity: number;
}

export interface Inventory {
    items: Item[];
    max_slots: number;
}

export type TileType = 'floor' | 'wall' | 'water' | 'void' | 'door' | 'terminal';

export interface Tile {
    tile_type: TileType;
    walkable: boolean;
    interactable: boolean;
}

export interface World {
    width: number;
    height: number;
    tiles: Tile[][];
    spawn_point: Position;
}

export type GamePhase = 'main_menu' | 'playing' | 'coding' | 'paused' | 'level_complete';

export interface GameState {
    player: Player;
    world: World;
    inventory: Inventory;
    current_level_id: string | null;
    game_phase: GamePhase;
    total_xp: number;
    levels_completed: string[];
}

export type ObjectType = 'terminal' | 'door' | 'npc' | 'collectible';

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

export type PlayerAction =
    | { type: 'move'; direction: Direction }
    | { type: 'interact' }
    | { type: 'submit_code'; code: string }
    | { type: 'open_inventory' }
    | { type: 'use_item'; item_id: string }
    | { type: 'pause' }
    | { type: 'resume' };

// ============================================================================
// Level Types (from docs/interfaces/level-schema.md)
// ============================================================================

export type WorldPreset = 'tutorial' | 'corridor' | 'maze' | { custom: string };

export interface WorldConfig {
    width: number;
    height: number;
    spawn_x: number;
    spawn_y: number;
    terminal_x: number;
    terminal_y: number;
    preset: WorldPreset;
}

export type SuccessCriteria =
    | { type: 'exact_match'; expected_stdout: string }
    | { type: 'regex_match'; regex: string }
    | { type: 'output_count'; token: string; count: number }
    | { type: 'compile_only' }
    | { type: 'all'; criteria: SuccessCriteria[] }
    | { type: 'any'; criteria: SuccessCriteria[] };

export interface Challenge {
    id: string;
    prompt: string;
    expected_output: string;
    starter_code: string;
}

export interface LevelData {
    id: string;
    title: string;
    concept: string;
    description: string;
    code_template: string;
    success_criteria: SuccessCriteria;
    hints: string[];
    xp_reward: number;
    world_config: WorldConfig;
    challenges: Challenge[];
}

export interface LevelInfo {
    id: string;
    title: string;
    concept: string;
    completed: boolean;
    locked: boolean;
}

export interface ExecutionOutput {
    stdout: string;
    stderr: string;
    compile_error: string | null;
    runtime_error: string | null;
    exit_code: number | null;
    execution_time_ms: number;
    timed_out: boolean;
}

export interface CodeResult {
    success: boolean;
    stdout: string;
    stderr: string;
    compile_error: string | null;
    execution_time_ms: number;
    feedback?: string;
    hint?: string | null;
    render_state?: RenderState;
    xp_earned?: number;
}

// ============================================================================
// Command Result Types (from docs/interfaces/tauri-commands.md)
// ============================================================================

export interface LevelCompleteResult {
    xp_earned: number;
    total_xp: number;
    next_level_id: string | null;
    levels_completed: number;
}

// ============================================================================
// Event Types (from docs/interfaces/tauri-commands.md)
// ============================================================================

export interface CodeOutput {
    stream: 'stdout' | 'stderr';
    content: string;
    is_final: boolean;
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
