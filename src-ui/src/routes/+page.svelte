<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import { get } from 'svelte/store';
    import {
        gameStore,
        type Direction,
        type CodeResult,
    } from '$lib/stores/game.svelte';
    import GameWorld from '$lib/components/GameWorld.svelte';
    import CodeTerminal from '$lib/components/CodeTerminal.svelte';
    import GameHUD from '$lib/components/GameHUD.svelte';
    import Toast, { type ToastMessage } from '$lib/components/Toast.svelte';

    const game = gameStore;

    let codeDraft = $state('// Write your C spell here...\n#include <stdio.h>\n\nint main() {\n    printf("Hello, World!\\n");\n    return 0;\n}');
    let toastMessages = $state<ToastMessage[]>([]);
    let toastCounter = 0; // Unique counter for toast IDs

    // Subscribe to stores
    const { renderState, currentLevelId, codeSubmitting, lastCodeResult } = game;

    // Computed: should show terminal?
    let showTerminal = $derived($renderState?.show_terminal ?? false);

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

    function dismissToast(id: string) {
        toastMessages = toastMessages.filter((t) => t.id !== id);
    }
</script>

<svelte:head>
    <title>Code Warrior: C Mastery</title>
</svelte:head>

<GameWorld renderState={$renderState} on:move={handleMove} on:interact={handleInteract}>
    <!-- HUD Overlay -->
    <GameHUD player={$renderState?.player ?? null} currentLevelId={$currentLevelId} />

    <!-- Code Terminal Modal -->
    {#if showTerminal}
        <CodeTerminal
            initialCode={codeDraft}
            submitting={$codeSubmitting}
            output={$lastCodeResult ? {
                success: $lastCodeResult.success,
                stdout: $lastCodeResult.stdout,
                stderr: $lastCodeResult.stderr,
                compile_error: $lastCodeResult.compile_error ?? undefined,
                message: $lastCodeResult.feedback
            } : null}
            onClose={handleTerminalClose}
            on:submit={handleCodeSubmit}
        />
    {/if}

    <!-- Toast Notifications -->
    <Toast messages={toastMessages} onDismiss={dismissToast} />
</GameWorld>
