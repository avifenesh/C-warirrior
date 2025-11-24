<script lang="ts">
    import { createEventDispatcher } from 'svelte';

    interface Props {
        initialCode?: string;
        placeholder?: string;
        submitting?: boolean;
        hint?: string;
        output?: {
            success?: boolean;
            stdout?: string;
            stderr?: string;
            compile_error?: string;
            message?: string;
        } | null;
        onClose?: () => void;
    }

    let {
        initialCode = '',
        placeholder = '// Cast your spell...',
        submitting = false,
        hint = '',
        output = null,
        onClose,
    }: Props = $props();

    const dispatcher = createEventDispatcher();
    let code = $state(initialCode);

    // Local reactive copies of props for proper Svelte 5 reactivity
    let isSubmitting = $state(submitting);
    let currentOutput = $state(output);

    $effect(() => {
        // Keep local state in sync with props
        code = initialCode;
        isSubmitting = submitting;
        currentOutput = output;
        console.log('[CodeTerminal] isSubmitting:', isSubmitting, 'currentOutput:', currentOutput);
    });

    // Lightweight syntax highlighting using token-based approach
    // to avoid number pattern matching inside CSS class names
    function escapeHtml(str: string) {
        return str.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
    }

    function highlight(text: string): string {
        // Token types with their patterns and CSS classes
        const tokenRules: Array<{ pattern: RegExp; className: string }> = [
            { pattern: /\/\/.*$/gm, className: 'hl-comment' },
            { pattern: /"(?:[^"\\]|\\.)*"/g, className: 'hl-string' },
            { pattern: /\b(int|float|double|char|void|return|if|else|for|while|do|switch|case|break|continue|struct|typedef|const|printf|scanf|malloc|free|sizeof|include|define)\b/g, className: 'hl-keyword' },
            { pattern: /\b(0x[\da-fA-F]+|\d+(?:\.\d+)?)\b/g, className: 'hl-number' },
        ];

        // Collect all tokens with their positions
        const tokens: Array<{ start: number; end: number; className: string }> = [];

        for (const rule of tokenRules) {
            rule.pattern.lastIndex = 0;
            let match;
            while ((match = rule.pattern.exec(text)) !== null) {
                const start = match.index;
                const end = start + match[0].length;
                // Only add if not overlapping with existing tokens
                const overlaps = tokens.some(t => !(end <= t.start || start >= t.end));
                if (!overlaps) {
                    tokens.push({ start, end, className: rule.className });
                }
            }
        }

        // Sort tokens by position
        tokens.sort((a, b) => a.start - b.start);

        // Build output with spans
        let result = '';
        let lastEnd = 0;
        for (const token of tokens) {
            result += escapeHtml(text.slice(lastEnd, token.start));
            result += `<span class="${token.className}">${escapeHtml(text.slice(token.start, token.end))}</span>`;
            lastEnd = token.end;
        }
        result += escapeHtml(text.slice(lastEnd));

        return result;
    }

    const highlighted = $derived(highlight(code));
    const isSuccess = $derived(currentOutput?.success === true);
    const isFailure = $derived(currentOutput?.success === false);

    $effect(() => {
        if (isSuccess) {
            dispatcher('sound', { name: 'door_open' });
        }
    });

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
        class={`relative w-full max-w-4xl rounded-2xl border-2 bg-slate-900/95 shadow-2xl backdrop-blur-md transition-all duration-300 ${
            isSuccess
                ? 'border-emerald-400/70 shadow-emerald-400/40 ring-2 ring-emerald-300/30'
                : isFailure
                ? 'border-rose-500/60 shadow-rose-500/30 failure-flash'
                : 'border-cyan-500/50 shadow-cyan-500/20'
        } ${isSubmitting ? 'animate-pulse' : ''}`}
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
            {#if hint}
                <div class="mb-3 rounded-lg border border-amber-500/40 bg-amber-500/10 px-3 py-2 text-xs text-amber-100 shadow-inner shadow-amber-500/20">
                    Hint: {hint}
                </div>
            {/if}

            <div class="relative rounded-xl border-2 border-slate-800 bg-slate-950 p-4 shadow-inner overflow-hidden">
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

                {#if isSubmitting}
                    <div class="absolute inset-0 flex items-center justify-center bg-slate-950/70">
                        <div class="h-10 w-10 animate-spin rounded-full border-4 border-cyan-400/30 border-t-cyan-300"></div>
                    </div>
                {/if}

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
                    disabled={isSubmitting}
                    class="relative rounded-lg border-2 border-cyan-500/70 bg-gradient-to-r from-cyan-600/30 to-cyan-400/30 px-6 py-2 font-semibold text-cyan-50 transition-all hover:-translate-y-0.5 hover:from-cyan-500/40 hover:to-cyan-300/30 disabled:opacity-50 disabled:cursor-not-allowed shadow-lg shadow-cyan-500/30"
                >
                    {isSubmitting ? 'Casting Spell...' : 'Compile & Run'}
                    {#if isSubmitting}
                        <span class="ml-2 inline-block h-4 w-4 animate-spin rounded-full border-2 border-cyan-200/60 border-t-transparent align-middle"></span>
                    {/if}
                </button>
            </div>

            <!-- Output Panel -->
            <div class="mt-4 rounded-xl border border-slate-800 bg-slate-950/70 p-4 shadow-inner shadow-slate-900/60">
                <div class="flex items-center justify-between text-xs uppercase tracking-[0.14em] text-slate-400">
                    <span>Output</span>
                    {#if isSuccess}
                        <span class="text-emerald-300">Spell Cast Successfully!</span>
                    {:else if isFailure}
                        <span class="text-rose-300">Spell fizzled</span>
                    {/if}
                </div>

                {#if output}
                    {#if output.compile_error}
                        <div class="mt-2 rounded-lg border border-rose-500/40 bg-rose-500/10 p-3 text-xs font-mono text-rose-100 whitespace-pre-wrap">
                            {output.compile_error}
                        </div>
                    {/if}
                    {#if output.stdout}
                        <div class="mt-2 rounded-lg border border-slate-700 bg-slate-800/60 p-3 text-xs font-mono text-slate-100 whitespace-pre-wrap">
                            {output.stdout}
                        </div>
                    {/if}
                    {#if output.stderr}
                        <div class="mt-2 rounded-lg border border-amber-500/30 bg-amber-500/10 p-3 text-xs font-mono text-amber-100 whitespace-pre-wrap">
                            {output.stderr}
                        </div>
                    {/if}
                    {#if output.message}
                        <p class="mt-2 text-xs text-slate-400">{output.message}</p>
                    {/if}
                    {#if isFailure && hint}
                        <p class="mt-2 text-xs text-amber-200">Hint: {hint}</p>
                    {/if}
                {:else}
                    <p class="mt-2 text-xs text-slate-500">No output yet.</p>
                {/if}
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

    @keyframes flash-fail {
        0% {
            box-shadow: 0 0 0 0 rgba(244, 63, 94, 0.45);
        }
        50% {
            box-shadow: 0 0 32px 8px rgba(244, 63, 94, 0.35);
        }
        100% {
            box-shadow: 0 0 0 0 rgba(244, 63, 94, 0.45);
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

    .failure-flash {
        animation: flash-fail 0.8s ease-out;
    }

    /* Syntax highlighting classes */
    :global(.hl-keyword) {
        color: #67e8f9; /* cyan-300 */
    }
    :global(.hl-number) {
        color: #fcd34d; /* amber-300 */
    }
    :global(.hl-string) {
        color: #6ee7b7; /* emerald-300 */
    }
    :global(.hl-comment) {
        color: #64748b; /* slate-500 */
    }
</style>
