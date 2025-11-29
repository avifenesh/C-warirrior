<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import { getBackend, type Backend } from '$lib/backend';
    import type { Direction, CodeResult, RenderState, LevelInfo, LevelData, PlayerProgress } from '$lib/types';
    import GameWorld from '$lib/components/GameWorld.svelte';
    import CodeTerminal from '$lib/components/CodeTerminal.svelte';
    import GameHUD from '$lib/components/GameHUD.svelte';
    import Toast, { type ToastMessage } from '$lib/components/Toast.svelte';
    import MainMenu from '$lib/components/MainMenu.svelte';
    // Agent 2's new components
    import Settings from '$lib/components/Settings.svelte';
    import ErrorBoundary from '$lib/components/ErrorBoundary.svelte';
    import ProgressTracker from '$lib/components/ProgressTracker.svelte';

    // Backend + state (Runes, no svelte/store)
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
    let errorUnsub: (() => void) | null = null;
    let codeUnsub: (() => void) | null = null;
    let levelCompleteUnsub: (() => void) | null = null;

    let codeDraft = $state('// Write your C spell here...\n#include <stdio.h>\n\nint main() {\n    printf("Hello, World!\\n");\n    return 0;\n}');
    let toastMessages = $state<ToastMessage[]>([]);
    let toastCounter = 0; // Unique counter for toast IDs
    let showSettings = $state(false); // Settings modal state

    // Derived values
    let showMainMenu = $derived((renderState?.game_phase ?? 'main_menu') === 'main_menu' || !renderState?.current_level_id);
    let showTerminal = $derived(renderState?.show_terminal ?? false);
    let codeTemplate = $derived(currentLevelData?.code_template ?? codeDraft);
    let isLevelComplete = $derived(renderState?.game_phase === 'level_complete');
    let currentLevelId = $derived(renderState?.current_level_id ?? null);

    // Hint state
    let hints = $state<string[]>([]);
    let loadingHint = $state(false);

    // Progress state
    let playerProgress = $state<PlayerProgress | null>(null);

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

    // Reset hints and ensure level data is loaded when terminal opens
    $effect(() => {
        if (showTerminal) {
            hints = [];
            // Ensure currentLevelData is loaded (handles race condition during level start)
            if (!currentLevelData && backend) {
                backend.getLevelData().then((data) => {
                    currentLevelData = data;
                }).catch(() => {
                    // Silently fail - data may already be loading via startLevel
                });
            }
        }
    });

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

        const nextId = getNextLevelId();
        if (nextId) {
            levelTransitioning = true;
            try {
                await startLevel(nextId);
                addInfoToast(`Starting ${nextId}...`);
            } finally {
                levelTransitioning = false;
            }
        } else {
            addInfoToast('Congratulations! You completed all levels!');
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

    async function handleCodeSubmit(event: CustomEvent<{ code: string }>) {
        codeDraft = event.detail.code;
        await submitCode(codeDraft);
        if (lastCodeResult) addToast(lastCodeResult);
    }

    function handleTerminalClose() {
        sendAction({ type: 'resume' });
    }

    function addToast(result: CodeResult) {
        toastCounter++;
        const toast: ToastMessage = {
            id: `toast-${toastCounter}`,
            type: result.success ? 'success' : 'error',
            message: result.success ? 'Spell cast successfully!' : 'Spell failed',
            details: result.compile_error || result.feedback || result.stdout || undefined,
        };
        toastMessages = [...toastMessages, toast];
    }

    function addInfoToast(message: string, details?: string) {
        toastCounter++;
        toastMessages = [...toastMessages, { id: `toast-${toastCounter}`, type: 'info', message, details }];
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
            uiStatus = { ...uiStatus, loading: false, status: 'Main Menu', error: null };
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
        if (!errorUnsub) errorUnsub = await backend.onGameError((error) => (uiStatus = { ...uiStatus, error: error.message }));
        if (!codeUnsub) codeUnsub = await backend.onCodeOutput((_o) => {});
        if (!levelCompleteUnsub) levelCompleteUnsub = await backend.onLevelComplete((_e) => {});
    }

    async function startLevel(levelId: string) {
        if (!backend) return;
        uiStatus = { ...uiStatus, loading: true, status: `Loading ${levelId}...`, error: null };
        try {
            await backend.loadLevel(levelId);
            currentLevelData = await backend.getLevelData();
            renderState = await backend.getRenderState();
            currentHintIndex = 0;
            lastCodeResult = null;
            uiStatus = { ...uiStatus, loading: false, status: `Level ${levelId} loaded`, error: null };
        } catch (err) {
            uiStatus = { ...uiStatus, loading: false, error: normalizeError(err) };
        }
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

    async function sendAction(action: any) {
        if (!backend) return;
        uiStatus = { ...uiStatus, error: null };
        try {
            renderState = await backend.processAction(action);
        } catch (err) {
            uiStatus = { ...uiStatus, error: normalizeError(err) };
        }
    }

    async function submitCode(code: string) {
        if (!backend) return;
        uiStatus = { ...uiStatus, error: null };
        codeSubmitting = true;
        try {
            const result = await backend.submitCode(code);
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

    function cleanup() {
        tickUnsub?.();
        errorUnsub?.();
        codeUnsub?.();
        levelCompleteUnsub?.();
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
{#if showMainMenu}
    <MainMenu
        onNewGame={handleNewGame}
        onContinue={handleContinue}
        onSettings={() => showSettings = true}
        ready={!uiStatus.loading && levels.length > 0}
    />
{:else}
    <GameWorld
        renderState={renderState}
        codeSuccess={lastCodeResult?.success ?? false}
        xpGained={0}
        on:move={handleMove}
        on:interact={handleInteract}
    >
            <!-- HUD Overlay -->
            <GameHUD player={renderState?.player ?? null} currentLevelId={currentLevelId} />

            <!-- Code Terminal Modal -->
            {#if showTerminal}
                <CodeTerminal
                    initialCode={codeTemplate}
                    submitting={codeSubmitting}
                    output={lastCodeResult ? {
                        success: lastCodeResult.success,
                        stdout: lastCodeResult.stdout,
                        stderr: lastCodeResult.stderr,
                        compile_error: lastCodeResult.compile_error ?? undefined,
                        message: lastCodeResult.feedback
                    } : null}
                    challenge={currentLevelData?.description ?? 'Complete the challenge'}
                    expectedOutput={currentLevelData?.challenges?.[0]?.expected_output}
                    {hints}
                    {loadingHint}
                    onClose={handleTerminalClose}
                    onRequestHint={handleRequestHint}
                    on:submit={handleCodeSubmit}
                />
            {/if}

            <!-- Level Complete Modal (Pixel Art Style) -->
            {#if isLevelComplete}
                <div class="fixed inset-0 z-50 flex items-center justify-center bg-slate-950/95">
                    <div class="pixel-modal">
                        <!-- Decorative sword icon -->
                        <div class="text-center mb-4">
                            <span class="text-4xl" style="filter: drop-shadow(2px 2px 0 #000);">&#9876;</span>
                        </div>

                        <h2 class="pixel-title text-center">QUEST COMPLETE!</h2>
                        <p class="text-sm text-slate-300 mb-4 text-center">
                            You've conquered <span class="text-emerald-400 font-bold">{currentLevelId}</span>
                        </p>

                        <!-- XP Reward Box -->
                        <div class="pixel-reward-box mb-6">
                            <div class="flex items-center justify-center gap-2">
                                <span class="text-amber-400 text-lg">&#9830;</span>
                                <span class="text-amber-300 text-lg font-bold">+{renderState?.player?.xp ?? 0} XP</span>
                            </div>
                        </div>

                        <button
                            onclick={handleNextLevel}
                            disabled={levelTransitioning}
                            class="pixel-button w-full"
                        >
                            {levelTransitioning ? 'LOADING...' : (getNextLevelId() ? 'CONTINUE QUEST' : 'RETURN TO VILLAGE')}
                        </button>
                    </div>
                </div>
            {/if}

        <!-- Toast Notifications -->
        <Toast messages={toastMessages} onDismiss={dismissToast} />

        <!-- Progress Tracker (bottom-left) -->
        <div class="fixed bottom-4 left-4 z-40">
            <ProgressTracker
                currentXP={playerProgress?.total_xp ?? renderState?.player?.xp ?? 0}
                totalXP={levels.reduce((sum, l) => sum + l.xp_reward, 0) || 7075}
                currentLevel={currentLevelId ?? 'L01'}
                completedLevels={playerProgress?.completed_levels ?? []}
                totalLevels={levels.length || 15}
                nextLevel={getNextLevelId()}
            />
        </div>

        <!-- Settings Button (top-right) -->
        <button
            class="fixed top-4 right-4 z-40 settings-btn"
            onclick={() => showSettings = true}
            title="Settings"
        >
            &#9881;
        </button>
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

    .settings-btn {
        background: rgba(26, 26, 46, 0.9);
        border: 2px solid #888;
        color: #e0e0e0;
        width: 44px;
        height: 44px;
        font-size: 24px;
        cursor: pointer;
        transition: all 0.2s;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .settings-btn:hover {
        border-color: #00fff5;
        color: #00fff5;
        box-shadow: 0 0 10px rgba(0, 255, 245, 0.3);
    }
</style>
