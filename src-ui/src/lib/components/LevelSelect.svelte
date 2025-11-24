<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    import type { LevelInfo } from '$lib/types';

    interface Props {
        levels?: LevelInfo[];
        loading?: boolean;
    }

    let { levels = [], loading = false }: Props = $props();

    const dispatcher = createEventDispatcher();

    function selectLevel(id: string) {
        dispatcher('select', { id });
    }
</script>

<div class="flex items-center justify-between mb-3">
    <div>
        <p class="text-xs uppercase tracking-[0.15em] text-slate-400">Levels</p>
        <h2 class="text-lg font-semibold">Select a level</h2>
    </div>
    <slot name="actions" />
</div>

{#if !levels.length}
    <p class="text-sm text-slate-400">No levels loaded yet.</p>
{:else}
    <div class="grid gap-3 sm:grid-cols-2">
        {#each levels as level (level.id)}
            <article class="flex h-full flex-col justify-between rounded-xl border border-slate-800/80 bg-slate-950/70 px-3 py-3">
                <div>
                    <p class="text-sm font-semibold text-slate-100">{level.title}</p>
                    <p class="text-xs text-slate-400">{level.concept}</p>
                </div>
                <div class="mt-3 flex items-center justify-between text-xs">
                    <span class={`rounded-full px-2 py-1 ${level.completed ? 'bg-emerald-900/50 text-emerald-200' : 'bg-slate-800 text-slate-300'}`}>
                        {level.completed ? 'Completed' : 'Not played'}
                    </span>
                    <button
                        class="control-btn"
                        disabled={level.locked || loading}
                        on:click={() => selectLevel(level.id)}
                    >
                        {level.locked ? 'Locked' : 'Play'}
                    </button>
                </div>
            </article>
        {/each}
    </div>
{/if}
