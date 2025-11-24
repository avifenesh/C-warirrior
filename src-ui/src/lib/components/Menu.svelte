<script lang="ts">
    import { createEventDispatcher } from 'svelte';

    const dispatcher = createEventDispatcher();

    function startGame() {
        dispatcher('start');
    }

    function openLevels() {
        dispatcher('levels');
    }
</script>

<div class="rounded-2xl border border-slate-800 bg-slate-900/70 p-6 shadow-lg shadow-slate-950/40">
    <p class="text-xs uppercase tracking-[0.2em] text-slate-400">Main Menu</p>
    <h2 class="mt-2 text-2xl font-semibold text-slate-100">Code Warrior</h2>
    <p class="mt-1 text-sm text-slate-400">Frontend is render-only. Rust owns the game state.</p>

    <div class="mt-4 flex flex-wrap gap-3">
        <button class="control-btn" on:click={startGame}>Start Game</button>
        <button class="control-btn" on:click={openLevels}>Level Select</button>
    </div>
</div>
