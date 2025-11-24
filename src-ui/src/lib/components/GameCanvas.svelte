<script lang="ts">
    import { onMount } from 'svelte';
    import { createEventDispatcher } from 'svelte';
    import type { RenderState, Direction } from '$lib/types';

    interface Props {
        renderState?: RenderState | null;
        tileSize?: number;
    }

    let { renderState = null, tileSize = 32 }: Props = $props();

    let canvasRef = $state<HTMLCanvasElement | null>(null);
    let ctx = $state<CanvasRenderingContext2D | null>(null);
    const dispatcher = createEventDispatcher();
    let focused = $state(false);

    onMount(() => {
        canvasRef?.focus();
    });

    $effect(() => {
        if (!canvasRef) return;
        const context = canvasRef.getContext('2d');
        if (!context) return;

        ctx = context;
        context.imageSmoothingEnabled = false;
        drawScene(context, renderState);
    });

    function drawScene(context: CanvasRenderingContext2D, state: RenderState | null) {
        const width = canvasRef?.width ?? 0;
        const height = canvasRef?.height ?? 0;
        context.fillStyle = '#0b1224';
        context.fillRect(0, 0, width, height);

        if (!state) {
            context.fillStyle = '#94a3b8';
            context.font = '14px "IBM Plex Mono", monospace';
            context.fillText('Waiting for game tick...', 16, 28);
            return;
        }

        for (let y = 0; y < state.visible_tiles.length; y++) {
            for (let x = 0; x < state.visible_tiles[y].length; x++) {
                const tile = state.visible_tiles[y][x];
                context.fillStyle = tileColor(tile.tile_type);
                context.fillRect(x * tileSize, y * tileSize, tileSize, tileSize);
                context.strokeStyle = 'rgba(148, 163, 184, 0.08)';
                context.strokeRect(x * tileSize, y * tileSize, tileSize, tileSize);
            }
        }

        const px = state.player.position.x - state.viewport_offset.x * tileSize;
        const py = state.player.position.y - state.viewport_offset.y * tileSize;
        context.fillStyle = '#22d3ee';
        context.beginPath();
        context.arc(px, py, tileSize * 0.3, 0, Math.PI * 2);
        context.fill();

        context.strokeStyle = '#0ea5e9';
        const dir = state.player.facing;
        const offset = tileSize * 0.4;
        const dx = dir === 'left' ? -offset : dir === 'right' ? offset : 0;
        const dy = dir === 'up' ? -offset : dir === 'down' ? offset : 0;
        context.beginPath();
        context.moveTo(px, py);
        context.lineTo(px + dx, py + dy);
        context.stroke();
    }

    function tileColor(type: string) {
        switch (type) {
            case 'wall':
                return '#1f2937';
            case 'water':
                return '#0ea5e9';
            case 'void':
                return '#020617';
            case 'door':
                return '#fbbf24';
            case 'terminal':
                return '#22c55e';
            case 'floor':
            default:
                return '#111827';
        }
    }

    function handleKeydown(event: KeyboardEvent) {
        const key = event.key.toLowerCase();
        const mapping: Record<string, Direction> = {
            w: 'up',
            arrowup: 'up',
            a: 'left',
            arrowleft: 'left',
            s: 'down',
            arrowdown: 'down',
            d: 'right',
            arrowright: 'right',
        };
        if (mapping[key]) {
            event.preventDefault();
            dispatcher('move', { direction: mapping[key] });
        }
        if (key === 'e' || key === ' ') {
            event.preventDefault();
            dispatcher('interact');
        }
    }

    function handleClick() {
        canvasRef?.focus();
    }
</script>

<canvas
    bind:this={canvasRef}
    width={tileSize * 20}
    height={tileSize * 15}
    class="w-full rounded-xl border border-slate-800 bg-slate-950 shadow-inner shadow-slate-900 outline-none"
    aria-label="Game viewport"
    role="application"
    tabindex="0"
    on:keydown={handleKeydown}
    on:focus={() => (focused = true)}
    on:blur={() => (focused = false)}
    on:click={handleClick}
></canvas>
