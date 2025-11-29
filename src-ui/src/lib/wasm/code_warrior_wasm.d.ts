/* tslint:disable */
/* eslint-disable */
export class WasmGame {
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Load a level locally from embedded levels.json
   */
  load_level(level_id: string): any;
  /**
   * Mark level complete (call after HTTP backend confirms code success)
   */
  complete_level(xp_reward: number): any;
  /**
   * Get full game state (for syncing to server)
   */
  get_game_state(): any;
  /**
   * Get data for currently loaded level
   */
  get_level_data(): any;
  /**
   * Process a player action locally (movement, interact, pause)
   * Code submission is NOT handled here - must go through HTTP
   */
  process_action(action_json: any): any;
  /**
   * Initialize or restore game state from server
   */
  init_from_state(state_json: any): void;
  /**
   * Get current render state for UI
   */
  get_render_state(): any;
  /**
   * Sync progression state from server (after load save, login, etc.)
   */
  sync_progression(total_xp: number, completed_levels: string[]): void;
  /**
   * Get list of all levels with current unlock/completion status
   */
  get_available_levels(): any;
  /**
   * Create a new game instance with embedded level data
   */
  constructor();
  /**
   * Get hint for current level by index
   */
  get_hint(hint_index: number): string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_wasmgame_free: (a: number, b: number) => void;
  readonly wasmgame_complete_level: (a: number, b: number) => [number, number, number];
  readonly wasmgame_get_available_levels: (a: number) => [number, number, number];
  readonly wasmgame_get_game_state: (a: number) => [number, number, number];
  readonly wasmgame_get_hint: (a: number, b: number) => [number, number, number, number];
  readonly wasmgame_get_level_data: (a: number) => [number, number, number];
  readonly wasmgame_get_render_state: (a: number) => [number, number, number];
  readonly wasmgame_init_from_state: (a: number, b: any) => [number, number];
  readonly wasmgame_load_level: (a: number, b: number, c: number) => [number, number, number];
  readonly wasmgame_new: () => number;
  readonly wasmgame_process_action: (a: number, b: any) => [number, number, number];
  readonly wasmgame_sync_progression: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
