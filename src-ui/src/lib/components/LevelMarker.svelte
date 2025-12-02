<script lang="ts">
    import type { LevelInfo } from '$lib/types';
    import type { LevelPosition } from '$lib/config/worldmap';

    interface Props {
        level: LevelInfo;
        position: LevelPosition;
        isCurrentLevel: boolean;
        currentXP: number;
        onClick: () => void;
    }

    let { level, position, isCurrentLevel, currentXP, onClick }: Props = $props();

    // Calculate XP needed to unlock this level
    let xpNeeded = $derived(Math.max(0, level.xp_required - currentXP));

    // Determine visual state (now includes 'in_progress' for partial quest completion)
    let state = $derived.by(() => {
        if (level.locked) return 'locked';
        if (level.completed) return 'completed';
        if (isCurrentLevel) return 'current';
        // Show "in_progress" if some quests completed but not all
        if (level.total_quests > 0 && level.completed_quests > 0) return 'in_progress';
        return 'available';
    });

    // Extract level number from ID (e.g., "L01" -> "01")
    let levelNum = $derived(level.id.replace('L', ''));

    // Progress ring calculations
    let hasQuests = $derived(level.total_quests > 0);
    let progressPercent = $derived(level.completion_percentage || 0);

    // SVG circle calculations for progress ring
    const ringRadius = 18;
    const ringCircumference = 2 * Math.PI * ringRadius;
    let progressOffset = $derived(ringCircumference - (progressPercent / 100) * ringCircumference);

    // Color gradient based on completion (blue → cyan → gold)
    let progressColor = $derived.by(() => {
        const pct = progressPercent / 100;
        if (pct === 0) return '#3b82f6'; // blue
        if (pct < 0.5) {
            // blue to cyan
            return `rgb(${Math.round(59 + (6 - 59) * pct * 2)}, ${Math.round(130 + (182 - 130) * pct * 2)}, ${Math.round(246 + (212 - 246) * pct * 2)})`;
        }
        if (pct < 1) {
            // cyan to gold
            const t = (pct - 0.5) * 2;
            return `rgb(${Math.round(6 + (251 - 6) * t)}, ${Math.round(182 + (191 - 182) * t)}, ${Math.round(212 + (36 - 212) * t)})`;
        }
        return '#fbbf24'; // gold
    });
</script>

<button
    class="level-marker {state}"
    style="left: {position.x}%; top: {position.y}%"
    onclick={onClick}
    disabled={level.locked}
    title={level.locked ? `Need ${xpNeeded} more XP to unlock` : level.title}
