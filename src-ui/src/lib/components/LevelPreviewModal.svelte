<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import type { LevelInfo } from '$lib/types';
    import { getConceptPreview } from '$lib/data/concept-previews';

    interface Props {
        level: LevelInfo;
        onStart: () => void;
        onCancel: () => void;
    }

    let { level, onStart, onCancel }: Props = $props();

    // Get concept preview data
    let conceptData = $derived(getConceptPreview(level.concept));

    // Calculate progress display
    let progressText = $derived.by(() => {
        if (level.completed) return 'Completed';
        if (level.completed_quests > 0) return `${level.completed_quests}/${level.total_quests} quests done`;
        return 'Not started';
    });

    let statusClass = $derived.by(() => {
        if (level.completed) return 'completed';
        if (level.completed_quests > 0) return 'in-progress';
        return 'ready';
    });

    // Accessibility: Reference for focus management
    let startButton: HTMLButtonElement | undefined;

    // Handle keyboard events (Escape to close) - window level for reliability
    function handleGlobalKeydown(event: KeyboardEvent) {
        if (event.key === 'Escape') {
            event.preventDefault();
            onCancel();
        }
    }

    // Focus the start button when modal opens and bind escape handler
    onMount(() => {
        startButton?.focus();
        window.addEventListener('keydown', handleGlobalKeydown);
    });

    onDestroy(() => {
        window.removeEventListener('keydown', handleGlobalKeydown);
    });
</script>

