<script lang="ts">
    import type { Player } from '$lib/stores/game.svelte';

    interface Props {
        player: Player | null;
        currentLevelId: string | null;
    }

    let { player, currentLevelId }: Props = $props();

    // Calculate health percentage
    let healthPercent = $derived(
        player ? Math.max(0, Math.min(100, (player.health / player.max_health) * 100)) : 0
    );

    // Calculate XP percentage (100 XP per level)
    let xpPercent = $derived(player ? (player.xp % 100) : 0);

    // Get health bar color based on percentage
    function getHealthColor(percent: number): string {
        if (percent > 66) return 'bg-emerald-500';
        if (percent > 33) return 'bg-amber-500';
        return 'bg-rose-500';
    }

    function getHealthGlow(percent: number): string {
        if (percent > 66) return 'shadow-emerald-500/50';
        if (percent > 33) return 'shadow-amber-500/50';
        return 'shadow-rose-500/50';
    }
</script>

<div class="pointer-events-none fixed left-4 top-4 z-40 space-y-3">
    <!-- Level Indicator -->
    <div
        class="rounded-lg border border-cyan-500/30 bg-slate-900/80 px-4 py-2 backdrop-blur-sm shadow-lg shadow-cyan-500/10"
    >
        <p class="text-xs uppercase tracking-widest text-cyan-400">Level</p>
        <p class="text-2xl font-bold text-cyan-200">{currentLevelId ?? '--'}</p>
    </div>

    {#if player}
        <!-- Health Bar -->
        <div
            class="rounded-lg border border-slate-700/50 bg-slate-900/80 p-3 backdrop-blur-sm shadow-lg"
        >
            <div class="mb-2 flex items-center justify-between">
                <p class="text-xs uppercase tracking-widest text-slate-400">Health</p>
                <p class="text-xs font-mono text-slate-300">{player.health}/{player.max_health}</p>
            </div>
            <div class="h-3 w-48 overflow-hidden rounded-full bg-slate-800 border border-slate-700">
                <div
                    class="h-full transition-all duration-300 shadow-lg {getHealthColor(
                        healthPercent
                    )} {getHealthGlow(healthPercent)}"
                    style="width: {healthPercent}%"
                ></div>
            </div>
        </div>

        <!-- XP Bar -->
        <div
            class="rounded-lg border border-slate-700/50 bg-slate-900/80 p-3 backdrop-blur-sm shadow-lg"
        >
            <div class="mb-2 flex items-center justify-between">
                <p class="text-xs uppercase tracking-widest text-slate-400">
                    XP
                    <span class="ml-1 text-cyan-400">Lv.{player.level}</span>
                </p>
                <p class="text-xs font-mono text-slate-300">{xpPercent}/100</p>
            </div>
            <div class="h-3 w-48 overflow-hidden rounded-full bg-slate-800 border border-slate-700">
                <div
                    class="h-full bg-gradient-to-r from-cyan-500 to-blue-500 transition-all duration-300 shadow-lg shadow-cyan-500/50"
                    style="width: {xpPercent}%"
                ></div>
            </div>
        </div>
    {/if}

    <!-- Scan line overlay effect -->
    <div class="absolute inset-0 pointer-events-none overflow-hidden rounded-lg">
        <div class="hud-scan-line"></div>
    </div>
</div>

<style>
    @keyframes hud-scan {
        from {
            transform: translateY(-100%);
        }
        to {
            transform: translateY(300%);
        }
    }

    .hud-scan-line {
        position: absolute;
        width: 100%;
        height: 100px;
        background: linear-gradient(
            to bottom,
            transparent,
            rgba(34, 211, 238, 0.02),
            transparent
        );
        animation: hud-scan 4s linear infinite;
    }
</style>
