<script lang="ts">
    import { createEventDispatcher, type Snippet } from 'svelte';
    import type { LevelInfo } from '$lib/types';

    interface Props {
        levels?: LevelInfo[];
        loading?: boolean;
        actions?: Snippet;
    }

    let { levels = [], loading = false, actions }: Props = $props();

    const dispatcher = createEventDispatcher();

    function selectLevel(id: string) {
        dispatcher('select', { id });
    }
</script>

<div class="quest-board">
    <!-- Header -->
    <div class="quest-board-header">
        <h2 class="quest-board-title">QUEST BOARD</h2>
        <p class="quest-board-subtitle">Choose Your Path</p>
    </div>

    {#if actions}
        <div class="quest-board-actions">
            {@render actions()}
        </div>
    {/if}

    {#if !levels.length}
        <div class="quest-empty">
            <p>No quests available...</p>
        </div>
    {:else}
        <div class="quest-grid">
            {#each levels as level (level.id)}
                <article class="quest-card">
                    <!-- Quest Header -->
                    <div class="quest-card-header">
                        <div class="quest-icon">
                            {#if level.locked}
                                &#128274;
                            {:else if level.completed}
                                &#9733;
                            {:else}
                                &#9876;
                            {/if}
                        </div>
                        <div class="quest-info">
                            <h3 class="quest-title">{level.title}</h3>
                            <p class="quest-concept">{level.concept}</p>
                        </div>
                    </div>

                    <!-- Quest Status -->
                    <div class="quest-status-bar">
                        <span class="quest-status" class:locked={level.locked} class:completed={level.completed}>
                            {#if level.locked}
                                LOCKED
                            {:else if level.completed}
                                COMPLETED
                            {:else}
                                READY
                            {/if}
                        </span>
                        <button
                            class="pixel-button quest-button"
                            disabled={level.locked || loading}
                            onclick={() => selectLevel(level.id)}
                            title={level.locked ? 'Prerequisite required' : 'Play level'}
                        >
                            {level.locked ? 'LOCKED' : level.completed ? 'REPLAY' : 'START'}
                        </button>
                    </div>
                </article>
            {/each}
        </div>
    {/if}
</div>

<style>
    /* Quest Board Container */
    .quest-board {
        background: linear-gradient(180deg, #1a1a2e 0%, #16213e 100%);
        border: 4px solid #0f3460;
        border-top-color: #3a506b;
        border-left-color: #3a506b;
        box-shadow:
            inset 0 0 0 2px #0a0a1e,
            8px 8px 0 #0a0a1e;
        padding: 24px;
        image-rendering: pixelated;
    }

    /* Header */
    .quest-board-header {
        text-align: center;
        margin-bottom: 24px;
        padding-bottom: 16px;
        border-bottom: 3px solid #0f3460;
    }

    .quest-board-title {
        font-family: 'Press Start 2P', 'Courier New', monospace;
        font-size: 18px;
        color: #fbbf24;
        text-shadow: 2px 2px 0 #92400e;
        margin-bottom: 8px;
        letter-spacing: 2px;
    }

    .quest-board-subtitle {
        font-family: 'Press Start 2P', 'Courier New', monospace;
        font-size: 10px;
        color: #94a3b8;
        letter-spacing: 1px;
    }

    .quest-board-actions {
        margin-bottom: 20px;
    }

    /* Empty State */
    .quest-empty {
        text-align: center;
        padding: 40px 20px;
        font-family: 'Press Start 2P', 'Courier New', monospace;
        font-size: 10px;
        color: #64748b;
    }

    /* Quest Grid */
    .quest-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
        gap: 16px;
    }

    /* Quest Card */
    .quest-card {
        background: #0a0a1e;
        border: 3px solid #0f3460;
        border-top-color: #1e3a5f;
        border-left-color: #1e3a5f;
        box-shadow: 4px 4px 0 #050510;
        padding: 16px;
        transition: transform 0.1s;
    }

    .quest-card:hover {
        transform: translate(-1px, -1px);
        box-shadow: 5px 5px 0 #050510;
    }

    /* Quest Card Header */
    .quest-card-header {
        display: flex;
        gap: 12px;
        align-items: start;
        margin-bottom: 16px;
        padding-bottom: 12px;
        border-bottom: 2px solid #1e3a5f;
    }

    .quest-icon {
        font-size: 24px;
        flex-shrink: 0;
        width: 32px;
        height: 32px;
        display: flex;
        align-items: center;
        justify-content: center;
        filter: drop-shadow(2px 2px 0 #000);
    }

    .quest-info {
        flex: 1;
        min-width: 0;
    }

    .quest-title {
        font-family: 'Press Start 2P', 'Courier New', monospace;
        font-size: 10px;
        color: #e2e8f0;
        margin-bottom: 6px;
        letter-spacing: 0.5px;
        line-height: 1.4;
    }

    .quest-concept {
        font-family: 'Press Start 2P', 'Courier New', monospace;
        font-size: 8px;
        color: #94a3b8;
        line-height: 1.5;
    }

    /* Quest Status Bar */
    .quest-status-bar {
        display: flex;
        justify-content: space-between;
        align-items: center;
        gap: 12px;
    }

    .quest-status {
        font-family: 'Press Start 2P', 'Courier New', monospace;
        font-size: 8px;
        padding: 6px 10px;
        background: #1e3a5f;
        border: 2px solid #3a506b;
        color: #cbd5e1;
        letter-spacing: 1px;
        flex: 1;
        text-align: center;
    }

    .quest-status.completed {
        background: #14532d;
        border-color: #22c55e;
        color: #dcfce7;
    }

    .quest-status.locked {
        background: #451a03;
        border-color: #92400e;
        color: #fbbf24;
    }

    /* Quest Button */
    .quest-button {
        font-size: 8px;
        padding: 8px 16px;
        flex-shrink: 0;
    }

    .quest-button:disabled {
        background: linear-gradient(180deg, #1e293b 0%, #0f172a 100%);
        border-color: #334155;
        color: #475569;
        cursor: not-allowed;
        box-shadow: 2px 2px 0 #0a0a1e;
    }

    .quest-button:disabled:hover {
        transform: none;
    }
</style>
