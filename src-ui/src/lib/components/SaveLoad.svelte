<script lang="ts">
    import { createEventDispatcher } from 'svelte';

    export interface SaveSlot {
        id: string;
        name: string;
        timestamp: string;
        progress: string;
        empty?: boolean;
    }

    interface Props {
        slots?: SaveSlot[];
        loading?: boolean;
    }

    const dispatcher = createEventDispatcher();
    let { slots = [], loading = false }: Props = $props();

    function save(id: string) {
        dispatcher('save', { id });
    }

    function load(id: string) {
        dispatcher('load', { id });
    }

    function remove(id: string) {
        dispatcher('delete', { id });
    }
</script>

<section class="save-load-container">
    <div class="save-load-header">
        <div>
            <p class="save-load-subtitle">Saves</p>
            <h2 class="save-load-title">Save / Load</h2>
        </div>
        <span class="save-load-hint">Local only (UI placeholder)</span>
    </div>

    <div class="save-slots">
        {#each slots as slot (slot.id)}
            <div class="save-slot-item">
                <div class="save-slot-info">
                    <p class="save-slot-name">{slot.empty ? 'Empty Slot' : slot.name}</p>
                    <p class="save-slot-progress">{slot.empty ? 'No data' : slot.progress}</p>
                    <p class="save-slot-timestamp">{slot.timestamp}</p>
                </div>
                <div class="save-slot-actions">
                    <button
                        class="pixel-button"
                        disabled={loading}
                        onclick={() => save(slot.id)}
                    >
                        Save
                    </button>
                    <button
                        class="pixel-button"
                        disabled={slot.empty || loading}
                        onclick={() => load(slot.id)}
                    >
                        Load
                    </button>
                    <button
                        class="pixel-button"
                        disabled={slot.empty || loading}
                        onclick={() => remove(slot.id)}
                    >
                        Delete
                    </button>
                </div>
            </div>
        {/each}
    </div>
</section>

<style lang="postcss">
    /* Import Press Start 2P font */
    @import url('https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap');

    .save-load-container {
        @apply space-y-4 font-['Press_Start_2P'] text-sm;
        background: linear-gradient(to bottom right, #1a1a2e, #16213e);
        padding: 1rem;
        border-radius: 8px;
        box-shadow: 4px 4px 8px rgba(0, 0, 0, 0.5);
    }

    .save-load-header {
        @apply flex items-center justify-between mb-4;
    }

    .save-load-subtitle {
        @apply text-xs uppercase tracking-widest text-[#fbbf24];
    }

    .save-load-title {
        @apply text-lg text-[#fbbf24];
    }

    .save-load-hint {
        @apply text-[11px] text-slate-400;
    }

    .save-slots {
        @apply space-y-3;
    }

    .save-slot-item {
        @apply flex items-center justify-between;
        background: linear-gradient(135deg, rgba(26, 26, 46, 0.8), rgba(22, 33, 62, 0.8));
        border: 2px solid #16213e;
        box-shadow:
            -2px -2px 4px rgba(255, 255, 255, 0.1),
            2px 2px 4px rgba(0, 0, 0, 0.3);
        padding: 1rem;
        border-radius: 8px;
    }

    .save-slot-info {
        @apply space-y-1;
    }

    .save-slot-name {
        @apply text-[#fbbf24] font-bold;
    }

    .save-slot-progress {
        @apply text-xs text-slate-300;
    }

    .save-slot-timestamp {
        @apply text-[11px] text-slate-400;
    }

    .save-slot-actions {
        @apply flex gap-2;
    }

    .pixel-button {
        @apply px-3 py-2 text-xs;
        background: linear-gradient(to bottom right, #16213e, #1a1a2e);
        color: #fbbf24;
        border: 2px solid #16213e;
        box-shadow:
            -2px -2px 4px rgba(255, 255, 255, 0.1),
            2px 2px 4px rgba(0, 0, 0, 0.3);
        cursor: pointer;
        transition: all 0.2s ease;
    }

    .pixel-button:hover:not(:disabled) {
        background: linear-gradient(to bottom right, #1a1a2e, #16213e);
        transform: translate(1px, 1px);
        box-shadow:
            -1px -1px 2px rgba(255, 255, 255, 0.1),
            1px 1px 2px rgba(0, 0, 0, 0.3);
    }

    .pixel-button:disabled {
        @apply opacity-50 cursor-not-allowed;
    }
</style>