<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import { get } from 'svelte/store';
    import { gameStore } from '$lib/stores/game.svelte';
    import type { Direction, CodeResult } from '$lib/types';
    import GameWorld from '$lib/components/GameWorld.svelte';
    import CodeTerminal from '$lib/components/CodeTerminal.svelte';
    import GameHUD from '$lib/components/GameHUD.svelte';
    import Toast, { type ToastMessage } from '$lib/components/Toast.svelte';
    import SaveLoad from '$lib/components/SaveLoad.svelte';
    import LevelSelect from '$lib/components/LevelSelect.svelte';
    import Menu from '$lib/components/Menu.svelte';
    import MainMenu from '$lib/components/MainMenu.svelte';

    const game = gameStore;

    let codeDraft = $state('// Write your C spell here...\n#include <stdio.h>\n\nint main() {\n    printf("Hello, World!\\n");\n    return 0;\n}');
    let toastMessages = $state<ToastMessage[]>([]);
    let toastCounter = 0; // Unique counter for toast IDs
    let saveSlots = $state([
        {
            id: 'slot1',
            name: 'Slot 1',
            timestamp: 'Empty',
            progress: '---',
            empty: true,
        },
        {
            id: 'slot2',
            name: 'Slot 2',
            timestamp: 'Empty',
            progress: '---',
            empty: true,
        },
        {
            id: 'slot3',
            name: 'Slot 3',
            timestamp: 'Empty',
            progress: '---',
            empty: true,
        },
    ]);

    // Subscribe to stores
    const { renderState, currentLevelId, codeSubmitting, lastCodeResult, levels, currentLevelData, phase } = game;

    // Computed: should show main menu?
    let showMainMenu = $derived($phase === 'main_menu' || !$currentLevelId);

    // Computed: should show terminal?
    let showTerminal = $derived($renderState?.show_terminal ?? false);

    // Get code template from level data, or use codeDraft
    let codeTemplate = $derived($currentLevelData?.code_template ?? codeDraft);

    // Hint state
    let hints = $state<string[]>([]);
    let loadingHint = $state(false);

    async function handleRequestHint() {
        loadingHint = true;
        const hint = await game.getNextHint();
        if (hint) {
            hints = [...hints, hint];
        } else {
            addInfoToast('No more hints available');
        }
        loadingHint = false;
    }

    // Reset hints when terminal opens
    $effect(() => {
        if (showTerminal) {
            hints = [];
        }
    });

    // Computed: is level complete?
    let isLevelComplete = $derived($renderState?.game_phase === 'level_complete');

    // Get the next level ID
    function getNextLevelId(): string | null {
        const currentId = $currentLevelId;
        if (!currentId) return null;
        const levelList = get(levels);
        const currentIndex = levelList.findIndex(l => l.id === currentId);
        if (currentIndex >= 0 && currentIndex < levelList.length - 1) {
            return levelList[currentIndex + 1].id;
        }
        return null;
    }

    async function handleNextLevel() {
        const nextId = getNextLevelId();
        if (nextId) {
            await game.startLevel(nextId);
            addInfoToast(`Starting ${nextId}...`);
        } else {
            addInfoToast('Congratulations! You completed all levels!');
        }
    }

    onMount(() => {
        game.boot();
    });

    onDestroy(() => {
        game.cleanup();
    });

    function handleMove(event: CustomEvent<{ direction: Direction }>) {
        game.sendAction({ type: 'move', direction: event.detail.direction });
    }

    function handleInteract() {
        game.sendAction({ type: 'interact' });
    }

    function handleStartMenu() {
        const first = get(levels)[0];
        if (first) {
            game.startLevel(first.id);
        }
    }

    function handleNewGame() {
        const first = get(levels)[0];
        if (first) {
            game.startLevel(first.id);
            addInfoToast('Starting new quest...');
        }
    }

    function handleContinue() {
        // For now, continue does the same as new game
        // TODO: Load last played level from save data
        const first = get(levels)[0];
        if (first) {
            game.startLevel(first.id);
            addInfoToast('Continuing quest...');
        }
    }

    async function handleCodeSubmit(event: CustomEvent<{ code: string }>) {
        codeDraft = event.detail.code;
        await game.submitCode(codeDraft);

        // Show toast after submission completes
        const result = get(lastCodeResult);
        if (result) {
            addToast(result);
        }
    }


    function handleTerminalClose() {
        // Exit coding mode when terminal is closed
        game.sendAction({ type: 'resume' });
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
        toastMessages = [...toastMessages, {
            id: `toast-${toastCounter}`,
            type: 'info',
            message,
            details,
        }];
    }

    function dismissToast(id: string) {
        toastMessages = toastMessages.filter((t) => t.id !== id);
    }

    function handleSave(event: CustomEvent<{ id: string }>) {
        const now = new Date().toLocaleString();
        saveSlots = saveSlots.map((slot) =>
            slot.id === event.detail.id
                ? {
                      ...slot,
                      timestamp: now,
                      progress: $currentLevelId ? `At ${$currentLevelId}` : 'In progress',
                      empty: false,
                  }
                : slot
        );
    }

    function handleLoad(event: CustomEvent<{ id: string }>) {
        addInfoToast(`Load slot ${event.detail.id} (UI only)`);
    }

    function handleDelete(event: CustomEvent<{ id: string }>) {
        saveSlots = saveSlots.map((slot) =>
            slot.id === event.detail.id
                ? { ...slot, timestamp: 'Empty', progress: '---', empty: true }
                : slot
        );
    }
</script>

<svelte:head>
    <title>Code Warrior: C Mastery</title>
</svelte:head>

{#if showMainMenu}
    <MainMenu
        onNewGame={handleNewGame}
        onContinue={handleContinue}
    />
{:else}
    <GameWorld
        renderState={$renderState}
        codeSuccess={$lastCodeResult?.success ?? false}
        xpGained={0}
        on:move={handleMove}
        on:interact={handleInteract}
    >
            <!-- HUD Overlay -->
            <GameHUD player={$renderState?.player ?? null} currentLevelId={$currentLevelId} />

            <!-- Code Terminal Modal -->
            {#if showTerminal}
                <CodeTerminal
                    initialCode={codeTemplate}
                    submitting={$codeSubmitting}
                    output={$lastCodeResult ? {
                        success: $lastCodeResult.success,
                        stdout: $lastCodeResult.stdout,
                        stderr: $lastCodeResult.stderr,
                        compile_error: $lastCodeResult.compile_error ?? undefined,
                        message: $lastCodeResult.feedback
                    } : null}
                    challenge={$currentLevelData?.description ?? 'Complete the challenge'}
                    expectedOutput={$currentLevelData?.challenges?.[0]?.expected_output}
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
                            You've conquered <span class="text-emerald-400 font-bold">{$currentLevelId}</span>
                        </p>

                        <!-- XP Reward Box -->
                        <div class="pixel-reward-box mb-6">
                            <div class="flex items-center justify-center gap-2">
                                <span class="text-amber-400 text-lg">&#9830;</span>
                                <span class="text-amber-300 text-lg font-bold">+{$renderState?.player?.xp ?? 0} XP</span>
                            </div>
                        </div>

                        <button
                            onclick={handleNextLevel}
                            class="pixel-button w-full"
                        >
                            {getNextLevelId() ? 'CONTINUE QUEST' : 'RETURN TO VILLAGE'}
                        </button>
                    </div>
                </div>
            {/if}

        <!-- Toast Notifications -->
        <Toast messages={toastMessages} onDismiss={dismissToast} />
    </GameWorld>
{/if}

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
</style>
