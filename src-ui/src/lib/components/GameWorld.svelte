<script lang="ts">
    import { onMount } from 'svelte';
    import { createEventDispatcher } from 'svelte';
    import type { RenderState, Direction } from '$lib/stores/game.svelte';

    interface Props {
        renderState: RenderState | null;
        tileSize?: number;
    }

    let { renderState = null, tileSize = 32 }: Props = $props();

    const dispatcher = createEventDispatcher();
    let canvasRef = $state<HTMLCanvasElement | null>(null);
    let containerRef = $state<HTMLDivElement | null>(null);

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

        // Draw tiles
        for (let y = 0; y < state.visible_tiles.length; y++) {
            for (let x = 0; x < state.visible_tiles[y].length; x++) {
                const tile = state.visible_tiles[y][x];
                context.fillStyle = getTileColor(tile.tile_type);
                context.fillRect(x * tileSize, y * tileSize, tileSize, tileSize);

                // Tile border (subtle grid)
                context.strokeStyle = 'rgba(148, 163, 184, 0.05)';
                context.lineWidth = 1;
                context.strokeRect(x * tileSize, y * tileSize, tileSize, tileSize);

                // Special tile highlights
                if (tile.tile_type === 'terminal') {
                    // Glow effect for terminals
                    context.shadowBlur = 10;
                    context.shadowColor = '#22c55e';
                    context.strokeStyle = '#22c55e';
                    context.lineWidth = 2;
                    context.strokeRect(
                        x * tileSize + 2,
                        y * tileSize + 2,
                        tileSize - 4,
                        tileSize - 4
                    );
                    context.shadowBlur = 0;
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
</script>

<div
    bind:this={containerRef}
    class="relative min-h-screen w-full bg-slate-950 outline-none"
    tabindex="-1"
    role="application"
    aria-label="Game world"
    onclick={handleContainerClick}
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
        </div>
    </div>

    <!-- Slot for HUD and other overlays -->
    <slot />
</div>

<style>
    /* Focus outline for accessibility */
    div:focus {
        outline: none;
    }
</style>
