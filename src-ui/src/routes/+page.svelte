<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import { getBackend, type Backend } from '$lib/backend';
    import type { Direction, CodeResult, RenderState, LevelInfo, LevelData, PlayerProgress, QuestInfo, PlayerAction } from '$lib/types';
    import GameWorld from '$lib/components/GameWorld.svelte';
    import CodeTerminal from '$lib/components/CodeTerminal.svelte';
    import GameHUD from '$lib/components/GameHUD.svelte';
    import Toast, { type ToastMessage } from '$lib/components/Toast.svelte';
    import MainMenu from '$lib/components/MainMenu.svelte';
    import WorldMap from '$lib/components/WorldMap.svelte';
    import Settings from '$lib/components/Settings.svelte';
    import ErrorBoundary from '$lib/components/ErrorBoundary.svelte';
    import LevelIntroModal from '$lib/components/LevelIntroModal.svelte';
    import LevelPreviewModal from '$lib/components/LevelPreviewModal.svelte';

    // Backend + state (Runes, no svelte/store)
    type GameScreen = 'boot' | 'world_map' | 'playing';
    let gameScreen = $state<GameScreen>('boot');
    let backend = $state<Backend | null>(null);
    let renderState = $state<RenderState | null>(null);
    let levels = $state<LevelInfo[]>([]);
    let currentLevelData = $state<LevelData | null>(null);
    let codeSubmitting = $state(false);
    let lastCodeResult = $state<CodeResult | null>(null);
    let levelTransitioning = $state(false);
    let uiStatus = $state<{ loading: boolean; status: string; error: string | null }>({
        loading: true,
        status: 'Booting Code Warrior...',
        error: null,
    });
    let currentHintIndex = $state(0);
    let tickUnsub: (() => void) | null = null;

    let codeDraft = $state('// Write your C spell here...\n#include <stdio.h>\n\nint main() {\n    printf("Hello, World!\\n");\n    return 0;\n}');
    let toastMessages = $state<ToastMessage[]>([]);
    let showSettings = $state(false); // Settings modal state
    let showLevelIntro = $state(false); // Level intro lesson modal
    let previewLevel = $state<LevelInfo | null>(null); // Level preview modal

    // Derived values
    let showMainMenu = $derived((renderState?.game_phase ?? 'main_menu') === 'main_menu' || !renderState?.current_level_id);
    let showTerminal = $derived(renderState?.show_terminal ?? false);
    // For function-based challenges, use user_template; otherwise use code_template
    let codeTemplate = $derived(
        currentLevelData?.user_template ?? currentLevelData?.code_template ?? codeDraft
    );
    let isLevelComplete = $derived(renderState?.game_phase === 'level_complete');
    let currentLevelId = $derived(renderState?.current_level_id ?? null);

    // Hint state
    let hints = $state<string[]>([]);
    let loadingHint = $state(false);

    // Progress state
    let playerProgress = $state<PlayerProgress | null>(null);

    // Quest state (auto-loaded based on active_quest_id from backend)
    let activeQuest = $state<QuestInfo | null>(null);
    let questLoadingInProgress = $state(false);

    // Derived: is this a multi-quest level?
    let isMultiQuestLevel = $derived((currentLevelData?.quests?.length ?? 0) > 0);
    // Active quest ID comes from render state (set when interacting with terminal)
    let activeQuestId = $derived(renderState?.active_quest_id ?? null);

    async function fetchProgress() {
        if (!backend) return;
        try {
            playerProgress = await backend.getProgress();
        } catch (e) {
            console.error('Failed to fetch progress:', e);
        }
    }

    async function handleRequestHint() {
        loadingHint = true;
        const hint = await getNextHint();
        if (hint) hints = [...hints, hint];
        else addInfoToast('No more hints available');
        loadingHint = false;
    }

    // Reset hints and auto-load quest when terminal opens with active_quest_id
    $effect(() => {
        if (showTerminal) {
            hints = [];
            lastCodeResult = null;
            // Auto-load quest if active_quest_id is set (multi-quest level)
            if (activeQuestId && backend && !questLoadingInProgress) {
                loadQuestById(activeQuestId);
            } else if (!activeQuestId && activeQuest) {
                // Quest was completed (server cleared active_quest_id) - reset local quest state
                activeQuest = null;
            }
        } else {
            // Terminal closed - reset quest
            activeQuest = null;
        }
    });

    // Load quest by ID (auto-triggered when interacting with a terminal that has quest_id)
    async function loadQuestById(questId: string) {
        if (!backend || questLoadingInProgress) return;
        questLoadingInProgress = true;
        try {
            const quest = await backend.loadQuest(questId);
            activeQuest = quest;
            hints = [];
            lastCodeResult = null;
        } catch (e) {
            console.error('Failed to load quest:', e);
            addInfoToast('Failed to load quest');
        } finally {
            questLoadingInProgress = false;
        }
    }

    // Get the next level ID
    function getNextLevelId(): string | null {
        const currentId = renderState?.current_level_id ?? null;
        if (!currentId) return null;
        const idx = levels.findIndex((l) => l.id === currentId);
        if (idx >= 0 && idx < levels.length - 1) return levels[idx + 1].id;
        return null;
    }

    async function handleNextLevel() {
        if (levelTransitioning) return; // Prevent double-click
        levelTransitioning = true;
        try {
            // Return to world map instead of auto-advancing
            gameScreen = 'world_map';
            await fetchProgress(); // Refresh progress
            addInfoToast('Returning to map...');
        } finally {
            levelTransitioning = false;
        }
    }

    onMount(() => {
        boot();
    });

    onDestroy(() => {
        cleanup();
    });

    function handleMove(event: CustomEvent<{ direction: Direction }>) {
        sendAction({ type: 'move', direction: event.detail.direction });
    }

    function handleInteract() {
        sendAction({ type: 'interact' });
    }

    function handleStartMenu() {
        const first = levels[0];
        if (first) startLevel(first.id);
    }

    function handleNewGame() {
        const first = levels[0];
        if (first) {
            startLevel(first.id);
            addInfoToast('Starting new quest...');
        } else {
            console.warn('[handleNewGame] No levels available yet');
            addInfoToast('Loading levels, please wait...');
        }
    }

    function handleContinue() {
        const first = levels[0];
        if (first) {
            startLevel(first.id);
            addInfoToast('Continuing quest...');
        } else {
            console.warn('[handleContinue] No levels available yet');
            addInfoToast('Loading levels, please wait...');
        }
    }

    function handleSelectLevel(levelId: string) {
        // Find the level and show preview modal
        const level = levels.find((l) => l.id === levelId);
        if (level && !level.locked) {
            previewLevel = level;
        }
    }

    async function handleStartFromPreview() {
        if (!previewLevel) return;
        const levelId = previewLevel.id;
        previewLevel = null; // Close preview
        await startLevel(levelId);
        gameScreen = 'playing';
    }

    function handleCancelPreview() {
        previewLevel = null;
    }

    async function handleBackToMap() {
        gameScreen = 'world_map';
        await fetchProgress(); // Refresh progress to show updated completion
    }

    async function handleCodeSubmit(event: CustomEvent<{ code: string; testOnly?: boolean; questId?: string }>) {
        codeDraft = event.detail.code;
        const testOnly = event.detail.testOnly ?? false;
        const questId = event.detail.questId ?? activeQuestId;

        if (questId) {
            // Multi-quest level - submit to specific quest
            await submitQuestCode(codeDraft, questId, testOnly);
        } else {
            // Single challenge level
            await submitCode(codeDraft, testOnly);
        }
        if (lastCodeResult) addToast(lastCodeResult);
    }

    function handleTerminalClose() {
        sendAction({ type: 'resume' });
    }

    function addToast(result: CodeResult) {
        const toast: ToastMessage = {
            id: crypto.randomUUID(),
            type: result.success ? 'success' : 'error',
            message: result.success ? 'Spell cast successfully!' : 'Spell failed',
            details: result.compile_error || result.feedback || result.stdout || undefined,
        };
        toastMessages = [...toastMessages, toast];
    }

    function addInfoToast(message: string, details?: string) {
        toastMessages = [...toastMessages, { id: crypto.randomUUID(), type: 'info', message, details }];
    }

    function dismissToast(id: string) {
        toastMessages = toastMessages.filter((t) => t.id !== id);
    }

    // ===== Backend wiring =====
    async function boot() {
        uiStatus = { ...uiStatus, loading: true, status: 'Initializing backend...', error: null };
        try {
            backend = await getBackend();
            renderState = await backend.initGame();
            uiStatus = { ...uiStatus, status: 'Loading levels...' };
            await hydrateLevels();
            await fetchProgress();
            await bindEvents();
            // Only set loading false after levels are fetched
            uiStatus = { ...uiStatus, loading: false, status: 'Ready', error: null };
            gameScreen = 'world_map'; // Show world map after boot
        } catch (err) {
            console.error('[BOOT] error', err);
            uiStatus = { ...uiStatus, loading: false, error: normalizeError(err), status: 'Error' };
        }
    }

    async function hydrateLevels() {
        if (!backend) return;
        try {
            levels = await backend.getAvailableLevels();
        } catch (err) {
            uiStatus = { ...uiStatus, error: uiStatus.error ?? normalizeError(err) };
        }
    }

    async function bindEvents() {
        if (!backend) return;
        if (!tickUnsub) tickUnsub = await backend.onGameTick((state) => (renderState = state));
    }

    async function startLevel(levelId: string) {
        if (!backend) return;
        uiStatus = { ...uiStatus, loading: true, status: `Loading ${levelId}...`, error: null };
        try {
            await backend.loadLevel(levelId);
            currentLevelData = await backend.getLevelData();
            console.log(`[startLevel] Level data loaded:`, currentLevelData?.id, `theme:`, currentLevelData?.theme);
            renderState = await backend.getRenderState();
            currentHintIndex = 0;
            lastCodeResult = null;
            // Show intro lesson modal if level has lesson content
            showLevelIntro = !!(currentLevelData?.lesson);
            uiStatus = { ...uiStatus, loading: false, status: `Level ${levelId} loaded`, error: null };
        } catch (err) {
            uiStatus = { ...uiStatus, loading: false, error: normalizeError(err) };
        }
    }

    function dismissLevelIntro() {
        showLevelIntro = false;
    }

    async function getNextHint(): Promise<string | null> {
        if (!backend) return null;
        const idx = currentHintIndex;
        try {
            const hint = await backend.getHint(idx);
            currentHintIndex = idx + 1;
            return hint;
        } catch {
            return null;
        }
    }

    async function sendAction(action: PlayerAction) {
        if (!backend) return;
        uiStatus = { ...uiStatus, error: null };
        try {
            renderState = await backend.processAction(action);
        } catch (err) {
            uiStatus = { ...uiStatus, error: normalizeError(err) };
        }
    }

    async function submitCode(code: string, testOnly: boolean = false) {
        if (!backend) return;
        uiStatus = { ...uiStatus, error: null };
        codeSubmitting = true;
        try {
            const result = await backend.submitCode(code, testOnly);
            lastCodeResult = result;
            if (result.success) {
                if (result.render_state) {
                    renderState = result.render_state;
                } else {
                    renderState = await backend.getRenderState();
                }
            }
        } catch (err) {
            uiStatus = { ...uiStatus, error: normalizeError(err) };
        } finally {
            codeSubmitting = false;
        }
    }

    async function submitQuestCode(code: string, questId: string, testOnly: boolean = false) {
        if (!backend) return;
        uiStatus = { ...uiStatus, error: null };
        codeSubmitting = true;
        try {
            const result = await backend.submitQuestCode(code, questId, testOnly);
            lastCodeResult = result;
            if (result.success) {
                if (result.render_state) {
                    renderState = result.render_state;
                } else {
                    renderState = await backend.getRenderState();
                }
                // Reload quest to update completion status
                if (!testOnly && activeQuestId) {
                    await loadQuestById(activeQuestId);
                }
            }
        } catch (err) {
            uiStatus = { ...uiStatus, error: normalizeError(err) };
        } finally {
            codeSubmitting = false;
        }
    }

    function cleanup() {
        tickUnsub?.();
        backend?.cleanup();
    }

    function normalizeError(err: unknown): string {
        if (err instanceof Error) return err.message;
        return typeof err === 'string' ? err : 'Unknown error';
    }
