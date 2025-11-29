<script lang="ts">
    import { onMount, type Snippet } from 'svelte';
    import { createEventDispatcher } from 'svelte';
    import type { RenderState, Direction } from '$lib/types';
    import { type LoadedAssets } from '$lib/engine/assets';
    import { getGlobalAssets } from '$lib/engine/assets-cache';
    import { ANIMATIONS, startAnimation, getCurrentFrame, type AnimationState } from '$lib/engine/animation';
    import { GameRenderer } from '$lib/engine/renderer';
    import { Camera } from '$lib/engine/camera';
    import { ParticleSystem, emitXpSparkles, emitCodeSuccessBurst } from '$lib/engine/particles';

    interface Props {
        renderState: RenderState | null;
        tileSize?: number;
        children?: Snippet;
        codeSuccess?: boolean;
        xpGained?: number;
    }

    let { renderState = null, tileSize = 64, children, codeSuccess = false, xpGained = 0 }: Props = $props();

    const dispatcher = createEventDispatcher();
    let canvasRef = $state<HTMLCanvasElement | null>(null);
    let containerRef = $state<HTMLDivElement | null>(null);
    let renderer: GameRenderer | null = null;
    
    let assets = $state<LoadedAssets | null>(null);
    let assetsLoaded = $state(false);

    // Camera and particle systems
    let camera = $state<Camera>(new Camera({ smoothing: 0.15 }));
    let particles = $state<ParticleSystem>(new ParticleSystem(500));
    let lastFrameTime = $state(0);

    // Animation state
    let playerAnimState = $state<AnimationState | null>(null);
    let terminalAnimState = $state<AnimationState | null>(null);
    let lastPlayerPos = { x: 0, y: 0 }; // NOT reactive - just for comparison
    let lastCodeSuccess = false; // NOT reactive - track code success state

    // Change detection for render optimization
    let lastRenderState: RenderState | null = null;
    let stateChanged = true; // Track if state changed since last render

    // Backend uses 32px tile size for positions
    const BACKEND_TILE_SIZE = 32;

    /**
     * Cheap state comparison to detect meaningful changes.
     * Checks for changes that would require tile cache rebuild.
     */
    function hasStateChanged(prev: RenderState | null, next: RenderState | null): boolean {
        // State went from null to something or vice versa
        if (!prev && !next) return false;
        if (!prev || !next) return true;

        // Game phase changed
        if (prev.game_phase !== next.game_phase) return true;

        // Level changed
        if (prev.current_level_id !== next.current_level_id) return true;

        // Player moved
        if (
            prev.player.position.x !== next.player.position.x ||
            prev.player.position.y !== next.player.position.y
        )
            return true;

        // Player direction changed
        if (prev.player.facing !== next.player.facing) return true;

        // Viewport changed (scrolling)
        if (
            prev.viewport_offset.x !== next.viewport_offset.x ||
            prev.viewport_offset.y !== next.viewport_offset.y
        )
            return true;

        // Tile grid dimensions changed (new level or area)
        if (
            prev.visible_tiles.length !== next.visible_tiles.length ||
            (prev.visible_tiles[0]?.length ?? 0) !== (next.visible_tiles[0]?.length ?? 0)
        )
            return true;

        // Check for door state changes (walkable toggles when unlocked)
        // Only check doors since they're the main dynamic tile type
        for (let y = 0; y < next.visible_tiles.length; y++) {
            const prevRow = prev.visible_tiles[y];
            const nextRow = next.visible_tiles[y];
            if (!prevRow || !nextRow) continue;
            for (let x = 0; x < nextRow.length; x++) {
                const prevTile = prevRow[x];
                const nextTile = nextRow[x];
                if (!prevTile || !nextTile) continue;
                // Door walkability changed = door unlocked/locked
                if (nextTile.tile_type === 'door' && prevTile.walkable !== nextTile.walkable) {
                    return true;
                }
            }
        }

        return false;
    }

    // Compute nearTerminal as derived state to avoid infinite loop
    const nearTerminal = $derived.by(() => {
        if (!renderState?.visible_tiles || !renderState?.player) return false;
        const playerTile = {
            x: Math.floor(renderState.player.position.x / BACKEND_TILE_SIZE),
            y: Math.floor(renderState.player.position.y / BACKEND_TILE_SIZE),
        };
        // Check if any visible tile is a terminal within 1 manhattan distance
        for (let y = 0; y < renderState.visible_tiles.length; y++) {
            for (let x = 0; x < renderState.visible_tiles[y].length; x++) {
                const tile = renderState.visible_tiles[y][x];
                if (tile.tile_type === 'terminal') {
                    const tileWorldX = x + renderState.viewport_offset.x;
                    const tileWorldY = y + renderState.viewport_offset.y;
                    const manhattan = Math.abs(playerTile.x - tileWorldX) + Math.abs(playerTile.y - tileWorldY);
                    if (manhattan <= 1) return true;
                }
            }
        }
        return false;
    });

    // Keyboard handling
    function handleKeydown(event: KeyboardEvent) {
        // Don't capture events when typing in form elements (textarea, input)
        const target = event.target as HTMLElement;
        if (target.tagName === 'TEXTAREA' || target.tagName === 'INPUT' || target.isContentEditable) {
            return;
        }

        const key = event.key.toLowerCase();

        // Movement keys (WASD + Arrows)
        const movementMapping: Record<string, Direction> = {
            w: 'up',
            arrowup: 'up',
            a: 'left',
            arrowleft: 'left',
            s: 'down',
            arrowdown: 'down',
            d: 'right',
            arrowright: 'right',
        };

        if (movementMapping[key]) {
            event.preventDefault();
            event.stopPropagation();
            dispatcher('move', { direction: movementMapping[key] });
            return; // Exit early to avoid other handlers
        }

        // Interact key (E only - space can cause scroll issues)
        if (key === 'e') {
            event.preventDefault();
            event.stopPropagation();
            if (nearTerminal) {
                dispatcher('interact');
            }
            return;
        }
    }

    onMount(() => {
        window.addEventListener('keydown', handleKeydown);
        containerRef?.focus();

        // Load assets
        if (canvasRef) {
            renderer = new GameRenderer(canvasRef, { tileSize });
        }

        getGlobalAssets()
            .then((loadedAssets) => {
                assets = loadedAssets;
                assetsLoaded = true;
                renderer?.setAssets(loadedAssets);

                // Initialize animations
                playerAnimState = startAnimation(ANIMATIONS.playerIdle);
                terminalAnimState = startAnimation(ANIMATIONS.terminalGlow);
            })
            .catch((err) => {
                console.error('[GameWorld] Asset loading failed:', err);
                assetsLoaded = true;
            });

        // Animation and rendering loop
        let animationFrameId = 0;
        const animLoop = (currentTime: number) => {
            // Snapshot renderState at the beginning of the frame to prevent mid-frame changes
            const currentState = renderState;

            const deltaTime = lastFrameTime > 0 ? currentTime - lastFrameTime : 16.67;
            lastFrameTime = currentTime;

            // Detect state changes for render optimization
            stateChanged = hasStateChanged(lastRenderState, currentState);
            if (stateChanged) {
                lastRenderState = currentState;
            }

            // Track player movement and update animations (non-reactive)
            if (currentState) {
                const currentPos = currentState.player.position;
                const moved = currentPos.x !== lastPlayerPos.x || currentPos.y !== lastPlayerPos.y;

                if (moved && playerAnimState) {
                    playerAnimState = startAnimation(ANIMATIONS.playerWalk);
                } else if (!moved && playerAnimState && !playerAnimState.finished) {
                    playerAnimState = startAnimation(ANIMATIONS.playerIdle);
                }

                lastPlayerPos = { x: currentPos.x, y: currentPos.y };

                // Update camera to follow player
                camera.setTarget(currentPos.x, currentPos.y);
                camera.update(deltaTime, canvasRef?.width ?? 640, canvasRef?.height ?? 480);

                // Emit particles on code success
                if (codeSuccess && !lastCodeSuccess) {
                    emitCodeSuccessBurst(particles, currentPos.x, currentPos.y);
                    if (xpGained > 0) {
                        emitXpSparkles(particles, currentPos.x, currentPos.y, xpGained);
                    }
                    camera.shake(8, 0.92);
                }
                lastCodeSuccess = codeSuccess;
            }

            // Always update particles (they animate independently)
            particles.update(deltaTime);

            // Render scene - pass stateChanged hint for tile cache invalidation
            if (renderer && canvasRef) {
                try {
                    renderer.render(
                        currentState,
                        particles,
                        deltaTime,
                        currentTime,
                        { player: playerAnimState, terminal: terminalAnimState },
                        stateChanged // Hint: tiles may need redraw
                    );
                } catch (err) {
                    console.error('[GameWorld] Rendering error:', err);
                }
            }

            animationFrameId = requestAnimationFrame(animLoop);
        };
        animationFrameId = requestAnimationFrame(animLoop);

        return () => {
            window.removeEventListener('keydown', handleKeydown);
            cancelAnimationFrame(animationFrameId);
        };
    });


    function handleContainerClick(event: MouseEvent) {
        // Don't steal focus if clicking inside a modal/form element
        const target = event.target as HTMLElement;
        const isInModal = target.closest('.grimoire-container') ||
                          target.closest('textarea') ||
                          target.closest('input') ||
                          target.closest('button');
        if (!isInModal) {
            containerRef?.focus();
        }
    }

    function handleContainerKeydown(event: KeyboardEvent) {
        // Focus is handled by the container, actual key handling is in handleKeydown
        if (event.key === 'Enter' || event.key === ' ') {
            containerRef?.focus();
        }
    }
