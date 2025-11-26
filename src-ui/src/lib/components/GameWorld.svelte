<script lang="ts">
    import { onMount, type Snippet } from 'svelte';
    import { createEventDispatcher } from 'svelte';
    import type { RenderState, Direction } from '$lib/types';
    import { loadAssets, DEFAULT_MANIFEST, type LoadedAssets } from '$lib/engine/assets';
    import { ANIMATIONS, startAnimation, getCurrentFrame, type AnimationState } from '$lib/engine/animation';
    import { Camera, applyCameraTransform, resetCameraTransform } from '$lib/engine/camera';
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
    let animationTime = $state(0);

    // Backend uses 32px tile size for positions
    const BACKEND_TILE_SIZE = 32;

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
                console.log('[GameWorld] Dispatching interact event, nearTerminal:', nearTerminal);
                dispatcher('interact');
            } else {
                console.log('[GameWorld] Interact blocked - nothing nearby');
            }
            return;
        }
    }

    onMount(() => {
        window.addEventListener('keydown', handleKeydown);
        containerRef?.focus();

        // Load assets
        console.log('[GameWorld] Starting asset load...');
        loadAssets(DEFAULT_MANIFEST)
            .then((loadedAssets) => {
                console.log('[GameWorld] Assets loaded successfully:', {
                    sprites: loadedAssets.sprites.size,
                    tiles: loadedAssets.tiles.size,
                });
                console.log('[GameWorld] Tile keys:', Array.from(loadedAssets.tiles.keys()));
                console.log('[GameWorld] Sprite keys:', Array.from(loadedAssets.sprites.keys()));
                console.log('[GameWorld] Floor tile:', loadedAssets.tiles.get('floor'));
                assets = loadedAssets;
                assetsLoaded = true;
                // Initialize animations
                playerAnimState = startAnimation(ANIMATIONS.playerIdle);
                terminalAnimState = startAnimation(ANIMATIONS.terminalGlow);
            })
            .catch((err) => {
                console.error('[GameWorld] Asset loading failed:', err);
                // Mark as loaded anyway so we use fallback
                assetsLoaded = true;
            });

        // Animation and rendering loop
        let animationFrameId = 0;
        const animLoop = (currentTime: number) => {
            // Snapshot renderState at the beginning of the frame to prevent mid-frame changes
            const currentState = renderState;

            const deltaTime = lastFrameTime > 0 ? currentTime - lastFrameTime : 16.67;
            lastFrameTime = currentTime;
            animationTime = currentTime;

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

            // Update particles
            particles.update(deltaTime);

            // Render scene with snapshot
            if (canvasRef) {
                const context = canvasRef.getContext('2d');
                if (context) {
                    try {
                        context.imageSmoothingEnabled = false;
                        drawScene(context, currentState);
                    } catch (err) {
                        console.error('[GameWorld] Rendering error:', err);
                    }
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

    function drawScene(context: CanvasRenderingContext2D, state: RenderState | null) {
        const width = canvasRef?.width ?? 0;
        const height = canvasRef?.height ?? 0;

        // Clear background with natural dark
        context.fillStyle = '#0a0a14';
        context.fillRect(0, 0, width, height);

        if (!state) {
            // Loading state with pixel art style
            context.fillStyle = '#fbbf24';
            context.font = '12px "Press Start 2P", "Courier New", monospace';
            context.textAlign = 'center';
            context.fillText(assetsLoaded ? 'Entering world...' : 'Loading...', width / 2, height / 2);
            return;
        }

        if (!assetsLoaded || !assets) {
            // Assets still loading, show colored rectangles as fallback
            drawFallbackScene(context, state);
            return;
        }

        // Extra null safety check - state may be partially populated during init
        if (!state.visible_tiles || !state.player) {
            return;
        }

        const playerTile = {
            x: Math.floor(state.player.position.x / BACKEND_TILE_SIZE) - state.viewport_offset.x,
            y: Math.floor(state.player.position.y / BACKEND_TILE_SIZE) - state.viewport_offset.y,
        };

        // Draw tiles with sprites
        for (let y = 0; y < state.visible_tiles.length; y++) {
            for (let x = 0; x < state.visible_tiles[y].length; x++) {
                const tile = state.visible_tiles[y][x];
                const tileX = x * tileSize;
                const tileY = y * tileSize;

                // Draw solid background color first (handles transparent sprites)
                context.fillStyle = getTileColor(tile.tile_type);
                context.fillRect(tileX, tileY, tileSize, tileSize);

                // For interactive tiles (terminal, npc), draw grass texture first as base
                if (tile.tile_type === 'terminal' || tile.tile_type === 'npc') {
                    const grassSprite = assets.tiles.get('grass');
                    if (grassSprite) {
                        context.drawImage(grassSprite, tileX, tileY, tileSize, tileSize);
                    }
                }

                // Then draw tile sprite on top (if available)
                const tileSprite = getTileSprite(tile.tile_type, tile.walkable);
                if (tileSprite) {
                    // Check if this is an animated tile (sprite sheet - width > height)
                    const isAnimatedTile = (tile.tile_type === 'water' || tile.tile_type === 'terminal')
                        && tileSprite.width > tileSprite.height;

                    if (isAnimatedTile) {
                        // 4-frame animation (water: 200ms, terminal: 300ms for slower pulse)
                        const frameCount = 4;
                        const frameDuration = tile.tile_type === 'terminal' ? 300 : 200;
                        const frameIndex = Math.floor(animationTime / frameDuration) % frameCount;
                        const frameWidth = tileSprite.width / frameCount;
                        context.drawImage(
                            tileSprite,
                            frameIndex * frameWidth, 0, frameWidth, tileSprite.height, // source
                            tileX, tileY, tileSize, tileSize // destination
                        );
                    } else {
                        context.drawImage(tileSprite, tileX, tileY, tileSize, tileSize);
                    }
                }

                const manhattan = Math.abs(playerTile.x - x) + Math.abs(playerTile.y - y);
                const isNear = manhattan <= 1;

                // Highlight for interactable tiles when player is near
                if ((tile.tile_type === 'terminal' || tile.tile_type === 'npc') && isNear) {
                    // Simple pixel-style highlight border
                    context.strokeStyle = tile.tile_type === 'npc' ? '#60a5fa' : '#fbbf24'; // blue for NPC, gold for terminal
                    context.lineWidth = 2;
                    context.strokeRect(tileX + 1, tileY + 1, tileSize - 2, tileSize - 2);
                }

                // Locked door indicator
                if (tile.tile_type === 'door' && !tile.walkable) {
                    // Red lock indicator
                    context.fillStyle = '#dc2626';
                    context.fillRect(tileX + tileSize - 8, tileY + 4, 4, 4);
                }
            }
        }

        // Draw player with sprite
        // Scale from backend coords (32px tiles) to frontend coords
        const scaleFactor = tileSize / BACKEND_TILE_SIZE;
        const px = state.player.position.x * scaleFactor - state.viewport_offset.x * tileSize;
        const py = state.player.position.y * scaleFactor - state.viewport_offset.y * tileSize;

        // Get player sprite based on facing direction
        const playerSpriteName = `player_${state.player.facing}`;
        const playerSprite = assets.sprites.get(playerSpriteName);

        if (playerSprite) {
            // Draw animated player sprite
            const frameIndex = playerAnimState ? getCurrentFrame(playerAnimState, animationTime) : 0;
            let spriteOffsetX = px - tileSize / 2;
            let spriteOffsetY = py - tileSize / 2;

            // Handle sprite sheets vs single frames
            const isSpriteSheet = playerSprite.width > playerSprite.height || playerSprite.height > playerSprite.width;
            
            if (isSpriteSheet) {
                // Assume horizontal strip for now (common format)
                // If vertical strip is needed, we'd check dimensions
                const frameWidth = playerSprite.height; // Assume square frames based on height
                const frameX = frameIndex * frameWidth;
                
                context.drawImage(
                    playerSprite, 
                    frameX, 0, frameWidth, playerSprite.height, // Source
                    spriteOffsetX, spriteOffsetY, tileSize, tileSize // Destination
                );
            } else {
                // Single frame - add "bobbing" animation if walking
                // Use ID comparison to avoid Svelte 5 proxy equality mismatch
                if (playerAnimState?.animation?.id === ANIMATIONS.playerWalk.id) {
                    const bobOffset = Math.sin(animationTime / 50) * 2; // +/- 2px bob
                    spriteOffsetY += bobOffset;
                }
                
                context.drawImage(playerSprite, spriteOffsetX, spriteOffsetY, tileSize, tileSize);
            }
        } else {
            // Fallback to colored knight shape if sprite not loaded
            context.fillStyle = '#708090'; // armor gray
            context.fillRect(px - tileSize * 0.35, py - tileSize * 0.4, tileSize * 0.7, tileSize * 0.8);
            // Gold trim
            context.fillStyle = '#fbbf24';
            context.fillRect(px - tileSize * 0.2, py - tileSize * 0.35, tileSize * 0.4, tileSize * 0.1);
        }

        // Render particles (on top of everything)
        particles.render(context, state.viewport_offset, tileSize);
    }

    function getTileSprite(tileType: string, walkable: boolean): HTMLImageElement | null | undefined {
        if (!assets) return null;

        let spriteName: string;
        switch (tileType) {
            case 'floor':
                spriteName = 'grass';  // Changed from 'floor' to 'grass'
                break;
            case 'wall':
                spriteName = 'wall';
                break;
            case 'water':
                spriteName = 'water';
                break;
            case 'void':
                spriteName = 'void';
                break;
            case 'terminal':
                spriteName = 'terminal';
                break;
            case 'door':
                spriteName = walkable ? 'door_open' : 'door_locked';
                break;
            case 'npc':
                // NPC sprites are in sprites map, not tiles
                return assets.sprites.get('npc_mentor');
            default:
                spriteName = 'grass';  // Default to grass tile
        }

        return assets.tiles.get(spriteName);
    }

    function drawFallbackScene(context: CanvasRenderingContext2D, state: RenderState) {
        const playerTile = {
            x: Math.floor(state.player.position.x / BACKEND_TILE_SIZE) - state.viewport_offset.x,
            y: Math.floor(state.player.position.y / BACKEND_TILE_SIZE) - state.viewport_offset.y,
        };

        // Draw tiles
        for (let y = 0; y < state.visible_tiles.length; y++) {
            for (let x = 0; x < state.visible_tiles[y].length; x++) {
                const tile = state.visible_tiles[y][x];
                const tileX = x * tileSize;
                const tileY = y * tileSize;
                context.fillStyle = getTileColor(tile.tile_type);
                context.fillRect(tileX, tileY, tileSize, tileSize);

                const manhattan = Math.abs(playerTile.x - x) + Math.abs(playerTile.y - y);
                const isNear = manhattan <= 1;

                if (tile.tile_type === 'terminal' && isNear) {
                    context.strokeStyle = '#fbbf24';
                    context.lineWidth = 2;
                    context.strokeRect(tileX + 1, tileY + 1, tileSize - 2, tileSize - 2);
                }
            }
        }

        // Draw player fallback (knight shape)
        // Scale from backend coords to frontend coords
        const scaleFactor = tileSize / BACKEND_TILE_SIZE;
        const px = state.player.position.x * scaleFactor - state.viewport_offset.x * tileSize;
        const py = state.player.position.y * scaleFactor - state.viewport_offset.y * tileSize;
        context.fillStyle = '#708090'; // armor gray
        context.fillRect(px - tileSize * 0.35, py - tileSize * 0.4, tileSize * 0.7, tileSize * 0.8);
        context.fillStyle = '#fbbf24'; // gold trim
        context.fillRect(px - tileSize * 0.2, py - tileSize * 0.35, tileSize * 0.4, tileSize * 0.1);
    }

    function getTileColor(type: string): string {
        switch (type) {
            case 'wall':
                return '#4a4a4a'; // stone gray
            case 'water':
                return '#1e6091'; // natural blue
            case 'void':
                return '#0a0a14'; // dark void
            case 'door':
                return '#8b5a2b'; // visible wood brown (brighter)
            case 'terminal':
            case 'npc':
                return '#3d7a37'; // same as floor (grass) - sprite has transparent bg
            case 'floor':
            default:
                return '#3d7a37'; // grass green
        }
    }

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
