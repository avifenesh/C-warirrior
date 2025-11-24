<script lang="ts">
    import type { Player } from '$lib/types';

    interface Props {
        player?: Player | null;
        currentLevelId?: string | null;
    }

    let { player = null, currentLevelId = null }: Props = $props();
</script>

<div class="grid grid-cols-3 gap-4 rounded-2xl border border-slate-800 bg-slate-900/70 p-4 text-sm shadow-lg shadow-slate-950/40">
    <div>
        <p class="text-[11px] uppercase tracking-[0.1em] text-slate-400">Level</p>
        <p class="text-lg font-semibold text-slate-100">{currentLevelId ?? '--'}</p>
    </div>
    <div>
        <p class="text-[11px] uppercase tracking-[0.1em] text-slate-400">XP</p>
        <p class="font-mono text-slate-100">{player ? player.xp : 0}</p>
    </div>
    <div>
        <p class="text-[11px] uppercase tracking-[0.1em] text-slate-400">Health</p>
        <p class="font-mono text-emerald-300">{player ? `${player.health}/${player.max_health}` : '0/0'}</p>
    </div>
</div>
