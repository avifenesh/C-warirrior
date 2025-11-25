<script lang="ts">
    import { onMount, type Snippet } from 'svelte';
    import { createEventDispatcher } from 'svelte';
    import type { RenderState, Direction } from '$lib/stores/game.svelte';

    interface Props {
        renderState: RenderState | null;
        tileSize?: number;
        children?: Snippet;
    }

    let { renderState = null, tileSize = 32, children }: Props = $props();

    const dispatcher = createEventDispatcher();
    let canvasRef = $state<HTMLCanvasElement | null>(null);
    let containerRef = $state<HTMLDivElement | null>(null);

    // Compute nearTerminal as derived state to avoid infinite loop
    const nearTerminal = $derived.by(() => {
        if (!renderState) return false;
        const playerTile = {
            x: Math.floor(renderState.player.position.x / tileSize),
            y: Math.floor(renderState.player.position.y / tileSize),
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
            dispatcher('move', { direction: movementMapping[key] });
        }

        // Interact key (E or Space)
        if (key === 'e' || key === ' ') {
            event.preventDefault();
            dispatcher('interact');
        }
    }

    onMount(() => {
        window.addEventListener('keydown', handleKeydown);
        containerRef?.focus();

        return () => {
            window.removeEventListener('keydown', handleKeydown);
        };
    });

    // Canvas rendering
    $effect(() => {
        if (!canvasRef) return;
        const context = canvasRef.getContext('2d');
        if (!context) return;

        context.imageSmoothingEnabled = false;
        drawScene(context, renderState);
    });

    function drawScene(context: CanvasRenderingContext2D, state: RenderState | null) {
        const width = canvasRef?.width ?? 0;
        const height = canvasRef?.height ?? 0;

        // Clear background with dark void
        context.fillStyle = '#020617';
        context.fillRect(0, 0, width, height);

        if (!state) {
            // Loading state
            context.fillStyle = '#64748b';
            context.font = '16px "IBM Plex Mono", monospace';
            context.textAlign = 'center';
            context.fillText('Initializing world...', width / 2, height / 2);
            return;
        }

        const playerTile = {
            x: Math.floor(state.player.position.x / tileSize) - state.viewport_offset.x,
            y: Math.floor(state.player.position.y / tileSize) - state.viewport_offset.y,
        };

        // Draw tiles
        for (let y = 0; y < state.visible_tiles.length; y++) {
            for (let x = 0; x < state.visible_tiles[y].length; x++) {
                const tile = state.visible_tiles[y][x];
                const tileX = x * tileSize;
                const tileY = y * tileSize;
                context.fillStyle = getTileColor(tile.tile_type);
                context.fillRect(tileX, tileY, tileSize, tileSize);

                // Tile border (subtle grid)
                context.strokeStyle = 'rgba(148, 163, 184, 0.05)';
                context.lineWidth = 1;
                context.strokeRect(tileX, tileY, tileSize, tileSize);

                const manhattan = Math.abs(playerTile.x - x) + Math.abs(playerTile.y - y);
                const isNear = manhattan <= 1;

                if (tile.tile_type === 'terminal') {
                    // Glow effect when nearby
                    if (isNear) {
                        context.shadowBlur = 14;
                        context.shadowColor = '#22c55e';
                        context.strokeStyle = '#22c55e';
                        context.lineWidth = 2;
                        context.strokeRect(tileX + 2, tileY + 2, tileSize - 4, tileSize - 4);
                        context.shadowBlur = 0;
                    } else {
                        context.strokeStyle = 'rgba(34, 197, 94, 0.5)';
                        context.lineWidth = 1;
                        context.strokeRect(tileX + 4, tileY + 4, tileSize - 8, tileSize - 8);
                    }
                }

                if (tile.tile_type === 'door') {
                    const locked = !tile.walkable;
                    context.fillStyle = locked ? 'rgba(248, 113, 113, 0.3)' : 'rgba(52, 211, 153, 0.25)';
                    context.fillRect(tileX, tileY, tileSize, tileSize);

                    if (locked) {
                        // Simple padlock icon
                        context.fillStyle = '#ef4444';
                        const lockX = tileX + tileSize * 0.35;
                        const lockY = tileY + tileSize * 0.35;
                        const lockW = tileSize * 0.3;
                        const lockH = tileSize * 0.3;
                        context.fillRect(lockX, lockY, lockW, lockH);
                        context.beginPath();
                        context.arc(lockX + lockW / 2, lockY, lockW / 2, Math.PI, 0);
                        context.fill();
                    } else {
                        context.strokeStyle = '#22c55e';
                        context.lineWidth = 2;
                        context.strokeRect(tileX + 6, tileY + 6, tileSize - 12, tileSize - 12);
                    }
                }
            }
        }

        // Draw player
        const px = state.player.position.x - state.viewport_offset.x * tileSize;
        const py = state.player.position.y - state.viewport_offset.y * tileSize;

        // Player glow
        context.shadowBlur = 15;
        context.shadowColor = '#22d3ee';

        // Player body (circle)
        context.fillStyle = '#22d3ee';
        context.beginPath();
        context.arc(px, py, tileSize * 0.35, 0, Math.PI * 2);
        context.fill();

        context.shadowBlur = 0;

        // Player direction indicator
        context.strokeStyle = '#0ea5e9';
        context.lineWidth = 3;
        const dir = state.player.facing;
        const offset = tileSize * 0.5;
        const dx = dir === 'left' ? -offset : dir === 'right' ? offset : 0;
        const dy = dir === 'up' ? -offset : dir === 'down' ? offset : 0;
        context.beginPath();
        context.moveTo(px, py);
        context.lineTo(px + dx, py + dy);
        context.stroke();
    }

    function getTileColor(type: string): string {
        switch (type) {
            case 'wall':
                return '#1e293b';
            case 'water':
                return '#0c4a6e';
            case 'void':
                return '#020617';
            case 'door':
                return '#78350f';
            case 'terminal':
                return '#14532d';
            case 'floor':
            default:
                return '#0f172a';
        }
    }

    function handleContainerClick() {
        containerRef?.focus();
    }

    function handleContainerKeydown(event: KeyboardEvent) {
        // Focus is handled by the container, actual key handling is in handleKeydown
        if (event.key === 'Enter' || event.key === ' ') {
            containerRef?.focus();
        }
    }
</script>

<div
    bind:this={containerRef}
    class="relative min-h-screen w-full bg-slate-950 outline-none"
    tabindex="0"
    role="application"
    aria-label="Game world - use WASD or arrow keys to move, E to interact"
    onclick={handleContainerClick}
    onkeydown={handleContainerKeydown}
>
    <!-- Canvas container -->
    <div class="flex items-center justify-center min-h-screen p-8">
        <div class="relative">
            <canvas
                bind:this={canvasRef}
                width={tileSize * 20}
                height={tileSize * 15}
                class="rounded-2xl border-2 border-slate-800 shadow-2xl shadow-slate-950/50"
                aria-label="Game viewport"
            ></canvas>

            <!-- Vignette effect -->
            <div
                class="pointer-events-none absolute inset-0 rounded-2xl"
                style="box-shadow: inset 0 0 100px 40px rgba(2, 6, 23, 0.5);"
            ></div>

            {#if nearTerminal}
                <div class="pointer-events-none absolute inset-0 flex items-end justify-center pb-6">
                    <div class="rounded-full bg-emerald-500/20 px-4 py-2 text-xs font-semibold text-emerald-100 shadow shadow-emerald-500/30">
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
</style>
