<script lang="ts">
    import type { LevelInfo, PlayerProgress } from '$lib/types';
    import { LEVEL_POSITIONS, MAP_CONFIG } from '$lib/config/worldmap';
    import LevelMarker from './LevelMarker.svelte';

    interface Props {
        levels: LevelInfo[];
        progress: PlayerProgress | null;
        onSelectLevel: (levelId: string) => void;
        onSettings?: () => void;
    }

    let { levels, progress, onSelectLevel, onSettings }: Props = $props();

    // Track if background image exists
    let imageError = $state(false);

    // Calculate completion stats
    let completedCount = $derived(progress?.completed_levels?.length ?? 0);
    let totalXP = $derived(progress?.total_xp ?? 0);

    // Map rendering logic for responsive aspect ratio
    let mapAreaWidth = $state(0);
    let mapAreaHeight = $state(0);
    const MAP_WIDTH = 1408;
    const MAP_HEIGHT = 768;
    const MAP_RATIO = MAP_WIDTH / MAP_HEIGHT;

    // Determine if we are constrained by width or height to maintain aspect ratio
    let fitMode = $derived((mapAreaWidth / mapAreaHeight) > MAP_RATIO ? 'height' : 'width');

    function handleLevelClick(level: LevelInfo) {
        if (!level.locked) {
            onSelectLevel(level.id);
        }
    }
</script>

