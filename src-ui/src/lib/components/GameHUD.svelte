<script lang="ts">
    import type { Player } from '$lib/types';

    interface Props {
        player: Player | null;
        currentLevelId: string | null;
        onBackToMap?: () => void;
    }

    let { player, currentLevelId, onBackToMap }: Props = $props();

    let xpDisplay = $derived(player ? player.xp : 0);
    let levelDisplay = $derived(player ? player.level : 1);
</script>

<!-- Minimal HUD - Top bar with quest and XP -->
<div class="pointer-events-none fixed top-3 left-3 z-40 flex items-center gap-2 pixel-font">
    <!-- Quest Badge -->
    <div class="hud-badge">
        <span class="badge-label">QUEST</span>
        <span class="badge-value">{currentLevelId ?? '---'}</span>
    </div>

    {#if player}
        <!-- XP Badge -->
        <div class="hud-badge xp">
            <span class="text-amber-400">&#9830;</span>
            <span class="badge-value">{xpDisplay}</span>
            <span class="badge-label">XP</span>
        </div>
    {/if}
</div>

<!-- Map Button (top right) -->
{#if onBackToMap}
    <button
        class="pointer-events-auto fixed top-4 right-4 z-40 map-btn"
        onclick={onBackToMap}
        title="Return to World Map"
    >
        MAP
    </button>
{/if}

<!-- Controls hint (bottom left) -->
<div class="pointer-events-none fixed left-4 bottom-4 z-40">
    <div class="pixel-panel-small flex gap-3 text-[10px] text-slate-400">
        <span><kbd class="pixel-key">WASD</kbd> Move</span>
        <span><kbd class="pixel-key">E</kbd> Interact</span>
    </div>
</div>

<style>
    .pixel-font {
        font-family: 'Press Start 2P', 'Courier New', monospace;
        image-rendering: pixelated;
    }

    /* Compact HUD badges */
    .hud-badge {
        display: flex;
        align-items: center;
        gap: 6px;
        background: rgba(10, 10, 30, 0.85);
        border: 2px solid #3a506b;
        padding: 6px 10px;
        font-size: 8px;
    }

    .hud-badge.xp {
        border-color: #92400e;
    }

    .badge-label {
        color: #64748b;
        font-size: 7px;
    }

    .badge-value {
        color: #e2e8f0;
        font-size: 10px;
    }

    /* Controls panel */
    .pixel-panel-small {
        background: rgba(10, 10, 30, 0.85);
        border: 2px solid #3a506b;
        padding: 6px 10px;
    }

    .pixel-key {
        display: inline-block;
        background: #1a1a2e;
        border: 2px solid #3a506b;
        border-bottom-color: #0f3460;
        border-right-color: #0f3460;
        padding: 2px 6px;
        font-family: inherit;
        font-size: 8px;
        color: #e2e8f0;
        margin: 0 2px;
    }

    /* Map button */
    .map-btn {
        background: rgba(10, 10, 30, 0.85);
        border: 2px solid #3a506b;
        padding: 8px 12px;
        font-family: 'Press Start 2P', monospace;
        font-size: 8px;
        color: #e2e8f0;
        cursor: pointer;
        transition: all 0.15s;
    }

    .map-btn:hover {
        border-color: #00fff5;
        color: #00fff5;
    }

    .map-btn:active {
        transform: translate(1px, 1px);
    }
</style>
