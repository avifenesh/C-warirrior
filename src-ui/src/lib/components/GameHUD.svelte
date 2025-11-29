<script lang="ts">
    import type { Player } from '$lib/types';

    interface Props {
        player: Player | null;
        currentLevelId: string | null;
        onBackToMap?: () => void;
    }

    let { player, currentLevelId, onBackToMap }: Props = $props();

    // Calculate health percentage
    let healthPercent = $derived(
        player ? Math.max(0, Math.min(100, (player.health / player.max_health) * 100)) : 0
    );

    // Calculate XP percentage (100 XP per level)
    let xpPercent = $derived(player ? (player.xp % 100) : 0);

    // Get health hearts (max 5 hearts)
    let hearts = $derived.by(() => {
        if (!player) return [];
        const maxHearts = 5;
        const healthPerHeart = player.max_health / maxHearts;
        const result = [];
        for (let i = 0; i < maxHearts; i++) {
            const heartHealth = Math.max(0, Math.min(healthPerHeart, player.health - i * healthPerHeart));
            if (heartHealth >= healthPerHeart) {
                result.push('full');
            } else if (heartHealth > 0) {
                result.push('half');
            } else {
                result.push('empty');
            }
        }
        return result;
    });

    // Get XP gem count (show as coins/gems)
    let xpDisplay = $derived(player ? player.xp : 0);
</script>

<!-- Pixel Art HUD Container -->
<div class="pointer-events-none fixed left-4 top-4 z-40 space-y-2 pixel-font">
    <!-- Level Badge -->
    <div class="pixel-panel flex items-center gap-2">
        <div class="pixel-icon scroll">
            <span class="text-amber-900">!</span>
        </div>
        <div>
            <p class="text-[10px] uppercase text-amber-200/70">Quest</p>
            <p class="text-sm font-bold text-amber-100">{currentLevelId ?? '---'}</p>
        </div>
    </div>

    {#if player}
        <!-- Health Bar with Hearts -->
        <div class="pixel-panel">
            <div class="flex items-center gap-2 mb-1">
                <div class="pixel-icon heart">
                    <span class="text-rose-600 drop-shadow">&#9829;</span>
                </div>
                <div class="flex gap-1">
                    {#each hearts as heart}
                        <div class="w-4 h-4 flex items-center justify-center">
                            {#if heart === 'full'}
                                <span class="text-rose-500 text-sm drop-shadow-[0_1px_0_rgba(0,0,0,0.8)]">&#9829;</span>
                            {:else if heart === 'half'}
                                <span class="text-rose-400/60 text-sm">&#9829;</span>
                            {:else}
                                <span class="text-slate-700 text-sm">&#9829;</span>
                            {/if}
                        </div>
                    {/each}
                </div>
                <span class="text-[10px] text-slate-400 ml-auto">{player.health}/{player.max_health}</span>
            </div>
            <!-- Pixel health bar -->
            <div class="pixel-bar-bg">
                <div
                    class="pixel-bar-fill health"
                    style="width: {healthPercent}%"
                ></div>
            </div>
        </div>

        <!-- XP/Gold Display -->
        <div class="pixel-panel">
            <div class="flex items-center gap-2 mb-1">
                <div class="pixel-icon gem">
                    <span class="text-amber-400 drop-shadow">&#9830;</span>
                </div>
                <span class="text-[10px] uppercase text-amber-200/70">Experience</span>
                <span class="text-xs font-bold text-amber-300 ml-auto">{xpDisplay} XP</span>
            </div>
            <!-- Pixel XP bar -->
            <div class="pixel-bar-bg">
                <div
                    class="pixel-bar-fill xp"
                    style="width: {xpPercent}%"
                ></div>
            </div>
            <div class="flex justify-between text-[9px] text-slate-500 mt-1">
                <span>Lv.{player.level}</span>
                <span>{player.xp % 100}/100</span>
            </div>
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
    /* Pixel art font styling */
    .pixel-font {
        font-family: 'Press Start 2P', 'Courier New', monospace;
        image-rendering: pixelated;
        -webkit-font-smoothing: none;
        -moz-osx-font-smoothing: grayscale;
    }

    /* Pixel panel - main container */
    .pixel-panel {
        background: linear-gradient(180deg, #1a1a2e 0%, #16213e 100%);
        border: 3px solid #0f3460;
        border-top-color: #3a506b;
        border-left-color: #3a506b;
        box-shadow:
            inset 0 0 0 1px #0a0a1e,
            4px 4px 0 #0a0a1e;
        padding: 8px 12px;
        min-width: 160px;
    }

    .pixel-panel-small {
        background: linear-gradient(180deg, #1a1a2e 0%, #16213e 100%);
        border: 2px solid #0f3460;
        border-top-color: #3a506b;
        border-left-color: #3a506b;
        box-shadow: 2px 2px 0 #0a0a1e;
        padding: 6px 10px;
    }

    /* Pixel icons */
    .pixel-icon {
        width: 20px;
        height: 20px;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 14px;
    }

    /* Pixel bar backgrounds */
    .pixel-bar-bg {
        height: 8px;
        background: #0a0a1e;
        border: 2px solid #0f3460;
        border-top-color: #050510;
        border-left-color: #050510;
        position: relative;
        overflow: hidden;
    }

    /* Pixel bar fills */
    .pixel-bar-fill {
        height: 100%;
        transition: width 0.3s ease;
        image-rendering: pixelated;
    }

    .pixel-bar-fill.health {
        background: linear-gradient(180deg,
            #dc2626 0%,
            #ef4444 40%,
            #b91c1c 60%,
            #991b1b 100%
        );
        box-shadow: inset 0 -1px 0 #7f1d1d;
    }

    .pixel-bar-fill.xp {
        background: linear-gradient(180deg,
            #fbbf24 0%,
            #fcd34d 40%,
            #d97706 60%,
            #b45309 100%
        );
        box-shadow: inset 0 -1px 0 #92400e;
    }

    /* Pixel keyboard keys */
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

    /* Drop shadow for depth */
    .drop-shadow {
        filter: drop-shadow(1px 1px 0 rgba(0, 0, 0, 0.8));
    }

    /* Map button */
    .map-btn {
        background: linear-gradient(180deg, #1a1a2e 0%, #16213e 100%);
        border: 3px solid #3a506b;
        border-top-color: #5a7090;
        border-left-color: #5a7090;
        box-shadow: 4px 4px 0 #0a0a1e;
        padding: 10px 16px;
        font-family: 'Press Start 2P', monospace;
        font-size: 9px;
        color: #e2e8f0;
        cursor: pointer;
        transition: all 0.15s;
        text-transform: uppercase;
        letter-spacing: 1px;
    }

    .map-btn:hover {
        border-color: #00fff5;
        color: #00fff5;
        box-shadow: 0 0 12px rgba(0, 255, 245, 0.3), 4px 4px 0 #0a0a1e;
    }

    .map-btn:active {
        transform: translate(2px, 2px);
        box-shadow: 2px 2px 0 #0a0a1e;
    }
</style>
