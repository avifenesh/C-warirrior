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

export type TileType = 'floor' | 'wall' | 'wall_top' | 'water' | 'void' | 'door' | 'terminal' | 'decoration' | 'decoration_alt' | 'tree' | 'rock' | 'lava' | 'ice' | 'bridge' | 'grass' | 'path' | 'pit';

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
    /** The quest ID of the terminal the player is interacting with */
    active_quest_id: string | null;
}

export type PlayerAction =
    | { type: 'move'; direction: Direction }
    | { type: 'interact' }
    | { type: 'submit_code'; code: string }
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

// ============================================================================
// Multi-Quest System Types
// ============================================================================

// Progressive teaching content for each quest
export interface QuestTeaching {
    concept: string;       // What this specific quest teaches
    explanation: string;   // Why/how explanation
    tip?: string;          // Pro tip for this concept
}

export interface Quest {
    id: string;
    order: number;
    title: string;
    description: string;
    recommended: boolean;
    function_signature: FunctionSignature;
    user_template: string;
    test_cases: TestCase[];
    hints: string[];
    xp_reward: number;
    teaching?: QuestTeaching; // Progressive teaching for this quest
}

export interface QuestInfo {
    id: string;
    order: number;
    title: string;
    description: string;
    recommended: boolean;
    completed: boolean;
    xp_reward: number;
    // Full quest details (included when loading specific quest)
    user_template: string;
    function_signature: FunctionSignature;
    hints: string[];
    test_cases: TestCase[];
    teaching?: QuestTeaching; // Progressive teaching for this quest
}

export interface QuestCompleteEvent {
    level_id: string;
    quest_id: string;
    xp_earned: number;
    quests_remaining: number;
}

// ============================================================================
// Function-Based Challenge Types
// ============================================================================

export interface LessonExample {
    code: string;
    explanation: string;
}

export interface Lesson {
    title: string;
    content: string[];
    examples: LessonExample[];
}

export interface FunctionParameter {
    name: string;
    type: string;
}

export interface FunctionSignature {
    name: string;
    return_type: string;
    parameters: FunctionParameter[];
}

export interface TestCase {
    input: unknown[];
    expected: string;
    sample: boolean;
}

export interface TestCaseResult {
    input: unknown[];
    expected: string;
    actual: string;
    passed: boolean;
}

export interface TestSuiteResult {
    passed: boolean;
    total: number;
    passed_count: number;
    results: TestCaseResult[];
    compilation_error?: string | null;
}

// ============================================================================

export interface LevelData {
    id: string;
    title: string;
    theme?: string; // Level visual theme (e.g., "L01_village", "L04_forest")
    concept: string;
    description: string;
    code_template: string;
    success_criteria?: SuccessCriteria;
    hints: string[];
    xp_reward: number;
    world_config: WorldConfig;
    challenges: Challenge[];
    // Multi-quest system
    quests?: Quest[];
    total_xp_reward?: number;
    // Legacy function-based challenge fields (single quest)
    lesson?: Lesson;
    function_signature?: FunctionSignature;
    user_template?: string;
    test_cases?: TestCase[];
}

export interface LevelInfo {
    id: string;
    title: string;
    concept: string;
    completed: boolean;
    locked: boolean;
    xp_reward: number;
    // Quest progress fields
    total_quests: number;
    completed_quests: number;
    completion_percentage: number;
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
    doors_unlocked?: boolean;
    test_results?: TestSuiteResult;
}

// ============================================================================
// Event Types
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
    newly_unlocked?: string[];
}

export interface GameError {
    code: string;
    message: string;
    recoverable: boolean;
}

// ============================================================================
// Save/Load Types
// ============================================================================

export interface SaveSlot {
    id: string;
    name: string;
    timestamp: string;
    progress: string;
    empty?: boolean;
}

// ============================================================================
// Progress Types
// ============================================================================

export interface PlayerProgress {
    total_xp: number;
    completed_levels: string[];
    current_level: string | null;
}