<!-- Keyboard events handled via window-level listener for Escape key -->
<div class="modal-backdrop" onclick={onCancel} role="presentation">
    <!-- svelte-ignore a11y_no_noninteractive_tabindex a11y_click_events_have_key_events -->
    <div class="preview-modal" onclick={(e) => e.stopPropagation()} role="dialog" aria-modal="true" aria-labelledby="preview-title" tabindex="0">
        <!-- Header -->
        <header class="preview-header">
            <div class="level-badge">{level.id.replace('L', '')}</div>
            <div class="header-content">
                <h2 id="preview-title" class="level-title">{level.title}</h2>
                <p class="level-concept">{level.concept}</p>
            </div>
        </header>

        <!-- Concept Preview -->
        <section class="concept-section">
            <h3 class="section-title">{conceptData.title}</h3>
            <p class="concept-preview">{conceptData.preview}</p>
        </section>

        <!-- Skills -->
        <section class="skills-section">
            <h4 class="skills-label">You'll learn:</h4>
            <div class="skills-list">
                {#each conceptData.skills as skill}
                    <span class="skill-tag">{skill}</span>
                {/each}
            </div>
        </section>

        <!-- Stats -->
        <div class="stats-row">
            <div class="stat">
                <span class="stat-icon">📜</span>
                <span class="stat-value">{level.total_quests}</span>
                <span class="stat-label">quests</span>
            </div>
            <div class="stat">
                <span class="stat-icon">⭐</span>
                <span class="stat-value">{level.xp_reward}</span>
                <span class="stat-label">XP</span>
            </div>
            <div class="stat {statusClass}">
                <span class="stat-icon">{level.completed ? '✓' : level.completed_quests > 0 ? '◐' : '○'}</span>
                <span class="stat-value status-text">{progressText}</span>
            </div>
        </div>

        <!-- Actions -->
        <div class="actions">
            <button type="button" class="btn-cancel" onclick={onCancel}>
                <span aria-hidden="true">←</span> Back
            </button>
            <button type="button" class="btn-start" bind:this={startButton} onclick={onStart}>
                {level.completed ? 'REPLAY' : level.completed_quests > 0 ? 'CONTINUE' : 'START'} <span aria-hidden="true">→</span>
            </button>
        </div>
    </div>
</div>

<style>
    .modal-backdrop {
        position: fixed;
        inset: 0;
        background: rgba(10, 10, 20, 0.95);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 100;
        animation: fadeIn 0.2s ease-out;
    }

    @keyframes fadeIn {
        from { opacity: 0; }
        to { opacity: 1; }
    }

    .preview-modal {
        background: linear-gradient(180deg, #1a1a2e 0%, #16213e 100%);
        border: 4px solid #3a506b;
        border-top-color: #5a7090;
        border-left-color: #5a7090;
        box-shadow: 8px 8px 0 #0a0a1e;
        padding: 0;
        width: 90%;
        max-width: 420px;
        font-family: 'Press Start 2P', monospace;
        image-rendering: pixelated;
        animation: slideUp 0.2s ease-out;
    }

    @keyframes slideUp {
        from { transform: translateY(20px); opacity: 0; }
        to { transform: translateY(0); opacity: 1; }
    }

    /* Respect user's motion preferences */
    @media (prefers-reduced-motion: reduce) {
        .modal-backdrop {
            animation: none;
        }
        .preview-modal {
            animation: none;
        }
        .btn-cancel,
        .btn-start {
            transition: none;
        }
    }

    /* Header */
    .preview-header {
        display: flex;
        gap: 16px;
        padding: 20px;
        border-bottom: 3px solid #0f3460;
        background: rgba(0, 0, 0, 0.2);
    }

    .level-badge {
        width: 48px;
        height: 48px;
        background: linear-gradient(180deg, #fcd34d 0%, #fbbf24 100%);
        border: 3px solid #f59e0b;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 14px;
        color: #451a03;
        text-shadow: 1px 1px 0 rgba(255,255,255,0.3);
        flex-shrink: 0;
    }

    .header-content {
        flex: 1;
        min-width: 0;
    }

    .level-title {
        font-size: 12px;
        color: #fbbf24;
        text-shadow: 2px 2px 0 #92400e;
        margin: 0 0 8px 0;
        line-height: 1.3;
    }

    .level-concept {
        font-size: 8px;
        color: #94a3b8;
        margin: 0;
        text-transform: uppercase;
        letter-spacing: 1px;
    }

    /* Concept Section */
    .concept-section {
        padding: 16px 20px;
        border-bottom: 2px solid #0f3460;
    }

    .section-title {
        font-size: 10px;
        color: #06b6d4;
        margin: 0 0 10px 0;
        text-shadow: 1px 1px 0 #0e7490;
    }

    .concept-preview {
        font-size: 8px;
        color: #e2e8f0;
        line-height: 1.8;
        margin: 0;
    }

    /* Skills Section */
    .skills-section {
        padding: 14px 20px;
        border-bottom: 2px solid #0f3460;
    }

    .skills-label {
        font-size: 7px;
        color: #64748b;
        margin: 0 0 10px 0;
        text-transform: uppercase;
    }

    .skills-list {
        display: flex;
        flex-wrap: wrap;
        gap: 8px;
    }

    .skill-tag {
        font-size: 7px;
        padding: 4px 8px;
        background: #0f3460;
        border: 2px solid #3a506b;
        color: #a5f3fc;
    }

    /* Stats Row */
    .stats-row {
        display: flex;
        justify-content: space-around;
        padding: 14px 20px;
        border-bottom: 2px solid #0f3460;
        background: rgba(0, 0, 0, 0.15);
    }

    .stat {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 4px;
    }

    .stat-icon {
        font-size: 14px;
    }

    .stat-value {
        font-size: 10px;
        color: #e2e8f0;
    }

    .stat-label {
        font-size: 6px;
        color: #64748b;
        text-transform: uppercase;
    }

    .stat.completed .stat-icon,
    .stat.completed .stat-value {
        color: #22c55e;
    }

    .stat.in-progress .stat-icon,
    .stat.in-progress .stat-value {
        color: #06b6d4;
    }

    .status-text {
        font-size: 7px;
    }

    /* Actions */
    .actions {
        display: flex;
        gap: 12px;
        padding: 16px 20px;
    }

    .btn-cancel {
        flex: 1;
        background: linear-gradient(180deg, #334155 0%, #1e293b 100%);
        border: 3px solid #64748b;
        border-bottom-color: #334155;
        border-right-color: #334155;
        box-shadow: 3px 3px 0 #0a0a1e;
        padding: 10px 16px;
        font-family: 'Press Start 2P', monospace;
        font-size: 8px;
        color: #e2e8f0;
        cursor: pointer;
        transition: transform 0.1s;
    }

    .btn-cancel:hover {
        transform: translate(1px, 1px);
        box-shadow: 2px 2px 0 #0a0a1e;
    }

    .btn-start {
        flex: 2;
        background: linear-gradient(180deg, #166534 0%, #14532d 100%);
        border: 3px solid #22c55e;
        border-bottom-color: #166534;
        border-right-color: #166534;
        box-shadow: 3px 3px 0 #0a0a1e;
        padding: 10px 16px;
        font-family: 'Press Start 2P', monospace;
        font-size: 9px;
        color: #dcfce7;
        cursor: pointer;
        transition: transform 0.1s;
    }

    .btn-start:hover {
        transform: translate(1px, 1px);
        box-shadow: 2px 2px 0 #0a0a1e;
    }

    /* Responsive */
    @media (max-width: 480px) {
        .preview-modal {
            width: 95%;
        }

        .preview-header {
            padding: 16px;
        }

        .level-badge {
            width: 40px;
            height: 40px;
            font-size: 12px;
        }

        .level-title {
            font-size: 10px;
        }

        .stats-row {
            padding: 12px 16px;
        }

        .actions {
            padding: 12px 16px;
        }
    }
</style>