</script>

<!-- svelte-ignore a11y_no_noninteractive_tabindex -->
<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<div
    bind:this={containerRef}
    class="relative min-h-screen w-full bg-[#0a0a14] outline-none"
    tabindex="0"
    role="application"
    aria-label="Game world - use WASD or arrow keys to move, E to interact"
    onclick={handleContainerClick}
    onkeydown={handleContainerKeydown}
>
    <!-- Canvas container -->
    <div class="flex items-center justify-center min-h-screen p-2">
        <div class="game-frame">
            <canvas
                bind:this={canvasRef}
                width={tileSize * 20}
                height={tileSize * 15}
                class="game-canvas"
                aria-label="Game viewport"
            ></canvas>

            {#if nearTerminal}
                <div class="pointer-events-none absolute inset-0 flex items-end justify-center pb-4">
                    <div class="interact-prompt">
                        <span class="prompt-icon">&#9733;</span>
                        Press E to interact
                    </div>
                </div>
            {/if}
        </div>
    </div>

    <!-- Slot for HUD and other overlays -->
    {#if children}
        {@render children()}
    {/if}
</div>

<style>
    /* Focus outline for accessibility */
    div:focus {
        outline: none;
    }

    /* Pixel art game frame */
    .game-frame {
        position: relative;
        background: #0a0a14;
        border: 6px solid #3a506b;
        border-top-color: #5a7090;
        border-left-color: #5a7090;
        box-shadow:
            inset 0 0 0 3px #1a1a2e,
            8px 8px 0 #050510;
        padding: 4px;
    }

    .game-canvas {
        display: block;
        image-rendering: pixelated;
        image-rendering: crisp-edges;
    }

    /* Pixel art interaction prompt */
    .interact-prompt {
        display: flex;
        align-items: center;
        gap: 6px;
        background: linear-gradient(180deg, #166534 0%, #14532d 100%);
        border: 3px solid #22c55e;
        border-bottom-color: #166534;
        border-right-color: #166534;
        box-shadow: 3px 3px 0 #050510;
        padding: 6px 12px;
        font-family: 'Press Start 2P', 'Courier New', monospace;
        font-size: 9px;
        color: #dcfce7;
        text-shadow: 1px 1px 0 #14532d;
    }

    .prompt-icon {
        color: #fbbf24;
        animation: twinkle 1s ease-in-out infinite;
    }

    @keyframes twinkle {
        0%, 100% { opacity: 1; }
        50% { opacity: 0.4; }
    }
</style>