>
    <!-- Progress ring at flag base (only for multi-quest levels) -->
    {#if hasQuests && !level.locked}
        <svg class="progress-ring" viewBox="0 0 44 44">
            <!-- Background circle -->
            <circle
                cx="22"
                cy="22"
                r={ringRadius}
                fill="none"
                stroke="rgba(0,0,0,0.4)"
                stroke-width="4"
            />
            <!-- Progress arc -->
            <circle
                cx="22"
                cy="22"
                r={ringRadius}
                fill="none"
                stroke={progressColor}
                stroke-width="4"
                stroke-linecap="round"
                stroke-dasharray={ringCircumference}
                stroke-dashoffset={progressOffset}
                transform="rotate(-90 22 22)"
                class="progress-arc"
            />
        </svg>
    {/if}

    <!-- Flag pole -->
    <div class="flag-pole"></div>

    <!-- Flag banner -->
    <div class="flag-banner">
        <span class="level-num">{levelNum}</span>
    </div>

    <!-- Tooltip on hover (I3: Added progress ring explanation) -->
    <div class="tooltip">
        <strong class="tooltip-title">{level.title}</strong>
        <span class="tooltip-concept">{level.concept}</span>
        {#if hasQuests}
            <span class="tooltip-quests">{level.completed_quests}/{level.total_quests} quests completed</span>
            {#if progressPercent > 0 && progressPercent < 100}
                <span class="tooltip-progress">Ring shows {Math.round(progressPercent)}% progress</span>
            {/if}
        {/if}
        {#if level.locked}
            <span class="tooltip-locked">Need {xpNeeded} XP to unlock</span>
        {:else if level.completed}
            <span class="tooltip-completed">+{level.xp_reward} XP earned</span>
        {:else}
            <span class="tooltip-xp">{level.xp_reward} XP</span>
        {/if}
    </div>
</button>

<style>
    .level-marker {
        position: absolute;
        transform: translate(-50%, -100%);
        cursor: pointer;
        transition: transform 0.2s, filter 0.2s;
        z-index: 10;
        background: none;
        border: none;
        padding: 0;
        font-family: 'Press Start 2P', monospace;
        filter: drop-shadow(0 2px 4px rgba(0,0,0,0.6));
    }

    .level-marker:hover:not(.locked) {
        transform: translate(-50%, -100%) scale(1.2);
        z-index: 20;
        filter: drop-shadow(0 4px 8px rgba(0,0,0,0.8));
    }

    /* V5: Focus visible for keyboard users */
    .level-marker:focus-visible {
        outline: 3px solid var(--color-accent-cyan, #67e8f9);
        outline-offset: 4px;
    }

    .level-marker.locked {
        cursor: not-allowed;
        filter: drop-shadow(0 0 8px rgba(100, 116, 139, 0.6)) drop-shadow(0 3px 6px rgba(0,0,0,0.8));
    }

    .level-marker.available {
        filter: drop-shadow(0 0 12px rgba(59, 130, 246, 0.8)) drop-shadow(0 0 20px rgba(59, 130, 246, 0.4)) drop-shadow(0 3px 6px rgba(0,0,0,0.8));
    }

    .level-marker.current {
        filter: drop-shadow(0 0 16px rgba(239, 68, 68, 0.9)) drop-shadow(0 0 24px rgba(239, 68, 68, 0.5)) drop-shadow(0 3px 6px rgba(0,0,0,0.8));
    }

    .level-marker.completed {
        filter: drop-shadow(0 0 12px rgba(34, 197, 94, 0.8)) drop-shadow(0 0 20px rgba(34, 197, 94, 0.4)) drop-shadow(0 3px 6px rgba(0,0,0,0.8));
    }

    .level-marker.in_progress {
        filter: drop-shadow(0 0 14px rgba(6, 182, 212, 0.8)) drop-shadow(0 0 22px rgba(6, 182, 212, 0.4)) drop-shadow(0 3px 6px rgba(0,0,0,0.8));
    }

    /* Progress ring at flag base */
    .progress-ring {
        position: absolute;
        bottom: -8px;
        left: 50%;
        transform: translateX(-50%);
        width: 48px;
        height: 48px;
        z-index: -1;
        filter: drop-shadow(0 0 6px rgba(0,0,0,0.6));
    }

    .progress-arc {
        transition: stroke-dashoffset 0.5s ease, stroke 0.3s ease;
    }

    /* Flag pole */
    .flag-pole {
        width: 6px;
        height: 52px;
        background: linear-gradient(90deg, #3d2817 0%, #8b5a2b 50%, #3d2817 100%);
        margin: 0 auto;
        border-radius: 2px;
        box-shadow: 2px 2px 0 rgba(0,0,0,0.7), -1px 0 0 rgba(255,255,255,0.3);
    }

    /* Flag banner - larger and more visible */
    .flag-banner {
        position: absolute;
        top: 2px;
        left: 9px;
        width: 42px;
        height: 30px;
        display: flex;
        align-items: center;
        justify-content: center;
        clip-path: polygon(0 0, 100% 0, 82% 50%, 100% 100%, 0 100%);
        animation: flag-wave 3s ease-in-out infinite;
        box-shadow: 2px 2px 4px rgba(0,0,0,0.5);
        border: 2px solid rgba(255,255,255,0.5);
        border-right: none;
    }

    .level-num {
        font-size: 11px;
        font-weight: bold;
        text-shadow: 2px 2px 0 rgba(0,0,0,0.9), -1px -1px 0 rgba(0,0,0,0.6);
    }

    /* Flag colors by state - brighter for visibility */
    .locked .flag-banner {
        background: linear-gradient(180deg, #94a3b8 0%, #64748b 100%);
        border-color: rgba(255,255,255,0.4);
    }
    .locked .level-num {
        color: #1e293b;
    }

    .available .flag-banner {
        background: linear-gradient(180deg, #60a5fa 0%, #3b82f6 100%);
        border-color: rgba(147, 197, 253, 0.6);
    }
    .available .level-num {
        color: #ffffff;
    }

    .current .flag-banner {
        background: linear-gradient(180deg, #f87171 0%, #ef4444 100%);
        border-color: rgba(254, 202, 202, 0.7);
        animation: flag-wave 2s ease-in-out infinite, pulse-glow 1.5s ease-in-out infinite;
    }
    .current .level-num {
        color: #ffffff;
    }

    .completed .flag-banner {
        background: linear-gradient(180deg, #4ade80 0%, #22c55e 100%);
        border-color: rgba(74, 222, 128, 0.7);
    }
    .completed .level-num {
        color: #052e16;
    }

    .in_progress .flag-banner {
        background: linear-gradient(180deg, #22d3ee 0%, #06b6d4 100%);
        border-color: rgba(103, 232, 249, 0.6);
        animation: flag-wave 2.5s ease-in-out infinite, shimmer 2s linear infinite;
    }
    .in_progress .level-num {
        color: #ffffff;
    }

    @keyframes shimmer {
        0% { filter: brightness(1); }
        50% { filter: brightness(1.15); }
        100% { filter: brightness(1); }
    }

    @keyframes flag-wave {
        0%, 100% { transform: skewX(0deg) scaleX(1); }
        50% { transform: skewX(-2deg) scaleX(0.98); }
    }

    @keyframes pulse-glow {
        0%, 100% { filter: drop-shadow(0 0 4px #ef4444); }
        50% { filter: drop-shadow(0 0 10px #ef4444); }
    }

    /* Tooltip */
    .tooltip {
        position: absolute;
        bottom: calc(100% + 8px);
        left: 50%;
        transform: translateX(-50%);
        background: linear-gradient(180deg, #1a1a2e 0%, #16213e 100%);
        border: 3px solid #3a506b;
        border-top-color: #5a7090;
        border-left-color: #5a7090;
        box-shadow: 4px 4px 0 #0a0a1e;
        padding: 8px 12px;
        white-space: nowrap;
        opacity: 0;
        pointer-events: none;
        transition: opacity 0.2s;
        z-index: 30;
        display: flex;
        flex-direction: column;
        gap: 4px;
    }

    .level-marker:hover .tooltip {
        opacity: 1;
    }

    .tooltip-title {
        color: #fbbf24;
        font-size: 8px;
        text-shadow: 1px 1px 0 #92400e;
    }

    .tooltip-concept {
        color: #a1a1aa; /* A3: Higher contrast for small text */
        font-size: 7px;
        text-transform: uppercase;
    }

    /* I3: Progress ring explanation */
    .tooltip-progress {
        color: #67e8f9;
        font-size: 6px;
        font-style: italic;
    }

    .tooltip-locked {
        color: #ef4444;
        font-size: 7px;
    }

    .tooltip-completed {
        color: #22c55e;
        font-size: 7px;
    }

    .tooltip-xp {
        color: #fbbf24;
        font-size: 7px;
    }

    .tooltip-quests {
        color: #06b6d4;
        font-size: 7px;
        font-weight: bold;
    }
</style>
