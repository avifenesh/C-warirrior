<script lang="ts">
    import { createEventDispatcher } from 'svelte';

    interface Props {
        initialCode?: string;
        placeholder?: string;
        submitting?: boolean;
        onClose?: () => void;
    }

    let {
        initialCode = '',
        placeholder = '// Cast your spell...',
        submitting = false,
        onClose,
    }: Props = $props();

    const dispatcher = createEventDispatcher();
    let code = $state(initialCode);

    $effect(() => {
        // Keep local state in sync if parent updates initialCode
        code = initialCode;
    });

    // Lightweight syntax highlighting
    const keywordPattern =
        /\b(int|float|double|char|void|return|if|else|for|while|do|switch|case|break|continue|struct|typedef|const|printf|scanf|malloc|free|sizeof|include|define)\b/g;
    const numberPattern = /\b(0x[\da-fA-F]+|\d+(?:\.\d+)?)\b/g;
    const stringPattern = /"(?:[^"\\]|\\.)*"/g;
    const commentPattern = /\/\/.*$/gm;

    function escapeHtml(str: string) {
        return str.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
    }

    function highlight(text: string) {
        let out = escapeHtml(text);
        out = out.replace(commentPattern, '<span class="text-slate-500">$&</span>');
        out = out.replace(stringPattern, '<span class="text-emerald-300">$&</span>');
        out = out.replace(keywordPattern, '<span class="text-cyan-300">$1</span>');
        out = out.replace(numberPattern, '<span class="text-amber-300">$1</span>');
        return out;
    }

    const highlighted = $derived(highlight(code));

    function submit() {
        dispatcher('submit', { code });
    }

    function handleClose() {
        if (onClose) {
            onClose();
        }
    }

    function handleKeydown(event: KeyboardEvent) {
        // Escape to close
        if (event.key === 'Escape' && onClose) {
            event.preventDefault();
            handleClose();
        }
        // Ctrl/Cmd + Enter to submit
        if ((event.ctrlKey || event.metaKey) && event.key === 'Enter') {
            event.preventDefault();
            submit();
        }
    }
</script>

<svelte:window on:keydown={handleKeydown} />

<!-- Modal Overlay -->
<div class="fixed inset-0 z-50 flex items-center justify-center bg-slate-950/80 backdrop-blur-sm">
    <!-- Terminal Window -->
    <div
        class="relative w-full max-w-4xl rounded-2xl border-2 border-cyan-500/50 bg-slate-900/95 shadow-2xl shadow-cyan-500/20 backdrop-blur-md"
    >
        <!-- Terminal Header -->
        <div
            class="flex items-center justify-between border-b border-cyan-500/30 bg-slate-950/50 px-6 py-4"
        >
            <div class="flex items-center gap-3">
                <div class="flex gap-2">
                    <div class="h-3 w-3 rounded-full bg-rose-500/50"></div>
                    <div class="h-3 w-3 rounded-full bg-amber-500/50"></div>
                    <div class="h-3 w-3 rounded-full bg-emerald-500/50"></div>
                </div>
                <h2 class="font-mono text-sm uppercase tracking-widest text-cyan-400">
                    // Code Terminal
                </h2>
            </div>
            <button
                onclick={handleClose}
                class="text-slate-400 hover:text-slate-200 transition-colors"
                aria-label="Close terminal"
            >
                <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M6 18L18 6M6 6l12 12"
                    />
                </svg>
            </button>
        </div>

        <!-- Terminal Body -->
        <div class="p-6">
            <div class="relative rounded-xl border-2 border-slate-800 bg-slate-950 p-4 shadow-inner">
                <!-- Syntax highlighted overlay -->
                <pre
                    class="pointer-events-none absolute inset-4 m-0 overflow-auto whitespace-pre-wrap font-mono text-sm leading-relaxed text-slate-200"
                    aria-hidden="true"
                ><code>{@html highlighted}</code></pre>

                <!-- Actual editable textarea -->
                <textarea
                    class="relative h-96 w-full resize-none bg-transparent font-mono text-sm text-transparent caret-cyan-300 outline-none"
                    spellcheck="false"
                    bind:value={code}
                    {placeholder}
                    autofocus
                ></textarea>

                <!-- Scan lines effect -->
                <div class="absolute inset-0 pointer-events-none overflow-hidden rounded-xl">
                    <div class="terminal-scan-line"></div>
                </div>
            </div>

            <!-- Terminal Footer -->
            <div class="mt-4 flex items-center justify-between">
                <p class="text-xs font-mono text-slate-500">
                    <kbd class="px-2 py-1 rounded bg-slate-800 text-cyan-400">ESC</kbd>
                    to close |
                    <kbd class="px-2 py-1 rounded bg-slate-800 text-cyan-400">Ctrl+Enter</kbd>
                    to cast spell
                </p>
                <button
                    onclick={submit}
                    disabled={submitting}
                    class="rounded-lg border-2 border-cyan-500/50 bg-cyan-500/10 px-6 py-2 font-semibold text-cyan-300 transition-all hover:border-cyan-400 hover:bg-cyan-500/20 hover:text-cyan-200 disabled:opacity-50 disabled:cursor-not-allowed shadow-lg shadow-cyan-500/20"
                >
                    {submitting ? 'Casting Spell...' : '⚡ Cast Spell'}
                </button>
            </div>
        </div>

        <!-- Glowing border effect -->
        <div class="absolute inset-0 pointer-events-none rounded-2xl">
            <div class="glow-pulse"></div>
        </div>
    </div>
</div>

<style>
    @keyframes terminal-scan {
        from {
            transform: translateY(-100%);
        }
        to {
            transform: translateY(200%);
        }
    }

    @keyframes glow {
        0%,
        100% {
            opacity: 0.5;
        }
        50% {
            opacity: 1;
        }
    }

    .terminal-scan-line {
        position: absolute;
        width: 100%;
        height: 100px;
        background: linear-gradient(
            to bottom,
            transparent,
            rgba(34, 211, 238, 0.05),
            transparent
        );
        animation: terminal-scan 3s linear infinite;
    }

    .glow-pulse {
        position: absolute;
        inset: -2px;
        border-radius: inherit;
        background: linear-gradient(
            135deg,
            rgba(34, 211, 238, 0.2),
            rgba(34, 211, 238, 0),
            rgba(34, 211, 238, 0.2)
        );
        animation: glow 2s ease-in-out infinite;
        pointer-events: none;
    }

    textarea::placeholder {
        color: #64748b;
    }
</style>