</script>

<svelte:head>
    <title>Code Warrior: C Mastery</title>
</svelte:head>

{#if uiStatus.loading || uiStatus.error}
    <div class="status-banner {uiStatus.error ? 'error' : 'info'}">
        {uiStatus.error ?? uiStatus.status}
    </div>
{/if}

<ErrorBoundary>
{#if gameScreen === 'boot'}
    <!-- Boot screen while loading -->
    <div class="fixed inset-0 flex items-center justify-center bg-slate-950">
        <div class="text-center">
            <h1 class="text-2xl text-amber-400 font-['Press_Start_2P'] mb-4">CODE WARRIOR</h1>
            <p class="text-slate-400 text-sm">Loading...</p>
        </div>
    </div>
{:else if gameScreen === 'world_map'}
    <div class="map-shell">
        <div class="map-help">
            <span class="dot locked"></span>
            <span class="help-text">Locked until you finish the previous level.</span>
        </div>
        <WorldMap
            {levels}
            progress={playerProgress}
            onSelectLevel={handleSelectLevel}
            onSettings={() => showSettings = true}
        />
    </div>
    {#if previewLevel}
        <LevelPreviewModal
            level={previewLevel}
            onStart={handleStartFromPreview}
            onCancel={handleCancelPreview}
        />
    {/if}
{:else}
    <!-- gameScreen === 'playing' -->
    <GameWorld
        renderState={renderState}
        codeSuccess={lastCodeResult?.success ?? false}
        xpGained={0}
        theme={currentLevelData?.theme ?? null}
        on:move={handleMove}
        on:interact={handleInteract}
    >
            <!-- HUD Overlay -->
            <GameHUD player={renderState?.player ?? null} currentLevelId={currentLevelId} onBackToMap={handleBackToMap} />

            <!-- Code Terminal Modal -->
            {#if showTerminal}
                {#if activeQuest}
                    <!-- Multi-quest level: CodeTerminal with auto-loaded quest -->
                    <CodeTerminal
                        initialCode={activeQuest.user_template}
                        submitting={codeSubmitting}
                        output={lastCodeResult ? {
                            success: lastCodeResult.success,
                            stdout: lastCodeResult.stdout,
                            stderr: lastCodeResult.stderr,
                            compile_error: lastCodeResult.compile_error ?? undefined,
                            message: lastCodeResult.feedback,
                            feedback: lastCodeResult.feedback,
                            test_results: lastCodeResult.test_results
                        } : null}
                        challenge={activeQuest.description}
                        hints={hints}
                        {loadingHint}
                        onClose={handleTerminalClose}
                        onRequestHint={handleRequestHint}
                        functionSignature={activeQuest.function_signature
                            ? `${activeQuest.function_signature.return_type} ${activeQuest.function_signature.name}(${activeQuest.function_signature.parameters?.map((p: {type: string, name: string}) => `${p.type} ${p.name}`).join(', ') ?? ''})`
                            : ''}
                        questId={activeQuestId}
                        questTitle={activeQuest.title}
                        questDescription={activeQuest.description}
                        questXpReward={activeQuest.xp_reward}
                        questTeaching={activeQuest.teaching ?? null}
                        on:submit={handleCodeSubmit}
                    />
                {:else if activeQuestId || questLoadingInProgress}
                    <!-- Quest loading - show loading state when quest_id exists but quest not loaded yet -->
                    <!-- This prevents race condition where legacy terminal renders before quest loads -->
                    <div class="fixed inset-0 z-50 flex items-center justify-center bg-slate-950/90">
                        <div class="quest-loading">
                            <p class="text-amber-400 font-['Press_Start_2P'] text-sm">Loading quest...</p>
                        </div>
                    </div>
                {:else if isMultiQuestLevel}
                    <!-- Multi-quest level: quest just completed, show success and close option -->
                    <div class="fixed inset-0 z-50 flex items-center justify-center bg-slate-950/90">
                        <div class="pixel-modal">
                            <div class="text-center mb-4">
                                <span class="text-4xl" style="filter: drop-shadow(2px 2px 0 #000);">✓</span>
                            </div>
                            <h2 class="pixel-title text-center">QUEST COMPLETE!</h2>
                            <p class="text-sm text-slate-300 mb-4 text-center">
                                {lastCodeResult?.feedback ?? 'Great work!'}
                            </p>
                            <div class="flex flex-col gap-3 mt-6">
                                <button onclick={handleTerminalClose} class="pixel-button w-full">
                                    CONTINUE
                                </button>
                            </div>
                        </div>
                    </div>
                {:else}
                    <!-- Single-challenge level (only when terminal has no quest_id) -->
                    <CodeTerminal
                        initialCode={codeTemplate}
                        submitting={codeSubmitting}
                        output={lastCodeResult ? {
                            success: lastCodeResult.success,
                            stdout: lastCodeResult.stdout,
                            stderr: lastCodeResult.stderr,
                            compile_error: lastCodeResult.compile_error ?? undefined,
                            message: lastCodeResult.feedback,
                            feedback: lastCodeResult.feedback,
                            test_results: lastCodeResult.test_results
                        } : null}
                        challenge={currentLevelData?.description ?? 'Complete the challenge'}
                        expectedOutput={currentLevelData?.challenges?.[0]?.expected_output}
                        {hints}
                        {loadingHint}
                        onClose={handleTerminalClose}
                        onRequestHint={handleRequestHint}
                        lesson={currentLevelData?.lesson ?? null}
                        functionSignature={currentLevelData?.function_signature
                            ? `${currentLevelData.function_signature.return_type} ${currentLevelData.function_signature.name}(${currentLevelData.function_signature.parameters.map(p => `${p.type} ${p.name}`).join(', ')})`
                            : ''}
                        on:submit={handleCodeSubmit}
                    />
                {/if}
            {/if}

            <!-- Level Complete Modal (Pixel Art Style) -->
            {#if isLevelComplete}
                <div class="fixed inset-0 z-50 flex items-center justify-center bg-slate-950/95">
                    <div class="pixel-modal">
                        <!-- Decorative sword icon -->
                        <div class="text-center mb-4">
                            <span class="text-4xl" style="filter: drop-shadow(2px 2px 0 #000);">&#9876;</span>
                        </div>

                        <h2 class="pixel-title text-center">LEVEL COMPLETE!</h2>
                        <p class="text-sm text-slate-300 mb-4 text-center">
                            You've conquered <span class="text-emerald-400 font-bold">{currentLevelId}</span>
                        </p>

                        <!-- Quest Progress -->
                        {#if currentLevelData?.quests?.length}
                            <p class="text-xs text-cyan-400 mb-4 text-center">
                                All {currentLevelData.quests.length} quests completed!
                            </p>
                        {/if}

                        <!-- XP Info Box -->
                        <div class="pixel-reward-box mb-6">
                            <div class="flex items-center justify-center gap-2">
                                <span class="text-amber-400 text-lg">&#9830;</span>
                                <span class="text-amber-300 text-lg font-bold">Total: {renderState?.player?.xp ?? 0} XP</span>
                            </div>
                        </div>

                        <!-- Action Buttons -->
                        <div class="flex flex-col gap-3">
                            <button
                                onclick={handleTerminalClose}
                                class="pixel-button-secondary w-full"
                            >
                                CONTINUE EXPLORING
                            </button>
                            <button
                                onclick={handleNextLevel}
                                disabled={levelTransitioning}
                                class="pixel-button w-full"
                            >
                                {levelTransitioning ? 'LOADING...' : 'RETURN TO MAP'}
                            </button>
                        </div>
                    </div>
                </div>
            {/if}

        <!-- Toast Notifications -->
        <Toast messages={toastMessages} onDismiss={dismissToast} />

        <!-- Level Intro Lesson Modal -->
        {#if showLevelIntro}
            <LevelIntroModal level={currentLevelData} onStart={dismissLevelIntro} />
        {/if}

            </GameWorld>
{/if}
</ErrorBoundary>

<!-- Settings Modal -->
<Settings
    isOpen={showSettings}
    onClose={() => showSettings = false}
    backend={backend}
    onLoadGame={(state) => { renderState = state; }}
/>

<style>
    /* Pixel Art Modal Styles */
    :global(.pixel-modal) {
        background: linear-gradient(180deg, #1a1a2e 0%, #16213e 100%);
        border: 4px solid #0f3460;
        border-top-color: #3a506b;
        border-left-color: #3a506b;
        box-shadow:
            inset 0 0 0 2px #0a0a1e,
            8px 8px 0 #0a0a1e;
        padding: 24px 32px;
        min-width: 320px;
        max-width: 400px;
        image-rendering: pixelated;
    }

    :global(.pixel-title) {
        font-family: 'Press Start 2P', 'Courier New', monospace;
        font-size: 16px;
        color: #fbbf24;
        text-shadow: 2px 2px 0 #92400e;
        margin-bottom: 8px;
        letter-spacing: 2px;
    }

    :global(.pixel-reward-box) {
        background: #0a0a1e;
        border: 3px solid #0f3460;
        border-top-color: #050510;
        border-left-color: #050510;
        padding: 12px 16px;
        text-align: center;
    }

    :global(.pixel-button) {
        background: linear-gradient(180deg, #166534 0%, #14532d 100%);
        border: 3px solid #22c55e;
        border-bottom-color: #166534;
        border-right-color: #166534;
        box-shadow: 4px 4px 0 #0a0a1e;
        padding: 12px 24px;
        font-family: 'Press Start 2P', 'Courier New', monospace;
        font-size: 10px;
        color: #dcfce7;
        text-shadow: 1px 1px 0 #14532d;
        cursor: pointer;
        transition: transform 0.1s;
        text-transform: uppercase;
        letter-spacing: 1px;
    }

    :global(.pixel-button:hover) {
        transform: translate(2px, 2px);
        box-shadow: 2px 2px 0 #0a0a1e;
    }

    :global(.pixel-button:active) {
        transform: translate(4px, 4px);
        box-shadow: none;
    }

    :global(.pixel-button-secondary) {
        background: linear-gradient(180deg, #334155 0%, #1e293b 100%);
        border: 3px solid #64748b;
        border-bottom-color: #334155;
        border-right-color: #334155;
        box-shadow: 4px 4px 0 #0a0a1e;
        padding: 12px 24px;
        font-family: 'Press Start 2P', 'Courier New', monospace;
        font-size: 10px;
        color: #e2e8f0;
        text-shadow: 1px 1px 0 #1e293b;
        cursor: pointer;
        transition: transform 0.1s;
        text-transform: uppercase;
        letter-spacing: 1px;
    }

    :global(.pixel-button-secondary:hover) {
        transform: translate(2px, 2px);
        box-shadow: 2px 2px 0 #0a0a1e;
    }

    :global(.pixel-button-secondary:active) {
        transform: translate(4px, 4px);
        box-shadow: none;
    }

    .status-banner {
        position: fixed;
        top: 12px;
        left: 50%;
        transform: translateX(-50%);
        padding: 10px 16px;
        border: 2px solid #0f172a;
        border-radius: 6px;
        font-family: 'Press Start 2P', 'Courier New', monospace;
        font-size: 10px;
        letter-spacing: 1px;
        z-index: 60;
        box-shadow: 0 6px 20px rgba(0, 0, 0, 0.35);
    }

    .status-banner.info {
        background: linear-gradient(180deg, #0ea5e9 0%, #0284c7 100%);
        color: #e0f2fe;
        border-color: #0369a1;
        text-shadow: 1px 1px 0 #075985;
    }

    .status-banner.error {
        background: linear-gradient(180deg, #ef4444 0%, #b91c1c 100%);
        color: #fee2e2;
        border-color: #7f1d1d;
        text-shadow: 1px 1px 0 #7f1d1d;
    }

    .map-shell {
        display: flex;
        flex-direction: column;
        gap: 12px;
        padding: 8px 12px;
    }

    .map-help {
        display: inline-flex;
        align-items: center;
        gap: 8px;
        background: linear-gradient(180deg, rgba(30,41,59,0.9), rgba(15,23,42,0.9));
        border: 1px solid #334155;
        border-radius: 10px;
        padding: 8px 12px;
        color: #e2e8f0;
        font-size: 12px;
        box-shadow: 0 8px 24px rgba(0,0,0,0.35);
    }

    .map-help .dot {
        width: 10px;
        height: 10px;
        border-radius: 999px;
        background: #f97316;
        box-shadow: 0 0 0 2px rgba(249,115,22,0.25);
    }

    .map-help .help-text {
        letter-spacing: 0.2px;
    }

    /* Quest Loading Styles */
    :global(.quest-loading) {
        background: linear-gradient(180deg, #1a1a2e 0%, #16213e 100%);
        border: 4px solid #0f3460;
        border-top-color: #3a506b;
        border-left-color: #3a506b;
        box-shadow:
            inset 0 0 0 2px #0a0a1e,
            8px 8px 0 #0a0a1e;
        padding: 24px 32px;
        animation: pulse 1.5s ease-in-out infinite;
    }

    @keyframes pulse {
        0%, 100% { opacity: 1; }
        50% { opacity: 0.7; }
    }
</style>
