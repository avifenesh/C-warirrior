<script lang="ts">
    import type { LevelData, Lesson } from '$lib/types';

    interface Props {
        level: LevelData | null;
        onStart: () => void;
    }

    let { level, onStart }: Props = $props();

    // Derive display values
    const title = $derived(level?.title ?? 'Unknown Level');
    const concept = $derived(level?.concept ?? 'programming');
    const lesson = $derived(level?.lesson);
    const questCount = $derived(level?.quests?.length ?? 0);
    const totalXp = $derived(level?.total_xp_reward ?? 0);

    // Format concept for display (e.g., "return_values" -> "Return Values")
    function formatConcept(c: string): string {
        return c
            .split(/[_\s]+/)
            .map(word => word.charAt(0).toUpperCase() + word.slice(1))
            .join(' ');
    }

    function handleKeydown(event: KeyboardEvent) {
        if (event.key === 'Enter' || event.key === 'Escape') {
            event.preventDefault();
            onStart();
        }
    }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if level}
<div class="intro-overlay">
    <div class="intro-modal">
        <!-- Header with level icon -->
        <div class="intro-header">
            <div class="level-icon">
                <span class="icon-glyph">&#9733;</span>
            </div>
            <div class="header-text">
                <span class="level-label">LEVEL {level.id?.replace('L', '') ?? '?'}</span>
                <h1 class="level-title">{title}</h1>
            </div>
        </div>

        <!-- Concept Banner -->
        <div class="concept-banner">
            <span class="concept-label">C CONCEPT</span>
            <span class="concept-name">{formatConcept(concept)}</span>
        </div>

        <!-- Lesson Content -->
        {#if lesson}
            <div class="lesson-section">
                <h2 class="lesson-title">{lesson.title}</h2>
                <div class="lesson-content">
                    {#each lesson.content as paragraph}
                        <p class="lesson-para">{paragraph}</p>
                    {/each}
                </div>

                {#if lesson.examples && lesson.examples.length > 0}
                    <div class="examples-section">
                        <h3 class="examples-title">Examples</h3>
                        {#each lesson.examples as example}
                            <div class="example-block">
                                <pre class="example-code"><code>{example.code}</code></pre>
                                <p class="example-explanation">{example.explanation}</p>
                            </div>
                        {/each}
                    </div>
                {/if}
            </div>
        {:else}
            <div class="lesson-section">
                <p class="lesson-para">Explore this level and complete the quests to master {formatConcept(concept)}!</p>
            </div>
        {/if}

        <!-- Quest Info -->
        <div class="quest-info">
            <div class="info-item">
                <span class="info-icon">&#9876;</span>
                <span class="info-text">{questCount} Quest{questCount !== 1 ? 's' : ''}</span>
            </div>
            <div class="info-item">
                <span class="info-icon">&#9830;</span>
                <span class="info-text">{totalXp} XP Available</span>
            </div>
        </div>

        <!-- Start Button -->
        <button class="start-btn" onclick={onStart}>
            BEGIN ADVENTURE
        </button>

        <p class="hint-text">Walk to terminals (&#9733;) to access quests</p>
    </div>
</div>
{/if}

<style>
    .intro-overlay {
        position: fixed;
        inset: 0;
        z-index: 100;
        display: flex;
        align-items: center;
        justify-content: center;
        background: rgba(5, 5, 16, 0.95);
        animation: fadeIn 0.3s ease-out;
    }

    @keyframes fadeIn {
        from { opacity: 0; }
        to { opacity: 1; }
    }

    .intro-modal {
        background: linear-gradient(180deg, #1a1a2e 0%, #0f0f1a 100%);
        border: 4px solid #3a506b;
        border-top-color: #5a7090;
        border-left-color: #5a7090;
        box-shadow:
            inset 0 0 0 2px #0a0a1e,
            8px 8px 0 #050510;
        width: 100%;
        max-width: 600px;
        max-height: 85vh;
        overflow-y: auto;
        animation: modalAppear 0.4s ease-out;
    }

    @keyframes modalAppear {
        from {
            opacity: 0;
            transform: scale(0.9) translateY(20px);
        }
        to {
            opacity: 1;
            transform: scale(1) translateY(0);
        }
    }

    /* Header */
    .intro-header {
        display: flex;
        align-items: center;
        gap: 16px;
        padding: 20px 24px;
        background: linear-gradient(180deg, #16213e 0%, #0f3460 100%);
        border-bottom: 3px solid #0a0a1e;
    }

    .level-icon {
        width: 56px;
        height: 56px;
        background: linear-gradient(180deg, #92400e 0%, #78350f 100%);
        border: 3px solid #fbbf24;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .icon-glyph {
        font-size: 28px;
        color: #fbbf24;
        text-shadow: 2px 2px 0 #92400e;
    }

    .header-text {
        flex: 1;
    }

    .level-label {
        font-family: 'Press Start 2P', 'Courier New', monospace;
        font-size: 9px;
        color: #64748b;
        letter-spacing: 2px;
    }

    .level-title {
        font-family: 'Press Start 2P', 'Courier New', monospace;
        font-size: 16px;
        color: #fbbf24;
        text-shadow: 2px 2px 0 #92400e;
        margin-top: 4px;
        line-height: 1.3;
    }

    /* Concept Banner */
    .concept-banner {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 12px;
        padding: 12px 24px;
        background: linear-gradient(180deg, #0f3460 0%, #1a1a2e 100%);
        border-bottom: 2px solid #1e293b;
    }

    .concept-label {
        font-family: 'Press Start 2P', 'Courier New', monospace;
        font-size: 8px;
        color: #64748b;
        letter-spacing: 1px;
    }

    .concept-name {
        font-family: 'Press Start 2P', 'Courier New', monospace;
        font-size: 12px;
        color: #67e8f9;
        text-shadow: 1px 1px 0 #0a0a1e;
    }

    /* Lesson Section */
    .lesson-section {
        padding: 20px 24px;
    }

    .lesson-title {
        font-family: 'Press Start 2P', 'Courier New', monospace;
        font-size: 11px;
        color: #4ade80;
        margin-bottom: 16px;
        text-shadow: 1px 1px 0 #14532d;
    }

    .lesson-content {
        margin-bottom: 16px;
    }

    .lesson-para {
        font-size: 14px;
        color: #e2e8f0;
        line-height: 1.7;
        margin-bottom: 12px;
    }

    .lesson-para:last-child {
        margin-bottom: 0;
    }

    /* Examples */
    .examples-section {
        margin-top: 20px;
        padding-top: 16px;
        border-top: 2px solid #1e293b;
    }

    .examples-title {
        font-family: 'Press Start 2P', 'Courier New', monospace;
        font-size: 9px;
        color: #67e8f9;
        margin-bottom: 12px;
    }

    .example-block {
        margin-bottom: 16px;
    }

    .example-block:last-child {
        margin-bottom: 0;
    }

    .example-code {
        background: #0a0a14;
        border: 2px solid #1e293b;
        padding: 12px;
        font-family: 'IBM Plex Mono', monospace;
        font-size: 12px;
        color: #4ade80;
        overflow-x: auto;
        margin-bottom: 8px;
    }

    .example-explanation {
        font-size: 12px;
        color: #94a3b8;
        font-style: italic;
        padding-left: 12px;
        border-left: 2px solid #3a506b;
    }

    /* Quest Info */
    .quest-info {
        display: flex;
        justify-content: center;
        gap: 32px;
        padding: 16px 24px;
        background: #0a0a14;
        border-top: 2px solid #1e293b;
        border-bottom: 2px solid #1e293b;
    }

    .info-item {
        display: flex;
        align-items: center;
        gap: 8px;
    }

    .info-icon {
        font-size: 16px;
        color: #fbbf24;
    }

    .info-text {
        font-family: 'Press Start 2P', 'Courier New', monospace;
        font-size: 9px;
        color: #e2e8f0;
    }

    /* Start Button */
    .start-btn {
        display: block;
        width: calc(100% - 48px);
        margin: 20px 24px;
        padding: 16px 24px;
        font-family: 'Press Start 2P', 'Courier New', monospace;
        font-size: 12px;
        color: #dcfce7;
        text-shadow: 1px 1px 0 #14532d;
        background: linear-gradient(180deg, #166534 0%, #14532d 100%);
        border: 3px solid #22c55e;
        border-bottom-color: #166534;
        border-right-color: #166534;
        box-shadow: 4px 4px 0 #0a0a1e;
        cursor: pointer;
        transition: transform 0.1s;
    }

    .start-btn:hover {
        transform: translate(2px, 2px);
        box-shadow: 2px 2px 0 #0a0a1e;
        background: linear-gradient(180deg, #15803d 0%, #166534 100%);
    }

    .start-btn:active {
        transform: translate(4px, 4px);
        box-shadow: none;
    }

    .hint-text {
        font-size: 11px;
        color: #64748b;
        text-align: center;
        padding-bottom: 20px;
    }
</style>
