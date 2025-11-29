<script lang="ts">
    import type { QuestInfo } from '$lib/types';

    interface Props {
        quests: QuestInfo[];
        currentQuestId: string | null;
        onSelectQuest: (questId: string) => void;
    }

    let { quests, currentQuestId, onSelectQuest }: Props = $props();

    // Sort quests by order (use $derived.by to avoid new array on every render)
    let sortedQuests = $derived.by(() => [...quests].sort((a, b) => a.order - b.order));

    // Calculate overall progress
    let completedCount = $derived(quests.filter(q => q.completed).length);
    let progressPercent = $derived(quests.length > 0 ? (completedCount / quests.length) * 100 : 0);

    // Find recommended quest (first incomplete, or marked as recommended)
    let recommendedQuestId = $derived.by(() => {
        const recommended = sortedQuests.find(q => q.recommended && !q.completed);
        if (recommended) return recommended.id;
        const firstIncomplete = sortedQuests.find(q => !q.completed);
        return firstIncomplete?.id ?? null;
    });

    function getQuestStatus(quest: QuestInfo): 'completed' | 'current' | 'recommended' | 'available' {
        if (quest.completed) return 'completed';
        if (quest.id === currentQuestId) return 'current';
        if (quest.id === recommendedQuestId) return 'recommended';
        return 'available';
    }
</script>

<!-- Quest HUD - shows in-level quest progress -->
<div class="quest-hud pixel-font">
    <div class="quest-header">
        <span class="quest-icon">&#9733;</span>
        <span class="quest-title">QUESTS</span>
        <span class="quest-progress">{completedCount}/{quests.length}</span>
    </div>

    <!-- Progress bar -->
    <div class="progress-bar-container">
        <div class="progress-bar" style="width: {progressPercent}%"></div>
    </div>

    <!-- Quest list -->
    <div class="quest-list">
        {#each sortedQuests as quest (quest.id)}
            {@const status = getQuestStatus(quest)}
            <button
                class="quest-item {status}"
                class:active={quest.id === currentQuestId}
                onclick={() => onSelectQuest(quest.id)}
                title={quest.description}
            >
                <span class="quest-check">
                    {#if quest.completed}
                        &#10003;
                    {:else if status === 'recommended'}
                        &#9733;
                    {:else}
                        &#9675;
                    {/if}
                </span>
                <span class="quest-name">{quest.title}</span>
                <span class="quest-xp">+{quest.xp_reward}</span>
            </button>
        {/each}
    </div>

    {#if completedCount === quests.length && quests.length > 0}
        <div class="quest-complete-banner">
            ALL QUESTS COMPLETE!
        </div>
    {/if}
</div>

<style>
    .pixel-font {
        font-family: 'Press Start 2P', 'Courier New', monospace;
        image-rendering: pixelated;
    }

    .quest-hud {
        background: linear-gradient(180deg, #1a1a2e 0%, #16213e 100%);
        border: 3px solid #0f3460;
        border-top-color: #3a506b;
        border-left-color: #3a506b;
        box-shadow:
            inset 0 0 0 1px #0a0a1e,
            4px 4px 0 #0a0a1e;
        padding: 8px;
        min-width: 180px;
        max-width: 220px;
    }

    .quest-header {
        display: flex;
        align-items: center;
        gap: 6px;
        padding-bottom: 6px;
        border-bottom: 2px solid #0f3460;
        margin-bottom: 6px;
    }

    .quest-icon {
        color: #fbbf24;
        font-size: 12px;
        filter: drop-shadow(1px 1px 0 rgba(0, 0, 0, 0.8));
    }

    .quest-title {
        color: #e2e8f0;
        font-size: 9px;
        flex: 1;
    }

    .quest-progress {
        color: #06b6d4;
        font-size: 8px;
    }

    .progress-bar-container {
        height: 6px;
        background: #0a0a1e;
        border: 2px solid #0f3460;
        border-top-color: #050510;
        border-left-color: #050510;
        margin-bottom: 8px;
        overflow: hidden;
    }

    .progress-bar {
        height: 100%;
        background: linear-gradient(180deg,
            #22d3ee 0%,
            #06b6d4 50%,
            #0891b2 100%
        );
        transition: width 0.3s ease;
    }

    .quest-list {
        display: flex;
        flex-direction: column;
        gap: 4px;
    }

    .quest-item {
        display: flex;
        align-items: center;
        gap: 6px;
        padding: 6px 8px;
        background: #0f172a;
        border: 2px solid transparent;
        border-radius: 2px;
        cursor: pointer;
        transition: all 0.15s;
        text-align: left;
        font-family: inherit;
    }

    .quest-item:hover {
        background: #1e293b;
        border-color: #3b82f6;
    }

    .quest-item.active {
        background: #1e3a5f;
        border-color: #3b82f6;
        box-shadow: 0 0 8px rgba(59, 130, 246, 0.3);
    }

    .quest-item.completed {
        opacity: 0.6;
    }

    .quest-item.completed:hover {
        opacity: 0.8;
    }

    .quest-check {
        width: 14px;
        font-size: 10px;
        text-align: center;
    }

    .quest-item.completed .quest-check {
        color: #22c55e;
    }

    .quest-item.recommended .quest-check {
        color: #fbbf24;
        animation: pulse 1.5s ease-in-out infinite;
    }

    .quest-item.current .quest-check {
        color: #3b82f6;
    }

    .quest-item.available .quest-check {
        color: #64748b;
    }

    .quest-name {
        flex: 1;
        font-size: 7px;
        color: #e2e8f0;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .quest-item.completed .quest-name {
        text-decoration: line-through;
        color: #94a3b8;
    }

    .quest-xp {
        font-size: 6px;
        color: #fbbf24;
    }

    .quest-item.completed .quest-xp {
        color: #22c55e;
    }

    .quest-complete-banner {
        margin-top: 8px;
        padding: 8px;
        background: linear-gradient(180deg, #14532d 0%, #166534 100%);
        border: 2px solid #22c55e;
        text-align: center;
        font-size: 7px;
        color: #bbf7d0;
        animation: glow 2s ease-in-out infinite;
    }

    @keyframes pulse {
        0%, 100% { opacity: 1; }
        50% { opacity: 0.5; }
    }

    @keyframes glow {
        0%, 100% { box-shadow: 0 0 4px rgba(34, 197, 94, 0.3); }
        50% { box-shadow: 0 0 12px rgba(34, 197, 94, 0.6); }
    }
</style>
