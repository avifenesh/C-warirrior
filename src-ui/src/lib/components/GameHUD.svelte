<script lang="ts">
    import type { Player } from '$lib/types';

    interface Props {
        player: Player | null;
        currentLevelId: string | null;
        onBackToMap?: () => void;
        /** Whether code has been modified (for confirmation dialog) */
        hasUnsavedCode?: boolean;
    }

    let { player, currentLevelId, onBackToMap, hasUnsavedCode = false }: Props = $props();

    let xpDisplay = $derived(player ? player.xp : 0);
    let levelDisplay = $derived(player ? player.level : 1);

    // U5: Confirmation dialog state
    let showConfirmDialog = $state(false);

    function handleMapClick() {
        if (hasUnsavedCode) {
            showConfirmDialog = true;
        } else {
            onBackToMap?.();
        }
    }

    function confirmLeave() {
        showConfirmDialog = false;
        onBackToMap?.();
    }

    function cancelLeave() {
        showConfirmDialog = false;
    }
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
        onclick={handleMapClick}
        title="Return to World Map"
    >
        MAP
    </button>
{/if}

<!-- U5: Confirmation Dialog -->
{#if showConfirmDialog}
    <div class="pointer-events-auto fixed inset-0 z-50 flex items-center justify-center bg-slate-950/90">
        <div class="confirm-dialog" role="alertdialog" aria-modal="true" aria-labelledby="confirm-title">
            <h3 id="confirm-title" class="confirm-title">Leave Quest?</h3>
            <p class="confirm-message">You have unsaved code changes. Are you sure you want to leave?</p>
            <div class="confirm-actions">
                <button class="confirm-btn-cancel" onclick={cancelLeave}>
                    Stay
                </button>
                <button class="confirm-btn-leave" onclick={confirmLeave}>
                    Leave
                </button>
            </div>
        </div>
    </div>
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

    /* V5: Focus visible for keyboard users */
    .map-btn:focus-visible {
        outline: 2px solid var(--color-accent-cyan, #67e8f9);
        outline-offset: 2px;
    }

    /* U5: Confirmation Dialog Styles */
    .confirm-dialog {
        background: linear-gradient(180deg, #1a1a2e 0%, #16213e 100%);
        border: 4px solid #3a506b;
        border-top-color: #5a7090;
        border-left-color: #5a7090;
        box-shadow: 8px 8px 0 #0a0a1e;
        padding: 20px 24px;
        min-width: 280px;
        max-width: 360px;
        font-family: 'Press Start 2P', monospace;
        text-align: center;
    }

    .confirm-title {
        font-size: 12px;
        color: #fbbf24;
        text-shadow: 2px 2px 0 #92400e;
        margin: 0 0 12px 0;
    }

    .confirm-message {
        font-size: 8px;
        color: #e2e8f0;
        line-height: 1.6;
        margin: 0 0 20px 0;
    }

    .confirm-actions {
        display: flex;
        gap: 12px;
        justify-content: center;
    }

    .confirm-btn-cancel {
        background: linear-gradient(180deg, #334155 0%, #1e293b 100%);
        border: 3px solid #64748b;
        border-bottom-color: #334155;
        border-right-color: #334155;
        box-shadow: 3px 3px 0 #0a0a1e;
        padding: 10px 20px;
        font-family: 'Press Start 2P', monospace;
        font-size: 8px;
        color: #e2e8f0;
        cursor: pointer;
        transition: transform 0.1s;
    }

    .confirm-btn-cancel:hover {
        transform: translate(2px, 2px);
        box-shadow: 1px 1px 0 #0a0a1e;
    }

    .confirm-btn-cancel:focus-visible {
        outline: 2px solid var(--color-accent-cyan, #67e8f9);
        outline-offset: 2px;
    }

    .confirm-btn-leave {
        background: linear-gradient(180deg, #dc2626 0%, #b91c1c 100%);
        border: 3px solid #ef4444;
        border-bottom-color: #b91c1c;
        border-right-color: #b91c1c;
        box-shadow: 3px 3px 0 #0a0a1e;
        padding: 10px 20px;
        font-family: 'Press Start 2P', monospace;
        font-size: 8px;
        color: #fee2e2;
        cursor: pointer;
        transition: transform 0.1s;
    }

    .confirm-btn-leave:hover {
        transform: translate(2px, 2px);
        box-shadow: 1px 1px 0 #0a0a1e;
    }

    .confirm-btn-leave:focus-visible {
        outline: 2px solid var(--color-accent-cyan, #67e8f9);
        outline-offset: 2px;
    }
</style>