<div class="world-map-container">
    <!-- Header -->
    <header class="map-header">
        <h1 class="map-title">{MAP_CONFIG.title}</h1>
        <p class="map-subtitle">{MAP_CONFIG.subtitle}</p>
    </header>

    <!-- Map Area -->
    <div 
        class="map-area"
        bind:clientWidth={mapAreaWidth}
        bind:clientHeight={mapAreaHeight}
    >
        <!-- Aspect Ratio Wrapper -->
        <div 
            class="map-content-wrapper"
            style:width={fitMode === 'width' ? '100%' : 'auto'}
            style:height={fitMode === 'height' ? '100%' : 'auto'}
            style:aspect-ratio="{MAP_RATIO}"
        >
            <!-- Background: try image first, fall back to CSS -->
            {#if !imageError}
                <img
                    src={MAP_CONFIG.backgroundImage}
                    alt="World Map"
                    class="map-background-image"
                    onerror={() => imageError = true}
                />
            {/if}

            <!-- CSS fallback background -->
            <div class="map-background-css" class:visible={imageError}>
                <div class="island-base"></div>
                <div class="mountains"></div>
                <div class="water-effect"></div>
            </div>

            <!-- Level Markers -->
            <div class="markers-container">
                {#each levels as level (level.id)}
                    {#if LEVEL_POSITIONS[level.id]}
                        <LevelMarker
                            {level}
                            position={LEVEL_POSITIONS[level.id]}
                            isCurrentLevel={progress?.current_level === level.id}
                            onClick={() => handleLevelClick(level)}
                        />
                    {/if}
                {/each}
            </div>
        </div>
    </div>

    <!-- Footer -->
    <footer class="map-footer">
        <div class="progress-info">
            <div class="progress-stat">
                <span class="stat-icon">&#9876;</span>
                <span class="stat-label">Completed</span>
                <span class="stat-value">{completedCount} / {levels.length}</span>
            </div>
            <div class="progress-stat">
                <span class="stat-icon">&#9830;</span>
                <span class="stat-label">Total XP</span>
                <span class="stat-value">{totalXP}</span>
            </div>
        </div>
        {#if onSettings}
            <button class="settings-btn" onclick={onSettings} title="Settings">
                &#9881;
            </button>
        {/if}
    </footer>
</div>

<style>
    .world-map-container {
        position: fixed;
        inset: 0;
        display: flex;
        flex-direction: column;
        background: linear-gradient(180deg, #0a0a14 0%, #1a1a2e 100%);
        font-family: 'Press Start 2P', monospace;
        image-rendering: pixelated;
    }

    /* Header */
    .map-header {
        padding: 20px 24px;
        text-align: center;
        background: linear-gradient(180deg, #0a0a14 0%, transparent 100%);
        border-bottom: 4px solid #0f3460;
        z-index: 10;
    }

    .map-title {
        font-size: 20px;
        color: #fbbf24;
        text-shadow: 3px 3px 0 #92400e, 0 0 20px rgba(251, 191, 36, 0.3);
        margin: 0 0 8px 0;
        letter-spacing: 3px;
    }

    .map-subtitle {
        font-size: 9px;
        color: #a1a1aa; /* A3: Higher contrast for small text */
        margin: 0;
        letter-spacing: 2px;
    }

    /* Map Area */
    .map-area {
        flex: 1;
        position: relative;
        margin: 16px;
        border: 4px solid #3a506b;
        border-top-color: #5a7090;
        border-left-color: #5a7090;
        box-shadow: 8px 8px 0 #050510;
        overflow: hidden;
        background: #0a2540;
        /* Center the aspect-ratio wrapper */
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .map-content-wrapper {
        position: relative;
        max-width: 100%;
        max-height: 100%;
    }

    .map-background-image {
        position: absolute;
        inset: 0;
        width: 100%;
        height: 100%;
        /* We don't need cover anymore, the wrapper enforces ratio */
        image-rendering: pixelated;
    }

    /* CSS Fallback Background */
    .map-background-css {
        position: absolute;
        inset: 0;
        opacity: 0;
        transition: opacity 0.3s;
    }

    .map-background-css.visible {
        opacity: 1;
    }

    .island-base {
        position: absolute;
        left: 8%;
        right: 8%;
        top: 12%;
        bottom: 8%;
        background: linear-gradient(
            135deg,
            #3d6b3d 0%,
            #2d5a2d 40%,
            #1d4a1d 100%
        );
        clip-path: polygon(
            10% 85%, 3% 55%, 5% 25%, 20% 10%,
            50% 3%, 80% 10%, 95% 35%, 97% 65%,
            90% 85%, 70% 95%, 30% 95%, 10% 85%
        );
        box-shadow: inset 0 -30px 60px rgba(0,0,0,0.4);
    }

    .mountains {
        position: absolute;
        left: 40%;
        top: 8%;
        width: 35%;
        height: 30%;
        background: linear-gradient(
            180deg,
            #9ca3af 0%,
            #6b7280 40%,
            #4b5563 100%
        );
        clip-path: polygon(
            0% 100%, 15% 40%, 25% 70%, 35% 20%,
            50% 60%, 60% 10%, 75% 50%, 85% 25%, 100% 100%
        );
    }

    .water-effect {
        position: absolute;
        inset: 0;
        background: radial-gradient(
            ellipse at 50% 50%,
            transparent 40%,
            #0a2540 70%
        );
        pointer-events: none;
    }

    /* Markers Container */
    .markers-container {
        position: absolute;
        inset: 0;
    }

    /* Footer */
    .map-footer {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 12px 24px;
        background: linear-gradient(180deg, transparent 0%, #0a0a14 100%);
        border-top: 4px solid #0f3460;
        z-index: 10;
    }

    .progress-info {
        display: flex;
        gap: 24px;
    }

    .progress-stat {
        display: flex;
        align-items: center;
        gap: 8px;
        padding: 8px 16px;
        background: linear-gradient(180deg, #1a1a2e 0%, #16213e 100%);
        border: 3px solid #0f3460;
        border-top-color: #3a506b;
        border-left-color: #3a506b;
        box-shadow: 3px 3px 0 #0a0a1e;
    }

    .stat-icon {
        font-size: 14px;
        color: #fbbf24;
    }

    .stat-label {
        font-size: 7px;
        color: #a1a1aa; /* A3: Higher contrast for small text */
        text-transform: uppercase;
    }

    .stat-value {
        font-size: 10px;
        color: #e2e8f0;
    }

    .settings-btn {
        background: linear-gradient(180deg, #1a1a2e 0%, #16213e 100%);
        border: 3px solid #3a506b;
        border-top-color: #5a7090;
        border-left-color: #5a7090;
        box-shadow: 3px 3px 0 #0a0a1e;
        color: #e2e8f0;
        width: 44px;
        height: 44px;
        font-size: 20px;
        cursor: pointer;
        transition: all 0.2s;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .settings-btn:hover {
        border-color: #00fff5;
        color: #00fff5;
        box-shadow: 0 0 10px rgba(0, 255, 245, 0.3);
    }

    /* V5: Focus visible for keyboard users */
    .settings-btn:focus-visible {
        outline: 2px solid var(--color-accent-cyan, #67e8f9);
        outline-offset: 2px;
    }

    /* Responsive adjustments */
    @media (max-width: 768px) {
        .map-title {
            font-size: 14px;
        }

        .map-subtitle {
            font-size: 7px;
        }

        .map-area {
            margin: 8px;
        }

        .progress-info {
            flex-direction: column;
            gap: 8px;
        }

        .progress-stat {
            padding: 6px 12px;
        }
    }
</style>
